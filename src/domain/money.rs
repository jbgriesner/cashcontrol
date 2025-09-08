use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Money(f64);

impl Money {
    pub fn new(amount: f64) -> Self {
        Money(amount)
    }

    pub fn amount(&self) -> f64 {
        self.0
    }

    pub fn is_positive(&self) -> bool {
        self.0 > 0.0
    }
}

impl std::ops::Add for Money {
    type Output = Money;
    fn add(self, other: Money) -> Money {
        Money(self.0 + other.0)
    }
}

impl std::ops::Sub for Money {
    type Output = Money;
    fn sub(self, other: Money) -> Money {
        Money(self.0 - other.0)
    }
}

impl std::iter::Sum for Money {
    fn sum<I: Iterator<Item = Money>>(iter: I) -> Money {
        Money(iter.map(|m| m.0).sum())
    }
}
