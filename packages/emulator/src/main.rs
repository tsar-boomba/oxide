#![feature(c_variadic)]

mod backend;
pub mod convert;
pub mod core;
mod ipc;

use arc_swap::ArcSwapOption;
use backend::BackendMessage;
use bpaf::Bpaf;
use crossbeam::channel::TryRecvError;
use fixed_map::{Key, Map};
use input::Button;
use once_cell::sync::OnceCell;
use tokio::sync::mpsc;
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{WindowBuilder, WindowLevel},
};

use crate::core::{save::load, Frame};
use std::{cell::RefCell, path::PathBuf, sync::Arc, thread, time::Duration};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
static ARGS: OnceCell<Args> = OnceCell::new();
static BACKEND_SENDER: OnceCell<mpsc::Sender<BackendMessage>> = OnceCell::new();

#[derive(Debug, Bpaf)]
#[bpaf(options)]
struct Args {
    #[bpaf(short, long, flag(true, false))]
    /// Use an auto save if it exists, essentially resume
    pub load_auto: bool,
    #[bpaf(positional)]
    /// Path to the core to use
    pub core_path: PathBuf,
    #[bpaf(positional)]
    /// Path to the game to run
    pub game_path: PathBuf,
}

impl Args {
    pub fn core_name(&self) -> &str {
        // Core paths should be in form {name}_libretro.so
        self.core_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .trim_end_matches("_libretro.so")
    }

    pub fn game_name(&self) -> &str {
        self.game_path.file_stem().unwrap().to_str().unwrap()
    }

    pub fn sys_dir(&self) -> String {
        format!("/mnt/SDCARD/Saves/{}", self.core_name())
    }

    pub fn save_dir(&self) -> String {
        format!("{}/saves", self.sys_dir())
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .compact()
        .with_env_filter("emulator=debug")
        .with_writer(
            std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open("/mnt/SDCARD/miyoo/app/emu_log.log")
                .unwrap(),
        )
        .init();

    let args = args().run();
    tracing::debug!("{args:#?}");
    ARGS.set(args).unwrap();

    // Init tokio backend to get input and handle audio
    let (backend_sender, mut input_recv) = backend::start();
    BACKEND_SENDER.set(backend_sender).unwrap();

    let mut window_width = 640u32;
    let mut window_height = 480u32;
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_position(LogicalPosition::new(0, 0))
        .with_inner_size(LogicalSize::new(window_width, window_height))
        .with_window_level(WindowLevel::AlwaysOnTop)
        .with_decorations(cfg!(debug_assertions))
        .with_visible(true)
        .with_resizable(false)
        .with_transparent(false)
        .build(&event_loop)
        .unwrap();
    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();
    surface
        .resize(
            window_width.try_into().unwrap(),
            window_height.try_into().unwrap(),
        )
        .unwrap();

    // Present so that window shows up (wayland kinda cringe for this)
    surface.buffer_mut().unwrap().present().unwrap();

    let frame_recv = core::init(ARGS.get().unwrap().core_path.as_os_str());

    let game_path = &ARGS.get().unwrap().game_path;
    if !core::load_game(game_path).unwrap() {
        panic!("Failed to load game from {}", game_path.display());
    };

    // Can be called after load_game
    core::av_info();

    if ARGS.get().unwrap().load_auto {
        if let Err(err) = load(None) {
            // It is valid for a auto file to not be found
            if err.kind() != std::io::ErrorKind::NotFound {
                panic!("Error loading auto save: {err:?}");
            }
        }
    }

    let mut input_state: Map<Button, bool> = Map::new();

    tracing::debug!("Starting event loop! :D");
    event_loop.run(move |event, _, _| {
        match event {
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::CloseRequested => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        };

        // Consume all inputs in channel
        loop {
            match input_recv.try_recv() {
                Ok(button_ev) => {
                    if button_ev.pressed() {
                        // Add to map, indicating it is pressed
                        tracing::debug!("{:?} Pressed", button_ev.button());
                        input_state.insert(*button_ev.button(), true);
                    } else {
                        // Remove from map, indicating release
                        tracing::debug!("{:?} Released", button_ev.button());
                        input_state.remove(*button_ev.button());
                    };
                }
                Err(mpsc::error::TryRecvError::Empty) => break,
                _ => panic!("Input sender dropped!"),
            }
        }

        core::run(surface.buffer_mut().unwrap(), input_state.clone());
    });
}
