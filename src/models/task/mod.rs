
pub mod sub_task;

use crate::models::task::sub_task::SubTask;

#[derive(Debug)]
pub struct Task {
    id: String,
    title: String,
    sub_tasks: Vec<SubTask>
}

impl Task {
    pub fn new(id: String, title: String, sub_tasks: Vec<SubTask>) -> Self {
        Task {id: id, title: title, sub_tasks: sub_tasks }
    }
    pub fn get_title(&self) -> &String {&self.title}
    pub fn get_id(&self) -> &String {&self.id}
    pub fn get_sub_tasks(&self) -> &Vec<SubTask> {&self.sub_tasks}
}