use crate::domain::{
    money::Money,
    transaction::{Transaction, TransactionType},
    FinancialMetrics, TransactionFilter,
};
use anyhow::Result;
use chrono::NaiveDate;
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

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let json = serde_json::to_string_pretty(self)?;
        Ok(json.into_bytes())
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let json = String::from_utf8(data.to_vec())?;
        let budget: Budget = serde_json::from_str(&json)?;
        Ok(budget)
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

impl TransactionFilter for Budget {
    fn by_category(&self, category: &str) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|t| t.category == category)
            .collect()
    }

    fn by_date_range(&self, start: NaiveDate, end: NaiveDate) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|t| t.date >= start && t.date <= end)
            .collect()
    }

    fn by_amount_range(&self, min: Money, max: Money) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|t| t.amount >= min && t.amount <= max)
            .collect()
    }
}
