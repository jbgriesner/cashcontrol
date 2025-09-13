use std::{fs, path::Path};

use crate::{domain::budget::Budget, storage::Repository};
use anyhow::Result;

pub struct JsonFileRepository {
    file_path: String,
}

impl JsonFileRepository {
    pub fn new(file_path: impl Into<String>) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }
}

#[async_trait::async_trait]
impl Repository for JsonFileRepository {
    async fn save(&self, budget: &Budget) -> Result<()> {
        let data = budget.to_bytes()?;
        fs::write(&self.file_path, data)?;
        Ok(())
    }

    async fn load(&self) -> Result<Budget> {
        if !self.exists().await {
            return Ok(Budget::new());
        }

        let data = fs::read(&self.file_path)?;
        Budget::from_bytes(&data)
    }

    async fn exists(&self) -> bool {
        Path::new(&self.file_path).exists()
    }
}
