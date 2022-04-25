use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::api::repository::repository_tasks_mongo::RepositoryTaskMongo;
use crate::api::controller::dto::{task_dto::TaskDto, error::ErrorJson};
use crate::api::repository::dbo::task_dbo::TaskDbo;
use crate::core::mapper::{MapperDto, MapperModel};
use crate::core::services::repository::Repository;
use crate::models::task::Task;

#[post("/tasks", data = "<task_dto_json>")]
pub async fn create_task(
    task_repository: &State<RepositoryTaskMongo>,
    task_dto_json: Json<TaskDto>
) -> Result<Json<TaskDto>, Json<ErrorJson>> {
    // etape 1 on cast TaskDto json vers TaskDto
    let task_dto: TaskDto = task_dto_json.0;

    // on transform notre dto en model
    let model = task_dto.to_model();

    // on ajoute notre model en db
    match task_repository.create(model).await {
        Ok(task) => Ok(Json(task.to_model().to_dto())),
        Err(err) => Err(Json(ErrorJson::new(err.to_string(), Status::NotFound.code)))
    }
}

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

#[put("/tasks", data = "<task_data_dto>")]
pub async fn update(
    task_repository: &State<RepositoryTaskMongo>,
    task_data_dto: Json<TaskDto>
) -> Result<Json<TaskDto>, Json<ErrorJson>> {
    let dto: TaskDto = task_data_dto.0;
    let model: Task = dto.to_model();
    task_repository.update(model).await;

    let id: String = dto.get_id().unwrap();
    
    match task_repository.read(id).await {
        Ok(task) => {
            let task_dto = task.to_model().to_dto();
            Ok(Json(task_dto))
        },
        Err(err) => {
            let error: ErrorJson = ErrorJson::new(err.to_string(), Status::NotFound.code);
            Err(Json(error))
        }
    }
} 

#[delete("/tasks/<id>")]
pub async fn delete_task_by_id(
    task_repository: &State<RepositoryTaskMongo>,
    id: String
) -> String {
    let dbos: Vec<TaskDbo> = task_repository.read_all().await;
    let tasks: Vec<Task> = dbos
        .into_iter()
        .filter(|dbo| dbo.get_id() == id)
        .map(|dbo| dbo.to_model())
        .collect::<_>();

    if tasks.len() > 0 {
        let model: Task = tasks[0].clone();
        task_repository.delete(model).await;
        format! ("nombre de suppression : {}", tasks.len())
    } else {
        format! ("pas d'id : {}", id)
    }
}

#[delete("/tasks")]
pub async fn delete_all(
    task_repository: &State<RepositoryTaskMongo>
) -> String {
    task_repository.delete_all().await;
    "Ok".to_string()
}
