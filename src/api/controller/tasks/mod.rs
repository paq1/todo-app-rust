use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::api::repository::repository_tasks_mongo::RepositoryTaskMongo;
use crate::api::controller::dto::task_dto::TaskDto;
use crate::api::repository::dbo::task_dbo::TaskDbo;
use crate::core::mapper::{MapperDto, MapperModel};
use crate::core::services::repository::Repository;

#[get("/tasks")]
pub async fn get_all(
    task_repository: &State<RepositoryTaskMongo>
) -> Json<Vec<TaskDto>> {
    let dbos: Vec<TaskDbo> = task_repository.read_all().await;
    let entities: Vec<TaskDto> = dbos
        .iter()
        .map(|dbo| dbo.to_model())
        .map(|model| model.to_dto())
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
    let task = task_dto.to_model();

    // on ajoute notre model en db
    task_repository.create(task).await;

    // tout c'est bien passé (pas de panic ici)
    Status::Ok
}
