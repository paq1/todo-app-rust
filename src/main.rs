#[macro_use] extern crate rocket;

mod models;
mod api;
mod core;

use crate::api::repository::repository_tasks_mongo::RepositoryTaskMongo;
use crate::api::controller::tasks::{get_all, create_task, update, delete_task_by_id, delete_all};
use crate::core::services::repository::Repository;
use crate::api::repository::dbo::task_dbo::TaskDbo;
use crate::core::mapper::MapperModel;
use crate::models::task::Task;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let task_repository: RepositoryTaskMongo = RepositoryTaskMongo::new().await;
    let dbos: Vec<TaskDbo> = task_repository.read_all().await;
    let models: Vec<Task> = dbos.iter()
        .map(|dbo| dbo.to_model())
        .collect::<_>();

    affichage_des_models(&models);
    
    rocket::build()
        .manage(task_repository)
        .mount("/", routes![get_all, create_task, update, delete_task_by_id, delete_all])
        .launch().await
}

fn affichage_des_models(models: &Vec<Task>) {
    println!("{:#?}", models);
}