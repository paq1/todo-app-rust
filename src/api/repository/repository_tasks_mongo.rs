use async_trait::async_trait;

use mongodb::bson::{doc, Bson, Document};
use mongodb::{options::ClientOptions, options::FindOptions, Client, Collection, Cursor};
use mongodb::bson::oid::ObjectId;
use futures::stream::TryStreamExt;

use crate::core::services::repository::Repository;
use crate::models::task::Task;
use crate::api::mapper::{MapperDocument};
use crate::api::repository::dbo::task_dbo::TaskDbo;

static DB_NAME: &str = "todo-db";
static COLLECTION_NAME: &str = "tasks";

pub struct RepositoryTaskMongo {
    collection: Collection<Document>
}

impl RepositoryTaskMongo {
    pub async fn new() -> Self {
        let client_options_f: mongodb::error::Result<ClientOptions> = ClientOptions::parse("mongodb://localhost:27017").await;
        let client_options: ClientOptions = client_options_f.unwrap();
        let client: Client = Client::with_options(client_options).unwrap();

        // Ping the server to see if you can connect to the server
        check_connection(&client).await.unwrap();

        let db = client.database(DB_NAME);
        let collection = db.collection::<Document>(COLLECTION_NAME);

        RepositoryTaskMongo { collection }
    }
}

#[async_trait]
impl Repository<Task, TaskDbo> for RepositoryTaskMongo {
    async fn create(&self, model: Task) {
        let doc: Document = model.to_document();
        let docs = vec![
            doc
        ];

        self.collection.insert_many(docs, None).await.unwrap();
    }

    async fn read_all(&self) -> Vec<TaskDbo> {
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
            
            // contruction du dbo
            let task_dbo: TaskDbo = TaskDbo::new(id_str, title_str);
            lst.push(task_dbo);
        }
        lst
    }
}

async fn check_connection(client: &Client) -> mongodb::error::Result<()> {
    client
        .database(DB_NAME)
        .run_command(doc! {"ping": 1}, None)
        .await?;

    println!("connection successful");
    Ok(())
}
