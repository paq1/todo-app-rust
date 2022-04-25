#[derive(Clone, Debug)]
pub struct Task {
    id: String,
    title: String
}

impl Task {
    pub fn new(id: String, title: String) -> Self {
        Task {id: id, title: title }
    }
    pub fn get_title(&self) -> String {self.title.clone()}
    pub fn get_id(&self) -> String {self.id.clone()}
}