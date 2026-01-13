use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DSResponse<T> {
    pub data: Option<T>,
    pub err : Option<&'static str>
}

impl<T> DSResponse<T> {
    pub fn new (data: Option<T>, err : Option<&'static str> ) -> Self {
        Self {data, err}
    }
}