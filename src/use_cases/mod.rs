use crate::{
    domain::{budget::Budget, transaction::Transaction, FinancialMetrics, TransactionFilter},
    storage::Repository,
};
use anyhow::Result;

pub struct BudgetService<R> {
    repository: R,
    budget: Budget,
}

impl<R: Repository> BudgetService<R> {
    pub async fn new(repository: R) -> Result<Self> {
        let budget = repository.load().await?;
        Ok(Self { repository, budget })
    }

    pub async fn add_transaction(&mut self, transaction: Transaction) -> Result<u64> {
        let id = self.budget.add_transaction(transaction)?;
        self.repository.save(&self.budget).await?;
        Ok(id)
    }

    pub fn metrics(&self) -> &impl FinancialMetrics {
        &self.budget
    }

    pub fn get_filter(&self) -> &impl TransactionFilter {
        &self.budget
    }

    pub fn transactions(&self) -> &[Transaction] {
        self.budget.transactions()
    }
}
