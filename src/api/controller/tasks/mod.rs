use async_trait::async_trait;

use rocket::request::{self, Request, FromRequest, Outcome};
use rocket::State;
use rocket::http::Status;
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

#[get("/tasks")]
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

#[post("/tasks", data = "<task_dto_json>")]
pub async fn create_task(
    task_repository: &State<RepositoryTaskMongo>,
    task_dto_json: Json<TaskDto>
) -> Status {
    // etape 1 on cast TaskDto json vers TaskDto
    let task_dto: TaskDto = task_dto_json.0;

    // on transform notre dto en model
    let task = Task::new(task_dto.get_title());

    // on ajoute notre model en db
    task_repository.create(task).await;

    // tout c'est bien passé (pas de panic ici)
    Status::Ok
}
