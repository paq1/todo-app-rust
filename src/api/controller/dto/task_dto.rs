use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskDto {
    id: String,
    title: String
}

impl TaskDto {
    pub fn new(id: String, title: String) -> Self {
        TaskDto {id: id, title: title}
    }

    pub fn get_title(&self) -> String { self.title.clone() }
    pub fn get_id(&self) -> String { self.id.clone() }
}