use async_trait::async_trait;

use crate::core::services::repository::Repository;
use crate::models::task::Task;
use mongodb::bson;
use mongodb::{bson::doc, options::ClientOptions, options::FindOptions, Client, Database, Collection, bson::Document, Cursor};
use crate::api::mapper::task_mapper::map_task_to_document;
use futures::stream::TryStreamExt;

pub struct RepositoryTaskMongo {
    client: Client,
    db: Database,
    collection: Collection<Document>
}

impl RepositoryTaskMongo {
    pub async fn new() -> Self {
        let mut client_options_f: mongodb::error::Result<ClientOptions> = ClientOptions::parse("mongodb://localhost:27017").await;
        let mut client_options: ClientOptions = ClientOptions::parse("mongodb://localhost:27017").await.unwrap(); // ? equivalent to .unwrap()
        let client: Client = Client::with_options(client_options).unwrap();

        // Ping the server to see if you can connect to the server
        check_connection(&client).await;

        let db = client.database("todo-db");
        let collection = db.collection::<Document>("tasks");

        RepositoryTaskMongo {client, db, collection}
    }
}

#[async_trait]
impl Repository<Task, Task> for RepositoryTaskMongo {
    async fn create(&self, model: Task) {
        let toto = doc! { "title": model.get_title() };
        let docs = vec![
            map_task_to_document(model)
        ];

        self.collection.insert_many(docs, None).await;
    }

    async fn read_all(&self) -> Vec<Task> {
        let filter = doc! {};
        let find_options = FindOptions::builder().build();
        let mut cursor: Cursor<Document> = self.collection.find(filter, find_options).await.unwrap();
        let mut lst = Vec::new();
        while let Some(task) = cursor.try_next().await.unwrap() {
            let title = &task.get("title").unwrap();
            lst.push(Task::new(title.to_string()))
        }
        lst
    }
}

async fn check_connection(client: &Client) -> mongodb::error::Result<()> {
    client
        .database("toto")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    println!("connection successful");
    Ok(())
}