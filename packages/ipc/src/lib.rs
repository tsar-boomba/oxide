#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "server")]
pub use axum::{extract, http::*, routing, Router};
#[cfg(feature = "server")]
pub use hyper::Error;

#[cfg(feature = "client")]
pub mod client;
pub mod functions;

pub const SOCKET_PATH: &str = "/tmp/ipc.sock";
