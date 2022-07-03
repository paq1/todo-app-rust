use serde::{Deserialize, Serialize};
use crate::api::controller::dto::sub_task_dto::SubTaskDto;

#[derive(Serialize, Deserialize)]
pub struct TaskDto {
    id: Option<String>,
    title: String,
    sub_tasks: Vec<SubTaskDto>
}

impl TaskDto {
    pub fn new(id: Option<String>, title: String, sub_tasks: Vec<SubTaskDto>) -> Self {
        TaskDto {id, title, sub_tasks}
    }

    pub fn get_id(&self) -> &Option<String> { &self.id }
    pub fn get_title(&self) -> &String { &self.title }
    pub fn get_sub_tasks(&self) -> &Vec<SubTaskDto> { &self.sub_tasks }
}