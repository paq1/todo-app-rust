use mongodb::{bson::Document};

pub mod task_mapper;

pub trait MapperDocument {
    fn to_document(&self, with_id: bool) -> Document;
}
