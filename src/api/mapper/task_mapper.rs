use mongodb::{bson::doc,bson::Document};

use crate::models::task::Task;
use crate::models::task::sub_task::SubTask;
use crate::api::controller::dto::task_dto::TaskDto;
use crate::api::controller::dto::sub_task_dto::SubTaskDto;
use crate::api::repository::dbo::task_dbo::TaskDbo;
use crate::core::mapper::{MapperDbo, MapperDto, MapperModel};
use crate::api::mapper::MapperDocument;

impl MapperDocument for Task {
    fn to_document(&self,  with_id: bool) -> Document {
        let sub_tasks_model_doc: Vec<Document> = self.get_sub_tasks()
            .iter()
            .map(|model| model.to_document(false)) // id a mapper car c'est un id à la con
            .collect::<_>();

        if with_id {
            doc! { "id": self.get_id(), "title": self.get_title(), "sub_tasks": sub_tasks_model_doc } 
        } else {
            doc! { "title": self.get_title(), "sub_tasks": sub_tasks_model_doc } 
        }
    }
}

impl MapperDbo<TaskDbo> for Task {
    fn to_dbo(&self) -> TaskDbo {

        let sub_tasks_dbo = self.get_sub_tasks().iter()
            .map(|model| model.to_dbo())
            .collect::<_>();

        TaskDbo::new(self.get_id().clone(), self.get_title().clone(), sub_tasks_dbo) 
    }
}

impl MapperDto<TaskDto> for Task {
    fn to_dto(&self) -> TaskDto {

        let sub_tasks_dto: Vec<SubTaskDto> = self.get_sub_tasks().iter()
            .map(|model| model.to_dto())
            .collect();

        TaskDto::new(Some(self.get_id().clone()), self.get_title().clone(), sub_tasks_dto) 
    }
}

impl MapperModel<Task> for TaskDbo {
    fn to_model(&self) -> Task {
        
        let sub_tasks_model = self.get_sub_tasks().iter()
            .map(|model| model.to_model())
            .collect::<_>();

        Task::new(self.get_id().clone(), self.get_title().clone(), sub_tasks_model) 
    }
}

impl MapperModel<Task> for TaskDto {
    fn to_model(&self) -> Task {
        let id: String = match self.get_id() {
            Some(id) => id.clone(),
            None => "".to_string()
        };

        let sub_tasks_model = self.get_sub_tasks().iter()
            .map(|model| model.to_model())
            .collect::<_>();

        Task::new(id, self.get_title().clone(), sub_tasks_model)
    }
}