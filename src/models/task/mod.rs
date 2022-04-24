#[derive(Clone, Debug)]
pub struct Task {
    title: String
}

impl Task {
    pub fn new(title: String) -> Self {
        Task { title: title }
    }
    pub fn get_title(&self) -> String {self.title.clone()}
}