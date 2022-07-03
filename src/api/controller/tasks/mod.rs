use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::api::repository::repository_tasks_mongo::RepositoryTaskMongo;
use crate::api::controller::dto::{task_dto::TaskDto, error_json::ErrorJson};
use crate::api::repository::dbo::task_dbo::TaskDbo;
use crate::core::mapper::{MapperDto, MapperModel};
use crate::core::services::repository::Repository;
use crate::models::task::Task;
use crate::models::error::ErrorMessage;

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
        Err(err) => Err(Json(ErrorJson::new(err.0, Status::NotFound.code)))
    }
}

#[get("/tasks")]
pub async fn get_all(
    task_repository: &State<RepositoryTaskMongo>
) -> Result<Json<Vec<TaskDto>>, Json<ErrorJson>> {
    // on recupere nos dbos dans un result
    let dbos_res: Result<Vec<TaskDbo>, ErrorMessage> = task_repository
        .read_all().await;

    // on retourne un nos element si tout c'est bien passé
    // sinon on envoi un message d'erreur
    match dbos_res {
        Ok(dbos) => {
            let entities: Vec<TaskDto> = dbos
                .iter()
                .map(|dbo| dbo.to_model())
                .map(|model| model.to_dto())
                .collect::<_>();
            Ok(Json(entities))
        },
        Err(err) => Err(Json(ErrorJson::new(err.0, Status::NotFound.code)))
    }
}

#[put("/tasks", data = "<task_data_dto>")]
pub async fn update(
    task_repository: &State<RepositoryTaskMongo>,
    task_data_dto: Json<TaskDto>
) -> Result<Json<TaskDto>, Json<ErrorJson>> {
    let dto: TaskDto = task_data_dto.0;
    let model: Task = dto.to_model();
    task_repository.update(&model).await.unwrap();
    let id: String = match dto.get_id() {
        Some(id) => id.clone(),
        None => "".to_string()
    };
    match task_repository.read(id).await {
        Ok(task) => {
            let task_dto = task.to_model().to_dto();
            Ok(Json(task_dto))
        },
        Err(err) => Err(Json(ErrorJson::new(err.0, Status::NotFound.code)))
    }
}

#[delete("/tasks/<id>")]
pub async fn delete_task_by_id(
    task_repository: &State<RepositoryTaskMongo>,
    id: String
) -> Result<String, Json<ErrorJson>> {
    let dbos_res: Result<Vec<TaskDbo>, ErrorMessage> = task_repository
        .read_all().await;

    match dbos_res {
        Ok(dbos) => {
            let tasks: Vec<Task> = dbos
                .into_iter()
                .filter(|dbo| *dbo.get_id() == id)
                .map(|dbo| dbo.to_model())
                .collect::<_>();

            if tasks.len() > 0 {
                let model: &Task = &tasks[0];
                match task_repository.delete(model).await {
                    Ok (delete_id) => Ok(format! ("delete : {}", delete_id)),
                    Err(err)       => Err(Json(ErrorJson::new(err.0, Status::NotFound.code)))
                }
            } else {
                let message = format!("pas d'id : {}", id);
                Err(Json(ErrorJson::new(message, Status::NotFound.code)))
            }
        },
        Err(err) => Err(Json(ErrorJson::new(err.0, Status::NotFound.code)))
    }
}

#[delete("/tasks")]
pub async fn delete_all(
    task_repository: &State<RepositoryTaskMongo>
) -> Result<String, Json<ErrorJson>> {
    match task_repository.delete_all().await {
        Ok(message) => Ok(message),
        Err(err)    => Err(Json(ErrorJson::new(err.0, Status::NotFound.code)))
    }
}
