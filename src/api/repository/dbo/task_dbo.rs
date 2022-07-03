use crate::api::repository::dbo::sub_task_dbo::SubTaskDbo;

#[derive(Debug, Clone)]
pub struct TaskDbo {
    id: String,
    title: String,
    sub_tasks: Vec<SubTaskDbo>
}

impl TaskDbo {
    pub fn new(id: String, title: String, sub_tasks: Vec<SubTaskDbo>) -> Self { TaskDbo {id, title, sub_tasks} }

    pub fn get_id(&self) -> &String { &self.id }
    pub fn get_title(&self) -> &String { &self.title }
    pub fn get_sub_tasks(&self) -> &Vec<SubTaskDbo> { &self.sub_tasks }
}