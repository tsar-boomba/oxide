#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .compact()
        .with_ansi(false)
        .with_writer(
            std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open("/mnt/SDCARD/miyoo/app/os.log")
                .unwrap(),
        )
        .init();

    std::env::set_var("RUST_BACKTRACE", "full");
    std::env::set_var("WINIT_UNIX_BACKEND", "wayland");
    std::env::set_var("HOME", "/mnt/SDCARD/miyoo/app");

    ui::start();
}
