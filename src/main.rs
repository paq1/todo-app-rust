mod models;
mod api;
mod core;

use crate::api::repository::repository_tasks_mongo::RepositoryTaskMongo;
use crate::core::services::repository::Repository;
use crate::models::task::Task;

#[rocket::main]
async fn main() -> mongodb::error::Result<()> {
    println!("Hello, world!");

    let task_repository: RepositoryTaskMongo = RepositoryTaskMongo::new().await;
    let task = Task::new("use service".to_string());
    task_repository.create(task).await;
    println!("creation Ok");
    let datas: Vec<Task> = task_repository.read_all().await;
    println!("{:?}", datas);
    Ok(())
}
