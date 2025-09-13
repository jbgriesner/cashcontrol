use chrono::NaiveDate;

use crate::domain::{money::Money, transaction::Transaction};

pub mod budget;
pub mod money;
pub mod transaction;

pub trait FinancialMetrics {
    fn balance(&self) -> Money;
    fn total_income(&self) -> Money;
    fn total_expenses(&self) -> Money;
}

pub trait Validate {
    type Error;
    fn validate(&self) -> Result<(), Self::Error>;
}

pub trait TransactionFilter {
    fn by_category(&self, category: &str) -> Vec<&Transaction>;
    fn by_date_range(&self, start: NaiveDate, end: NaiveDate) -> Vec<&Transaction>;
    fn by_amount_range(&self, min: Money, max: Money) -> Vec<&Transaction>;
}
