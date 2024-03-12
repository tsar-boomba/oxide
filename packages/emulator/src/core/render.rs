use std::{
    ffi::c_void,
    num::NonZeroU32,
    path::Path,
    sync::{atomic::AtomicUsize, Arc},
};

use fast_image_resize::{CropBox, PixelType, Resizer};
use libretro_sys::PixelFormat;
use winit::window::Window;

use crate::{convert, core::av_info};

use super::{bytes_per_pixel, pixel_format};

struct Image {
    inner: fast_image_resize::Image<'static>,
    pub height: NonZeroU32,
    pub width: NonZeroU32,
}

impl Image {
    pub fn save(&self, path: impl AsRef<Path>) {
        let mut encoder = png::Encoder::new(
            std::io::BufWriter::new(
                std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)
                    .unwrap(),
            ),
            self.inner.width().get(),
            self.inner.height().get(),
        );
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        tracing::debug!(
            "w: {}; h: {}; buf len: {}",
            self.inner.width(),
            self.inner.height(),
            self.inner.buffer().len()
        );

        let mut writer = encoder.write_header().unwrap();
        writer
            .write_image_data(&convert::xrgb8888_to_rgba888(unsafe {
                std::slice::from_raw_parts(
                    self.inner.buffer().as_ptr().cast(),
                    self.inner.buffer().len() / 4,
                )
            }))
            .unwrap();
    }

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        self.inner.buffer_mut()
    }

    /// View of frame with any padding cropped out
    pub fn view(&self) -> fast_image_resize::DynamicImageView<'_> {
        let mut view = self.inner.view();
        view.set_crop_box(CropBox {
            top: 0.,
            left: 0.,
            height: self.height.get() as f64,
            width: self.width.get() as f64,
        })
        .unwrap();

        view
    }
}

/// Hold frame from core that has been converted to rgb8
static mut RAW_FRAME_BUFFER: Option<Image> = None;
static mut SKIPPED: bool = false;
static mut RESIZER: Option<Resizer> = None;
static mut CROP: Option<CropBox> = None;

/// Handle frame directly from core
pub unsafe extern "C" fn handle_raw_frame(
    raw_pixels: *const c_void,
    width: u32,
    height: u32,
    pitch: usize,
) {
    // SAFETY: This static will only be accessed from this module which will only be used on the main thread

    let image = if let Some(image) = unsafe { RAW_FRAME_BUFFER.as_mut() } {
        image
    } else {
        // Initialize the frame buffer
        RESIZER = Some(Resizer::new(fast_image_resize::ResizeAlg::Nearest));

        // Do not init on a skipped frame
        if raw_pixels.is_null() {
            unsafe { SKIPPED = true };
            return;
        }

        tracing::debug!("w: {width}; h: {height}; pitch: {pitch}");

        // This must be first frame, initialize RAW_FRAME_BUFFER
        let height = NonZeroU32::new(height).unwrap();
        let av_info = av_info();
        let inner = fast_image_resize::Image::from_vec_u8(
            (pitch as u32 / bytes_per_pixel() as u32)
                .try_into()
                .unwrap(),
            height,
            // Length is (pixels.len / bpp) * 4 so that is pixels is 16bit it will be right size
            vec![0u8; ((height.get() as usize * pitch) / bytes_per_pixel() as usize) * 4],
            fast_image_resize::PixelType::U8x4,
        )
        .unwrap();

        let crop = if av_info.geometry.aspect_ratio < (4.0 / 3.0) {
            // Console is tall
            let new_height = 480.0;
            // use aspect ratio to get the proper width knowing the height
            let new_width = new_height * (1.0 / av_info.geometry.aspect_ratio as f64);
            let leftover_width = 640.0 - new_width;
            let left = leftover_width / 2.0;

            CropBox {
                height: new_height.try_into().unwrap(),
                width: (new_width as u32).try_into().unwrap(),
                left,
                top: 0.,
            }
        } else {
            // Console is wide
            let new_width = 640.0;
            // use aspect ratio to get the proper height knowing the width
            let new_height = new_width * (1.0 / av_info.geometry.aspect_ratio as f64);
            let leftover_height = 480.0 - new_height;
            let top = leftover_height / 2.0;

            CropBox {
                height: (new_height as u32).try_into().unwrap(),
                width: new_width.try_into().unwrap(),
                left: 0.,
                top,
            }
        };

        tracing::debug!("Working with crop: {crop:?}");

        unsafe { CROP = Some(crop) };

        RAW_FRAME_BUFFER = Some(Image {
            inner,
            height,
            width: av_info.geometry.base_width.try_into().unwrap(),
        });

        // SAFETY: We just set it and these functions are guaranteed to only be called from this thread
        unsafe { RAW_FRAME_BUFFER.as_mut().unwrap_unchecked() }
    };

    if !raw_pixels.is_null() {
        let pixels: &[u8] = std::slice::from_raw_parts(raw_pixels.cast(), height as usize * pitch);

        tracing::debug!("[pre-convert] w: {width}; h: {height}; p: {pitch}");
        match pixel_format() {
            PixelFormat::ARGB1555 => todo!(),
            PixelFormat::RGB565 => convert::rgb565_to_xrgb8888(pixels, image.buffer_mut()),
            PixelFormat::ARGB8888 => convert::argb8888_to_xrgb8888(pixels, image.buffer_mut()),
        }

        SKIPPED = false;
    } else {
        SKIPPED = true;
    }
}

static RENDERED: AtomicUsize = AtomicUsize::new(0);

#[inline]
pub fn render(mut buffer: softbuffer::Buffer<'_, &Window, &Window>) {
    if unsafe { SKIPPED } {
        // If frame was skipped, buffer may not be initialized plus there's no reason to render
        return;
    }

    // SAFETY: RAW_FRAME_BUFFER is never set to None besides on initialization
    let frame = unsafe { RAW_FRAME_BUFFER.as_ref().unwrap_unchecked() };

    // Allow window buffer to be written in terms of bytes
    let u8_buffer = unsafe {
        std::slice::from_raw_parts_mut::<'_, u8>(buffer.as_mut_ptr().cast(), buffer.len() * 4)
    };

    let mut buffer_img = fast_image_resize::Image::from_slice_u8(
        640.try_into().unwrap(),
        480.try_into().unwrap(),
        u8_buffer,
        PixelType::U8x4,
    )
    .unwrap();

    // SAFETY: This function is always called after handle_raw_frame where RESIZER is initialized
    let resizer = unsafe { RESIZER.as_mut().unwrap_unchecked() };

    let CropBox {
        left,
        top,
        width,
        height,
    } = unsafe { CROP.as_ref().copied().unwrap() };
    let mut dst = buffer_img
        .view_mut()
        .crop(
            left as u32,
            top as u32,
            (width as u32).try_into().unwrap(),
            (height as u32).try_into().unwrap(),
        )
        .unwrap();

    resizer.resize(&frame.view(), &mut dst).unwrap();

    // Store a copy of frame for screenshots
    super::CURRENT_FRAME.store(Some(Arc::new(buffer.to_vec())));
    buffer.present().unwrap();
    RENDERED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}
