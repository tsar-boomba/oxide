use std::io::{BufWriter, Write};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(Debug, Clone, Copy)]
struct Rgba(u8, u8, u8, u8);

impl Rgba {
    pub fn bytes(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }
}

const X_RES: usize = 640;
const Y_RES: usize = 480;
const FB: [Rgba; X_RES * Y_RES] = [Rgba(255, 255, 255, 0); X_RES * Y_RES];

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .compact()
        .with_ansi(false)
        .init();

    let mut fb = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .open("/dev/fb0")
        .unwrap();
    fb.write_all(&FB.iter().flat_map(Rgba::bytes).collect::<Vec<_>>())
        .unwrap();

    ui::start();
}
