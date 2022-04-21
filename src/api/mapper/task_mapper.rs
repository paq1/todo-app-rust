use crate::models::task::Task;
use mongodb::{bson::doc,bson::Document};

pub fn map_task_to_document(task: Task) -> Document {
    doc! { "title": task.get_title() }
}