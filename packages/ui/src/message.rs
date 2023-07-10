use miyoo_mini_hal::model::Model;
use system::{Init, SystemMessage};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    System(SystemMessage),
    StartupDone(Init),
}
