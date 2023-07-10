//! Mostly implemented thanks to https://www.retroreversing.com/CreateALibRetroFrontEndInRust

mod render;
pub mod save;
mod variable;

use std::{
    ffi::*,
    fs::OpenOptions,
    io::{self, BufReader, Read},
    ops::Deref,
    os::unix::prelude::OsStrExt,
    path::Path,
    ptr,
};

use arc_swap::ArcSwapOption;
use crossbeam::channel;
use fixed_map::Map;
use libloading::Library;
use libretro_sys::{CoreAPI, GameInfo, PixelFormat, SystemAvInfo};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;

use crate::{convert, Button, ARGS};

use self::variable::VariableDef;

/// There will only ever be one core loaded per instance of this application
static CORE: OnceCell<Core> = OnceCell::new();
static STATE: OnceCell<Mutex<State>> = OnceCell::new();
/// Vec of XRGB8888 bytes
static CURRENT_FRAME: ArcSwapOption<Vec<u32>> = ArcSwapOption::const_empty();

const EXPECTED_LIB_RETRO_VERSION: u32 = 1;

#[derive(Debug)]
pub struct Core {
    core: libretro_sys::CoreAPI,
    _lib: libloading::Library,
}

#[derive(Debug)]
struct State {
    pixel_format: Option<PixelFormat>,
    input_state: Map<Button, bool>,
    window_width: u32,
    window_height: u32,
    bytes_per_pixel: u8,
}

#[derive(Debug)]
pub struct Frame {
    pub buffer: Vec<u32>,
    pub height: usize,
    pub width: usize,
    pub pitch: usize,
}

/// Loads the library into the `CORE` static and returns a receiver for the frames
pub fn init(path: impl AsRef<OsStr>) -> channel::Receiver<Frame> {
    unsafe {
        let lib = Library::new(path).expect("Failed to load Core");

        let core = CoreAPI {
            retro_set_environment: *(lib.get(b"retro_set_environment").unwrap()),
            retro_set_video_refresh: *(lib.get(b"retro_set_video_refresh").unwrap()),
            retro_set_audio_sample: *(lib.get(b"retro_set_audio_sample").unwrap()),
            retro_set_audio_sample_batch: *(lib.get(b"retro_set_audio_sample_batch").unwrap()),
            retro_set_input_poll: *(lib.get(b"retro_set_input_poll").unwrap()),
            retro_set_input_state: *(lib.get(b"retro_set_input_state").unwrap()),

            retro_init: *(lib.get(b"retro_init").unwrap()),
            retro_deinit: *(lib.get(b"retro_deinit").unwrap()),

            retro_api_version: *(lib.get(b"retro_api_version").unwrap()),

            retro_get_system_info: *(lib.get(b"retro_get_system_info").unwrap()),
            retro_get_system_av_info: *(lib.get(b"retro_get_system_av_info").unwrap()),
            retro_set_controller_port_device: *(lib
                .get(b"retro_set_controller_port_device")
                .unwrap()),

            retro_reset: *(lib.get(b"retro_reset").unwrap()),
            retro_run: *(lib.get(b"retro_run").unwrap()),

            retro_serialize_size: *(lib.get(b"retro_serialize_size").unwrap()),
            retro_serialize: *(lib.get(b"retro_serialize").unwrap()),
            retro_unserialize: *(lib.get(b"retro_unserialize").unwrap()),

            retro_cheat_reset: *(lib.get(b"retro_cheat_reset").unwrap()),
            retro_cheat_set: *(lib.get(b"retro_cheat_set").unwrap()),

            retro_load_game: *(lib.get(b"retro_load_game").unwrap()),
            retro_load_game_special: *(lib.get(b"retro_load_game_special").unwrap()),
            retro_unload_game: *(lib.get(b"retro_unload_game").unwrap()),

            retro_get_region: *(lib.get(b"retro_get_region").unwrap()),
            retro_get_memory_data: *(lib.get(b"retro_get_memory_data").unwrap()),
            retro_get_memory_size: *(lib.get(b"retro_get_memory_size").unwrap()),
        };

        CORE.set(Core { core, _lib: lib }).unwrap();

        // Init with defaults until the core tells us what format to use
        STATE
            .set(Mutex::new(State {
                pixel_format: None,
                input_state: Map::new(),
                window_height: 480,
                window_width: 640,
                bytes_per_pixel: 4,
            }))
            .unwrap();

        let (send, recv) = channel::bounded(32);

        let core = CORE.get().unwrap();
        if (core.retro_api_version)() != EXPECTED_LIB_RETRO_VERSION {
            panic!("The Core has been compiled with a LibRetro API that is unexpected, we expected version to be: {}", EXPECTED_LIB_RETRO_VERSION)
        }
        (core.retro_set_environment)(libretro_environment_callback);
        (core.retro_init)();
        (core.retro_set_video_refresh)(render::handle_raw_frame);
        (core.retro_set_input_poll)(libretro_set_input_poll_callback);
        (core.retro_set_input_state)(libretro_set_input_state_callback);
        (core.retro_set_audio_sample)(libretro_set_audio_sample_callback);
        (core.retro_set_audio_sample_batch)(libretro_set_audio_sample_batch_callback);

        recv
    }
}

pub fn load_game(path: impl AsRef<Path>) -> io::Result<bool> {
    let core = CORE.get().unwrap();
    let path = path.as_ref();
    let file = OpenOptions::new().read(true).open(&path)?;
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let path = CString::new(path.as_os_str().as_bytes()).unwrap();

    let info = Box::new(GameInfo {
        data: buf.as_ptr() as *const c_void,
        path: path.as_ptr(),
        size: buf.len(),
        meta: ptr::null(),
    });
    let loaded_successfully = unsafe { (core.retro_load_game)(&*info as *const GameInfo) };

    // Don't free this memory while core may use it??? (idk if core copies it or nah)
    std::mem::forget(info);
    std::mem::forget(buf);

    Ok(loaded_successfully)
}

#[inline(always)]
pub fn bytes_per_pixel() -> u8 {
    // Cache this so no need to lock state
    static BPP: OnceCell<u8> = OnceCell::new();
    *BPP.get_or_init(|| STATE.get().unwrap().lock().bytes_per_pixel)
}

#[inline(always)]
pub fn pixel_format() -> PixelFormat {
    // Cache this so no need to lock state
    static PIXEL_FORMAT: OnceCell<PixelFormat> = OnceCell::new();
    *PIXEL_FORMAT.get_or_init(|| STATE.get().unwrap().lock().pixel_format.unwrap())
}

#[inline(always)]
pub fn av_info() -> &'static SystemAvInfo {
    static AV_INFO: OnceCell<SystemAvInfo> = OnceCell::new();
    AV_INFO.get_or_init(|| {
        let mut av_info = SystemAvInfo {
            geometry: libretro_sys::GameGeometry {
                base_width: 0,
                base_height: 0,
                max_width: 0,
                max_height: 0,
                aspect_ratio: 0.0,
            },
            timing: libretro_sys::SystemTiming {
                fps: 0.0,
                sample_rate: 0.0,
            },
        };

        unsafe {
            (CORE.get().unwrap().retro_get_system_av_info)(&mut av_info);
        }

        tracing::debug!("AV Info: {av_info:#?}");
        av_info
    })
}

pub fn reset() {
    unsafe { (CORE.get().unwrap().retro_reset)() }
}

#[inline(always)]
pub fn render(buffer: softbuffer::Buffer<'_>) {
    render::render(buffer);
}

pub fn dry_run() {
    unsafe { (CORE.get().unwrap().retro_run)() }
}

/// Runs the emulator once
#[inline(always)]
pub fn run(buffer: softbuffer::Buffer<'_>, input_state: Map<Button, bool>) {
    STATE.get().unwrap().lock().input_state = input_state;
    unsafe { (CORE.get().unwrap().retro_run)() };
    render(buffer);
}

impl Deref for Core {
    type Target = CoreAPI;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

unsafe extern "C" fn libretro_environment_callback(command: u32, data: *mut c_void) -> bool {
    match command {
        libretro_sys::ENVIRONMENT_GET_CAN_DUPE => {
            *(data as *mut bool) = true;
            tracing::debug!("ENVIRONMENT_GET_CAN_DUPE");
        }
        libretro_sys::ENVIRONMENT_GET_SYSTEM_DIRECTORY
        | libretro_sys::ENVIRONMENT_GET_SAVE_DIRECTORY => {
            let dir = CString::new(ARGS.get().unwrap().sys_dir()).unwrap();
            *(data as *mut *const u8) = dir.as_ptr();

            // Don't free this memory while core may use it??? (idk if core copies it or nah)
            std::mem::forget(dir);

            return true;
        }
        libretro_sys::ENVIRONMENT_SET_PIXEL_FORMAT => {
            let mut state = STATE.get().unwrap().lock();
            let pixel_format = *(data as *const u32);
            let pixel_format = PixelFormat::from_uint(pixel_format).unwrap();
            match pixel_format {
                PixelFormat::ARGB1555 | PixelFormat::RGB565 => state.bytes_per_pixel = 2,
                PixelFormat::ARGB8888 => state.bytes_per_pixel = 4,
                _ => panic!("Core is trying to use an Unknown Pixel Format: {pixel_format:?}"),
            };
            tracing::debug!("Starting with format: {pixel_format:?}");
            state.pixel_format = Some(pixel_format);
            return true;
        }
        libretro_sys::ENVIRONMENT_SET_VARIABLES => {
            let var_defs = VariableDef::from_raw_array(data as *const *const u8);
            tracing::debug!("Variables: {var_defs:#?}");
            return true;
        }
        libretro_sys::ENVIRONMENT_GET_VARIABLE => {}
        libretro_sys::ENVIRONMENT_GET_VARIABLE_UPDATE => {
            // Not updating variables right now
            *(data as *mut bool) = false;
            return true;
        }
        libretro_sys::ENVIRONMENT_GET_LOG_INTERFACE => {
            let cb = &mut *(data as *mut libretro_sys::LogCallback);
            // SAFETY: libretro_sys has the wrong type here as it is actually variadic
            cb.log = std::mem::transmute::<_, _>(
                log as unsafe extern "C" fn(libretro_sys::LogLevel, *const u8, ...),
            );
            return true;
        }
        // Get audio video enable
        65583 => {
            // This enables audio & video
            // TODO add a muted mode that will disable audio for more perf
            //*(data as *mut i32) = 0b0011 as i32;
            return false;
        }
        // Set minimum audio latency
        63 => {
            tracing::debug!("Set audio latency");
            return true;
        }
        // Fast-forwarding override
        64 => return true,
        _ => tracing::debug!(
            "libretro_environment_callback Called with command: {}",
            command
        ),
    }
    false
}

unsafe extern "C" fn libretro_set_input_poll_callback() {}

unsafe extern "C" fn libretro_set_input_state_callback(
    port: u32,
    device: u32,
    index: u32,
    id: u32,
) -> i16 {
    let state = STATE.get().unwrap().lock();
    let input = &state.input_state;

    //tracing::debug!("Input state requested");

    match id {
        libretro_sys::DEVICE_ID_JOYPAD_A => input.get(Button::A).is_some() as i16,
        libretro_sys::DEVICE_ID_JOYPAD_B => input.get(Button::B).is_some() as i16,
        libretro_sys::DEVICE_ID_JOYPAD_X => input.get(Button::X).is_some() as i16,
        libretro_sys::DEVICE_ID_JOYPAD_Y => input.get(Button::Y).is_some() as i16,
        libretro_sys::DEVICE_ID_JOYPAD_UP => input.get(Button::Up).is_some() as i16,
        libretro_sys::DEVICE_ID_JOYPAD_DOWN => input.get(Button::Down).is_some() as i16,
        libretro_sys::DEVICE_ID_JOYPAD_LEFT => input.get(Button::Left).is_some() as i16,
        libretro_sys::DEVICE_ID_JOYPAD_RIGHT => input.get(Button::Right).is_some() as i16,
        libretro_sys::DEVICE_ID_JOYPAD_L => input.get(Button::L1).is_some() as i16,
        libretro_sys::DEVICE_ID_JOYPAD_L2 => input.get(Button::L2).is_some() as i16,
        libretro_sys::DEVICE_ID_JOYPAD_R => input.get(Button::R1).is_some() as i16,
        libretro_sys::DEVICE_ID_JOYPAD_R2 => input.get(Button::R2).is_some() as i16,
        libretro_sys::DEVICE_ID_JOYPAD_START => input.get(Button::Start).is_some() as i16,
        libretro_sys::DEVICE_ID_JOYPAD_SELECT => input.get(Button::Select).is_some() as i16,
        _ => 0,
    }
}

unsafe extern "C" fn libretro_set_audio_sample_callback(left: i16, right: i16) {}

unsafe extern "C" fn libretro_set_audio_sample_batch_callback(
    data: *const i16,
    frames: usize,
) -> usize {
    const CHANNELS: usize = 2;
    let audio = std::slice::from_raw_parts(data, frames * CHANNELS).to_vec();
    frames
}

unsafe extern "C" fn log(level: libretro_sys::LogLevel, format_str: *const c_char, mut args: ...) {
    //GBA DMA: Starting DMA 3 0x03007E44 -> 0x02000000 (8500:0000)
    nix::libc::printf(format_str, args);
    match level {
        libretro_sys::LogLevel::Info => tracing::info!("Core"),
        libretro_sys::LogLevel::Debug => tracing::debug!("Core"),
        libretro_sys::LogLevel::Error => tracing::error!("Core"),
        libretro_sys::LogLevel::Warn => tracing::warn!("Core"),
    }
}
