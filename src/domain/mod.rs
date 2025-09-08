use crate::domain::money::Money;

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
