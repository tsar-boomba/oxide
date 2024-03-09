use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Function {
    type ReqBody: Serialize + DeserializeOwned;

    type ResBody: Serialize + DeserializeOwned;

    fn path() -> &'static str;
}

pub struct SaveState;

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveStateArgs {
    pub slot: Option<usize>,
}

impl Function for SaveState {
    type ReqBody = SaveStateArgs;
    type ResBody = ();

    fn path() -> &'static str {
        "/save-state"
    }
}

pub struct Stop;

#[derive(Debug, Serialize, Deserialize)]
pub struct StopArgs {}

impl Function for Stop {
    type ReqBody = StopArgs;
    type ResBody = ();

    fn path() -> &'static str {
        "/stop"
    }
}

pub struct Start;

#[derive(Debug, Serialize, Deserialize)]
pub struct StartArgs {}

impl Function for Start {
    type ReqBody = StartArgs;
    type ResBody = ();

    fn path() -> &'static str {
        "/start"
    }
}
