#[macro_use] extern crate rocket;

mod models;
mod api;
mod core;

use crate::api::repository::repository_tasks_mongo::RepositoryTaskMongo;
use crate::api::controller::tasks::get_all;
use crate::core::services::repository::Repository;
use crate::models::task::Task;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    println!("Hello, world!");

    let task_repository: RepositoryTaskMongo = RepositoryTaskMongo::new().await;
    let task = Task::new("finir le projet notification".to_string());
    task_repository.create(task).await;
    println!("creation Ok");
    let datas: Vec<Task> = task_repository.read_all().await;
    println!("{:?}", datas);
    
    rocket::build().mount("/", routes![get_all]).launch().await
}
