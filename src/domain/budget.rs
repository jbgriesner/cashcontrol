use crate::domain::{
    money::Money,
    transaction::{Transaction, TransactionType},
    FinancialMetrics,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Budget {
    transactions: Vec<Transaction>,
    next_id: u64,
}

impl Budget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_transaction(&mut self, mut transaction: Transaction) -> Result<u64> {
        // transaction.validate()?;
        transaction.id = self.next_id;
        self.transactions.push(transaction);
        let id = self.next_id;
        self.next_id += 1;
        Ok(id)
    }

    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }
}

impl FinancialMetrics for Budget {
    fn balance(&self) -> Money {
        self.transactions
            .iter()
            .map(|t| Money::new(t.amount.amount() * t.transaction_type.sign()))
            .sum()
    }

    fn total_income(&self) -> Money {
        self.transactions
            .iter()
            .filter(|t| matches!(t.transaction_type, TransactionType::Income))
            .map(|t| t.amount)
            .sum()
    }

    fn total_expenses(&self) -> Money {
        self.transactions
            .iter()
            .filter(|t| matches!(t.transaction_type, TransactionType::Expense))
            .map(|t| t.amount)
            .sum()
    }
}
