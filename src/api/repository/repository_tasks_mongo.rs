use async_trait::async_trait;

use mongodb::bson::{doc, Bson, Document};
use mongodb::{options::ClientOptions, options::FindOptions, Client, Collection, Cursor};
use mongodb::bson::oid::ObjectId;
use futures::stream::TryStreamExt;
use futures::future;

use crate::core::services::repository::Repository;
use crate::models::task::Task;
use crate::api::mapper::MapperDocument;
use crate::core::mapper::{MapperModel, MapperDbo};
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
    async fn create(&self, model: Task) -> Result<TaskDbo, String> {
        let doc: Document = model.to_document();
        let docs = vec![
            doc
        ];

        let value = self.collection.insert_many(docs, None).await.unwrap();
        let ids_map = value.inserted_ids;
        let mut ids: Vec<String> = Vec::new();

        for (_, value) in ids_map.into_iter() {
            let obj: &Bson = &value;
            let id = obj.as_object_id().unwrap().to_hex();
            ids.push(id)
        }

        let id: String = ids[0].clone();
        
        self.read(id).await
    }

    async fn read(&self, id: String) -> Result<TaskDbo, String> {
        let res: Vec<TaskDbo> = self.read_all().await
            .into_iter()
            .filter(|dbo| dbo.get_id() == id)
            .collect::<Vec<TaskDbo>>();
        
        if res.len() > 0 {
            Ok(res[0].clone())
        } else {
            Err("Il y a une erreur".to_string())
        }
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

    async fn update(&self, model: Task) {
        let dbo = model.to_dbo();
        let object_id: ObjectId = ObjectId::parse_str(dbo.get_id()).unwrap();

        let id_document: Document = doc! { "_id": object_id };
        let update = doc! { "$set": { "title": dbo.get_title() } };
        self.collection.update_one(id_document, update, None).await.unwrap();
    }

    async fn delete(&self, model: Task) {   
        let object_id: ObjectId = ObjectId::parse_str(model.get_id()).unwrap();
        let document = doc! {"_id": object_id };
        let delete_result = self.collection.delete_many(document, None).await.unwrap();
        println!("{}", delete_result.deleted_count);
    }

    async fn delete_all(&self) {
        // on recupere toutes les tasks
        let dbos: Vec<TaskDbo> = self.read_all().await;
        let models: Vec<Task> = dbos.iter().map(|dbo| dbo.to_model()).collect();

        future::join_all(
            models.iter().map(|model| {
                self.delete(model.clone())
            })
        ).await;
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
