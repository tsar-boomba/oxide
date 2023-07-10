pub mod charger;
pub mod gpio;
pub mod model;
mod pwm;
pub mod rumble;
pub mod screen;

/// Spawn a task on the blocking thread pool
pub(crate) async fn asyncify<F, T>(f: F) -> std::io::Result<T>
where
    F: FnOnce() -> std::io::Result<T> + Send + 'static,
    T: Send + 'static,
{
    match tokio::task::spawn_blocking(f).await {
        Ok(res) => res,
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "background task failed",
        )),
    }
}
