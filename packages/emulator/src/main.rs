#![feature(c_variadic)]

mod backend;
pub mod convert;
pub mod core;
mod ipc;

use backend::BackendMessage;
use bpaf::Bpaf;
use fixed_map::Map;
use input::Button;
use once_cell::sync::OnceCell;
use tokio::sync::mpsc;
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{WindowBuilder, WindowLevel},
};

use crate::core::save;
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

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
                .open(format!(
                    "/mnt/SDCARD/miyoo/app/emu_log_{}.log",
                    std::time::SystemTime::now()
                        .duration_since(std::time::SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                ))
                .unwrap(),
        )
        .init();

    let args = args().run();
    tracing::debug!("{args:#?}");
    ARGS.set(args).unwrap();

    // Init tokio backend to get input and handle audio
    let (backend_sender, mut input_recv) = backend::start();
    BACKEND_SENDER.set(backend_sender).unwrap();

    let window_width = 640u32;
    let window_height = 480u32;
    let event_loop = EventLoop::new().unwrap();
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
    let context = softbuffer::Context::new(&window).unwrap();
    let mut surface = softbuffer::Surface::new(&context, &window).unwrap();
    surface
        .resize(
            window_width.try_into().unwrap(),
            window_height.try_into().unwrap(),
        )
        .unwrap();

    // Present so that window shows up (wayland kinda cringe for this)
    surface.buffer_mut().unwrap().present().unwrap();

    core::init(&ARGS.get().unwrap().core_path);

    let game_path = &ARGS.get().unwrap().game_path;
    if !core::load_game(game_path).unwrap() {
        panic!("Failed to load game from {}", game_path.display());
    };

    // Can be called after load_game
    let av_info = core::av_info();
    let fps = av_info.timing.fps;
    let seconds_per_frame = 1.0 / fps;
    let nanos_per_frame = (seconds_per_frame * 1e+9) as u64;
    // Subtract 300 microseconds to account for time between loops
    let nanos_per_frame = Duration::from_nanos(nanos_per_frame - 300_000);

    // Can be called after av_info is available
    core::audio::init();

    if ARGS.get().unwrap().load_auto {
        if let Err(err) = save::load(None) {
            // It is valid for a auto file to not be found
            if err.kind() != std::io::ErrorKind::NotFound {
                panic!("Error loading auto save: {err:?}");
            }
        }
    }

    let mut input_state: Map<Button, bool> = Map::new();
    let mut saving_receiver = save::init();
    let mut last_loop_end = Instant::now();

    tracing::debug!("Starting event loop! :D");
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    event_loop
        .run(move |event, _| {
            tracing::debug!(
                "Between loops: {}us",
                (Instant::now() - last_loop_end).as_micros()
            );
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        std::process::exit(0);
                    }
                    _ => {}
                },
                _ => {}
            };

            // The sender half can NEVER be dropped so its okay not to handle that case
            if let Ok(ack) = saving_receiver.try_recv() {
                let thread = std::thread::current();
                ack.send(thread).unwrap();

                tracing::debug!("Main thread waiting on save...");
                // Wait for save op to finish and unpark this thread
                std::thread::park();
            }

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

            let frame_start_time = Instant::now();
            core::run(surface.buffer_mut().unwrap(), input_state.clone());

            let render_time = Instant::now() - frame_start_time;

            if nanos_per_frame > render_time {
                // If there is leftover time in the frame sleep with system sleep or spinning (targets 60fps)
                let time_left_till_next_frame = Duration::new(
                    0,
                    nanos_per_frame.as_nanos() as u32 - render_time.as_nanos() as u32,
                );

                let start = Instant::now();
                while start.elapsed() < time_left_till_next_frame {
                    // Use spin loop because we want this to be high performance
                    std::hint::spin_loop();
                }
            }
            last_loop_end = Instant::now();
        })
        .unwrap();
}
