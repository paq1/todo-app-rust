use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SubTaskDto {
    id: u32,
    title: String,
    description: Option<String>
}

impl SubTaskDto {
    pub fn new(id: u32, title: String, description: Option<String>) -> Self {
        SubTaskDto {id, title, description}
    }

    pub fn get_id(&self) -> u32 { self.id }
    pub fn get_title(&self) -> &String { &self.title }
    pub fn get_description(&self) -> &Option<String> { &self.description }
}