use async_trait::async_trait;
use crate::api::repository::repository_tasks_mongo::RepositoryTaskMongo;
use crate::core::services::repository::Repository;
use crate::models::task::Task;
use rocket::request::{self, Request, FromRequest, Outcome};
use rocket::State;

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
) -> String {
    let entities: Vec<Task> = taskRepository.read_all().await;

    let titles: Vec<String> = entities
        .iter()
        .map(|entity| entity.get_title())
        .collect::<Vec<String>>();

    let res: String = titles.join("\n");
    res
}
