use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorJson {
    error: String,
    code: u16
}

impl ErrorJson {
    pub fn new(error: String, code: u16) -> Self {
        ErrorJson { error, code }
    }
}