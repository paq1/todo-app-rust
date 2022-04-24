use async_trait::async_trait;

use rocket::request::{self, Request, FromRequest, Outcome};
use rocket::State;
use rocket::serde::json::Json;

use crate::api::repository::repository_tasks_mongo::RepositoryTaskMongo;
use crate::api::dto::task_dto::TaskDto;
use crate::api::mapper::task_mapper::map_task_to_taskDto;
use crate::core::services::repository::Repository;
use crate::models::task::Task;

pub struct TT { 
    pub user_val: String
}

impl TT {
    pub fn new() -> Self {
        print!(" ---------------------- nouvelle instance");
        TT {user_val: "coucou".to_string()}
    }
}

#[get("/test")]
pub async fn get_all(
    state: &State<TT>, 
    taskRepository: &State<RepositoryTaskMongo>
) -> Json<Vec<TaskDto>> {
    let models: Vec<Task> = taskRepository.read_all().await;
    let entities: Vec<TaskDto> = models
        .iter()
        .map(|model| map_task_to_taskDto(model.clone()))
        .collect::<_>();

    Json(entities)
}
