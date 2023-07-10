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
