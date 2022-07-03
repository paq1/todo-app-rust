use async_trait::async_trait;
use std::vec::Vec;

use crate::models::error::ErrorMessage;

#[async_trait]
pub trait Repository<Model, Dbo, Id> {
    async fn create(&self, model: Model) -> Result<Dbo, ErrorMessage>;
    async fn read(&self, id: String) -> Result<Dbo, ErrorMessage>;
    async fn read_all(&self) -> Result<Vec<Dbo>, ErrorMessage>;
    async fn update(&self, model: &Model) -> Result<Dbo, ErrorMessage>;
    async fn delete(&self, model: &Model) -> Result<Id, ErrorMessage>;
    async fn delete_all(&self) -> Result<String, ErrorMessage>; // todo trouver autre chose que String
}