use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskDto {
    id: Option<String>,
    title: String,
    sub_tasks: Vec<TaskDto>
}

impl TaskDto {
    pub fn new(id: Option<String>, title: String, sub_tasks: Vec<TaskDto>) -> Self {
        TaskDto {id: id, title: title, sub_tasks: sub_tasks}
    }

    pub fn get_title(&self) -> String { self.title.clone() }
    pub fn get_id(&self) -> Option<String> { self.id.clone() }
    pub fn get_sub_tasks(&self) -> Vec<TaskDto> { self.sub_tasks.clone() }
}