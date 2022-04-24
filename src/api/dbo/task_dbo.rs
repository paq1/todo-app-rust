use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TaskDto {
    title: String
}
