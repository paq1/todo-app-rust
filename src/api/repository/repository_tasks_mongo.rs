use async_trait::async_trait;

use mongodb::bson::{doc, Bson, Document};
use mongodb::{options::ClientOptions, options::FindOptions, Client, Database, Collection, Cursor};
use mongodb::bson::oid::ObjectId;
use futures::stream::TryStreamExt;

use crate::core::services::repository::Repository;
use crate::models::task::Task;
use crate::api::mapper::task_mapper::map_task_to_document;

pub struct RepositoryTaskMongo {
    client: Client,
    db: Database,
    collection: Collection<Document>
}

impl RepositoryTaskMongo {
    pub async fn new() -> Self {
        let mut client_options_f: mongodb::error::Result<ClientOptions> = ClientOptions::parse("mongodb://localhost:27017").await;
        let mut client_options: ClientOptions = client_options_f.unwrap();
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
        let toto: Document = doc! { "title": model.get_title() };
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
            let title_bson: &Bson = &task.get("title").unwrap();
            let id_bson: &Bson = &task.get("_id").unwrap();
            let title_str: String = title_bson.as_str().unwrap().to_string();
            let obj_id: ObjectId = id_bson.as_object_id().unwrap();
            let id_str: String = obj_id.to_hex();
            println!("id : {}", id_str);
            lst.push(Task::new(title_str))
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
