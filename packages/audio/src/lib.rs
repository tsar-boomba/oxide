pub fn init() {
    tracing::info!("audio hosts: {:?}", cpal::available_hosts());
}
