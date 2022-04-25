use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorJson {
    error: String
}

impl ErrorJson {
    pub fn new(error: String) -> Self {
        ErrorJson { error }
    }
}