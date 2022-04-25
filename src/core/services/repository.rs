use async_trait::async_trait;
use std::vec::Vec;

#[async_trait]
pub trait Repository<Model, Dbo> {
    async fn create(&self, model: Model);
    async fn read_all(&self) -> Vec<Dbo>;
}