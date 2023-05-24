use std::{collections::HashMap, convert::Infallible};

use framebuffer::Framebuffer;
use smithay::{
    backend::renderer::{ImportAll, ImportDmaWl, ImportMemWl, Renderer, Texture, ImportDma, ImportMem},
    utils::{Physical, Size},
};

const RENDERER_ID: usize = 1113;
const SUPPORTED_TEXTURE_FORMATS: [smithay::backend::renderer::; 1] =
    [smithay::reexports::wayland_server::protocol::wl_shm::Format::Abgr8888];

#[derive(Debug)]
pub struct Backend {
    fb: Framebuffer,
    current_frame: Vec<u8>,
    textures: HashMap<Texture>,
}

#[derive(Debug)]
pub struct Frame<'frame> {
    buf: &'frame [u8],
    x_res: u32,
}

pub struct Texture {
    h: u32,
    w: u32,
}

impl Renderer for Backend {
    type Error = Infallible;
    type Frame<'frame> = Frame<'frame>;
    type TextureId = Texture;

    fn render(
        &mut self,
        output_size: smithay::utils::Size<i32, smithay::utils::Physical>,
        dst_transform: smithay::utils::Transform,
    ) -> Result<Self::Frame<'_>, Self::Error> {
        let frame = Frame {
            buf: &self.current_frame,
            x_res: self.fb.var_screen_info.xres,
        };
        Ok(frame)
    }

    fn id(&self) -> usize {
        RENDERER_ID
    }

    fn debug_flags(&self) -> smithay::backend::renderer::DebugFlags {
        smithay::backend::renderer::DebugFlags::empty()
    }

    fn downscale_filter(
        &mut self,
        filter: smithay::backend::renderer::TextureFilter,
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn set_debug_flags(&mut self, flags: smithay::backend::renderer::DebugFlags) {
        unimplemented!()
    }

    fn upscale_filter(
        &mut self,
        filter: smithay::backend::renderer::TextureFilter,
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

impl smithay::backend::renderer::Texture for Texture {
    fn format(&self) -> Option<smithay::reexports::gbm::Format> {
        Some(smithay::reexports::gbm::Format::Abgr8888)
    }

    fn height(&self) -> u32 {
        self.h
    }

    fn width(&self) -> u32 {
        self.w
    }
}

impl smithay::backend::renderer::Frame for Frame {
    type Error = Infallible;
    type TextureId = Texture;

    fn id(&self) -> usize {
        RENDERER_ID
    }

    fn finish(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn clear(
        &mut self,
        color: [f32; 4],
        at: &[smithay::utils::Rectangle<i32, smithay::utils::Physical>],
    ) -> Result<(), Self::Error> {
        for rect in at {
            self.draw_rect(color, rect);
        }

        Ok(())
    }

    fn draw_solid(
        &mut self,
        dst: smithay::utils::Rectangle<i32, smithay::utils::Physical>,
        damage: &[smithay::utils::Rectangle<i32, smithay::utils::Physical>],
        color: [f32; 4],
    ) -> Result<(), Self::Error> {
        // what is `damage` for???

        self.draw_rect(&color, rect);
        Ok(())
    }

    fn render_texture_at(
        &mut self,
        texture: &Self::TextureId,
        pos: smithay::utils::Point<i32, smithay::utils::Physical>,
        texture_scale: i32,
        output_scale: impl Into<smithay::utils::Scale<f64>>,
        src_transform: smithay::utils::Transform,
        damage: &[smithay::utils::Rectangle<i32, smithay::utils::Physical>],
        alpha: f32,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn render_texture_from_to(
        &mut self,
        texture: &Self::TextureId,
        src: smithay::utils::Rectangle<f64, smithay::utils::Buffer>,
        dst: smithay::utils::Rectangle<i32, smithay::utils::Physical>,
        damage: &[smithay::utils::Rectangle<i32, smithay::utils::Physical>],
        src_transform: smithay::utils::Transform,
        alpha: f32,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn transformation(&self) -> smithay::utils::Transform {
        todo!()
    }
}

impl Backend {
    pub fn new() -> (Self, Size<i32, Physical>) {
        let fb = Framebuffer::new("/dev/fb0").unwrap();
        (
            Self {
                current_frame: Vec::with_capacity(
                    4 * fb.var_screen_info.xres * fb.var_screen_info.yres,
                ),
                fb,
                textures: HashMap::new(),
            },
            (
                fb.var_screen_info.xres as i32,
                fb.var_screen_info.yres as i32,
            )
                .into(),
        )
    }

    pub fn commit(&mut self) {
        self.fb.write_frame(&self.current_frame)
    }
}

impl Frame {
    #[inline]
    pub fn draw_rect<K>(&mut self, color: [f32; 4], rect: smithay::utils::Rectangle<i32, K>) {
        for y in (rect.loc.y)..(rect.size.h) {
            for x in (rect.loc.x)..(rect.size.w) {
                self.write_coord(&color, x, y);
            }
        }
    }

    #[inline(always)]
    pub fn write_coord(&mut self, color: &[f32; 4], x: i32, y: i32) {
        // y * x_res => go to correct row
        // x => go to correct column
        // * 4 => account for 4 bytes per pixel
        for (i, color) in color.iter().enumerate() {
            self.buf[((y * self.x_res) + x) * 4 + i] = color as u8;
        }
    }
}

impl ImportMem for Backend {
    fn import_memory(
			&mut self,
			data: &[u8],
			format: smithay::reexports::gbm::Format,
			size: Size<i32, smithay::utils::Buffer>,
			flipped: bool,
		) -> Result<<Self as Renderer>::TextureId, <Self as Renderer>::Error> {
		
	}

    fn shm_formats(
        &self,
    ) -> Box<dyn Iterator<Item = smithay::reexports::wayland_server::protocol::wl_shm::Format>>
    {
		Box::new(&SUPPORTED_TEXTURE_FORMATS)
    }
}

impl ImportDma for Backend {
    fn dmabuf_formats(&self) -> Box<dyn Iterator<Item = smithay::backend::allocator::Format>> {
		
	}

	fn import_dmabuf(
			&mut self,
			dmabuf: &smithay::backend::allocator::dmabuf::Dmabuf,
			damage: Option<&[smithay::utils::Rectangle<i32, smithay::utils::Buffer>]>,
		) -> Result<<Self as Renderer>::TextureId, <Self as Renderer>::Error> {
		
	}
}
