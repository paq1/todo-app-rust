use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskDto {
    title: String
}

impl TaskDto {
    pub fn new(title: String) -> Self {
        TaskDto {title: title}
    }
}