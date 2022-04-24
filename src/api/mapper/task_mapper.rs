use crate::models::task::Task;
use crate::api::dto::task_dto::TaskDto;
use mongodb::{bson::doc,bson::Document};

pub fn map_task_to_document(task: Task) -> Document {
    doc! { "title": task.get_title() }
}

pub fn map_task_to_taskDto(task: Task) -> TaskDto {
    TaskDto::new(task.get_title())
}