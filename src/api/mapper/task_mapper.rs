use mongodb::{bson::doc,bson::Document};

use crate::models::task::Task;
use crate::api::controller::dto::task_dto::TaskDto;
use crate::api::repository::dbo::task_dbo::TaskDbo;
use crate::api::mapper::{MapperDocument, MapperDbo, MapperDto, MapperModel};

impl MapperDocument for Task {
    fn to_document(&self) -> Document {
        doc! { "title": self.get_title() } 
    }
}

impl MapperDbo<TaskDbo> for Task {
    fn to_dbo(&self) -> TaskDbo {
        TaskDbo::new(self.get_id(), self.get_title()) 
    }
}

impl MapperDto<TaskDto> for Task {
    fn to_dto(&self) -> TaskDto {
        TaskDto::new(self.get_id(), self.get_title()) 
    }
}

impl MapperModel<Task> for TaskDbo {
    fn to_model(&self) -> Task {
        Task::new(self.get_id(), self.get_title()) 
    }
}

impl MapperModel<Task> for TaskDto {
    fn to_model(&self) -> Task {
        Task::new(self.get_id(), self.get_title()) 
    }
}