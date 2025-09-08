pub mod json;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Repository<T> {
    async fn save(&self, item: &T) -> Result<()>;
    async fn load(&self) -> Result<T>;
    async fn exists(&self) -> bool;
}
