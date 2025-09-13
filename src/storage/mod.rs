pub mod json;
use anyhow::Result;
use async_trait::async_trait;

use crate::domain::budget::Budget;

#[async_trait]
pub trait Repository {
    async fn save(&self, item: &Budget) -> Result<()>;
    async fn load(&self) -> Result<Budget>;
    async fn exists(&self) -> bool;
}
