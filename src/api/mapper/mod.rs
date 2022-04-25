use mongodb::{bson::Document};

pub mod task_mapper;

pub trait MapperDocument {
    fn to_document(&self) -> Document;
}

pub trait MapperDto<Dto> {
    fn to_dto(&self) -> Dto;
}

pub trait MapperDbo<Dbo> {
    fn to_dbo(&self) -> Dbo;
}

pub trait MapperModel<Model> {
    fn to_model(&self) -> Model;
}