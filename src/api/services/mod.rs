use crate::api::repository::repository_tasks_mongo::RepositoryTaskMongo;

struct Services {
    task_repository: RepositoryTaskMongo
}

impl Services {
    async fn new() -> Self {
        let ts = RepositoryTaskMongo::new().await;
        Services {task_repository: ts}
    }
}