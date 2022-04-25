#[macro_use] extern crate rocket;

mod models;
mod api;
mod core;

use crate::api::repository::repository_tasks_mongo::RepositoryTaskMongo;
use crate::api::controller::tasks::{get_all, create_task, update, delete_task_by_id, delete_all};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // initialisation des services
    let task_repository: RepositoryTaskMongo = RepositoryTaskMongo::new().await;
    
    // lancement du backend
    rocket::build()
        .manage(task_repository) // injection du service db
        .mount("/", routes![get_all, create_task, update, delete_task_by_id, delete_all])
        .launch().await
}
