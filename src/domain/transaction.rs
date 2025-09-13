use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::{money::Money, Validate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Income,
    Expense,
}

impl TransactionType {
    pub fn sign(&self) -> f64 {
        match self {
            TransactionType::Income => 1.0,
            TransactionType::Expense => -1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u64,
    pub date: NaiveDate,
    pub amount: Money,
    pub transaction_type: TransactionType,
    pub category: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum TransactionValidationError {
    InvalidAmount,
    EmptyDescription,
    EmptyCategory,
}

impl Validate for Transaction {
    type Error = TransactionValidationError;

    fn validate(&self) -> Result<(), Self::Error> {
        if !self.amount.is_positive() {
            return Err(TransactionValidationError::InvalidAmount);
        }
        if self.description.trim().is_empty() {
            return Err(TransactionValidationError::EmptyDescription);
        }
        if self.category.trim().is_empty() {
            return Err(TransactionValidationError::EmptyCategory);
        }
        Ok(())
    }
}
