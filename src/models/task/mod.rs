#[derive(Clone, Debug)]
pub struct Task {
    id: String,
    title: String,
    sub_tasks: Vec<Task>
}

impl Task {
    pub fn new(id: String, title: String, sub_tasks: Vec<Task>) -> Self {
        Task {id: id, title: title, sub_tasks: sub_tasks }
    }
    pub fn get_title(&self) -> String {self.title.clone()}
    pub fn get_id(&self) -> String {self.id.clone()}
    pub fn get_sub_tasks(&self) -> Vec<Task> {self.sub_tasks.clone()}
}