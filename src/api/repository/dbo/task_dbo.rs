pub struct TaskDbo {
    id: String,
    title: String
}

impl TaskDbo {
    pub fn new(id: String, title: String) -> Self { TaskDbo {id, title} }
    pub fn get_id(&self) -> String { self.id.clone() }
    pub fn get_title(&self) -> String { self.title.clone() }
}