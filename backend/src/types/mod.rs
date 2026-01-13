use serde::{Deserialize, Serialize};
pub mod message_req_types;

#[derive(Serialize, Deserialize)]
pub struct DSResponse<T> {
    pub data: Option<T>,
    pub err: Option<&'static str>,
}

impl<T> DSResponse<T> {
    pub fn new(data: Option<T>, err: Option<&'static str>) -> Self {
        Self { data, err }
    }
}

pub struct DSError<T> {
    pub data: Option<T>,
    pub err: Option<T>,
}

impl<T> DSError<T> {
    pub fn new(data: Option<T>, err: Option<T>) -> Self {
        Self { data, err }
    }
}
