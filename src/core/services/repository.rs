use async_trait::async_trait;
use std::vec::Vec;

#[async_trait]
pub trait Repository<Model, Dbo> {
    async fn create(&self, model: Model) -> Result<Dbo, String>;
    async fn read(&self, id: String) -> Result<Dbo, String>;
    async fn read_all(&self) -> Vec<Dbo>;
    async fn update(&self, model: Model);
    async fn delete(&self, model: Model);
    async fn delete_all(&self);
}