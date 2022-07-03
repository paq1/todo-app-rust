use mongodb::{bson::doc,bson::Document};

use crate::models::task::sub_task::SubTask;
use crate::api::repository::dbo::sub_task_dbo::SubTaskDbo;
use crate::api::controller::dto::sub_task_dto::SubTaskDto;
use crate::core::mapper::{MapperDbo, MapperDto, MapperModel};
use crate::api::mapper::MapperDocument;

impl MapperDocument for SubTask {
    fn to_document(&self,  with_id: bool) -> Document {
        match self.get_description() {
            Some(description) => doc! { "id": self.get_id(), "title": self.get_title(), "description": description },
            None => doc! { "id": self.get_id(), "title": self.get_title() } 
        }
    }
}

impl MapperDbo<SubTaskDbo> for SubTask {
    fn to_dbo(&self) -> SubTaskDbo {
        SubTaskDbo::new(self.get_id(), self.get_title().clone(), self.get_description().clone())
    }
}

impl MapperDto<SubTaskDto> for SubTask {
    fn to_dto(&self) -> SubTaskDto {
        SubTaskDto::new(self.get_id(), self.get_title().clone(), self.get_description().clone()) 
    }
}

impl MapperModel<SubTask> for SubTaskDbo {
    fn to_model(&self) -> SubTask {

        SubTask::new(self.get_id(), self.get_title().clone(), self.get_description().clone()) 
    }
}


impl MapperModel<SubTask> for SubTaskDto {
    fn to_model(&self) -> SubTask {
        

        SubTask::new(self.get_id(), self.get_title().clone(), self.get_description().clone())
    }
}