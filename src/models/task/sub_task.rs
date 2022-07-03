#[derive(Debug)]
pub struct SubTask {
    id: u32,
    title: String,
    description: Option<String>
}

impl SubTask {
    
    pub fn new(id: u32, title: String, description: Option<String>) -> Self {
        SubTask {id, title, description }
    }

    pub fn get_title(&self) -> &String { &self.title }
    pub fn get_id(&self) -> u32 { self.id }
    pub fn get_description(&self) -> &Option<String> { &self.description }
}