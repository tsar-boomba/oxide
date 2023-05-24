use system::SystemMessage;

#[derive(Debug, Clone)]
pub enum Message {
	System(SystemMessage),
	Noop
}
