#[derive(Clone)]
pub struct TaskDbo {
    id: String,
    title: String,
    sub_tasks: Vec<TaskDbo>
}

impl TaskDbo {
    pub fn new(id: String, title: String, sub_tasks: Vec<TaskDbo>) -> Self { TaskDbo {id, title, sub_tasks} }
    pub fn get_id(&self) -> String { self.id.clone() }
    pub fn get_title(&self) -> String { self.title.clone() }
    pub fn get_sub_tasks(&self) -> Vec<TaskDbo> { self.sub_tasks.clone() }
}