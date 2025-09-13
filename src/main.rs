use chrono::Local;
use clap::Parser;

use crate::{
    cli::{Cli, Commands},
    domain::{
        money::Money,
        transaction::{Transaction, TransactionType},
        FinancialMetrics, TransactionFilter,
    },
    storage::json::JsonFileRepository,
    use_cases::BudgetService,
};

mod cli;
mod domain;
mod storage;
mod use_cases;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let repository = JsonFileRepository::new("budget.json");
    let mut service = BudgetService::new(repository).await?;

    match cli.command {
        Commands::Add {
            amount,
            category,
            description,
            income,
        } => {
            let transaction = Transaction {
                id: 0,
                date: Local::now().date_naive(),
                amount: Money::new(amount),
                transaction_type: if income {
                    TransactionType::Income
                } else {
                    TransactionType::Expense
                },
                category,
                description,
            };

            let id = service.add_transaction(transaction).await?;
            println!("Transaction ajoutée avec l'ID: {}", id);
        }

        Commands::List => {
            for transaction in service.transactions() {
                println!(
                    "{} | {:?} | {:.2}€ | {} | {}",
                    transaction.date,
                    transaction.transaction_type,
                    transaction.amount.amount(),
                    transaction.category,
                    transaction.description
                );
            }
        }

        Commands::Balance => {
            let metrics = service.metrics();
            println!("Solde: {:.2}€", metrics.balance().amount());
        }

        Commands::Category { name } => {
            let filter = service.get_filter();
            let transactions = filter.by_category(&name);
            println!("Transactions '{}':", name);
            for t in transactions {
                println!("{} | {:.2}€ | {}", t.date, t.amount.amount(), t.description);
            }
        }

        Commands::Stats => {
            let metrics = service.metrics();
            println!("=== STATISTIQUES ===");
            println!("Revenus totaux: {:.2}€", metrics.total_income().amount());
            println!(
                "Dépenses totales: {:.2}€",
                metrics.total_expenses().amount()
            );
            println!("Solde: {:.2}€", metrics.balance().amount());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::domain::{money::Money, transaction::TransactionType, Validate};

    use super::*;

    #[test]
    fn test_money_operations() {
        let a = Money::new(100.0);
        let b = Money::new(50.0);
        assert_eq!((a + b).amount(), 150.0);
        assert_eq!((a - b).amount(), 50.0);
    }

    #[test]
    fn test_transaction_validation() {
        let valid_transaction = Transaction {
            id: 1,
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            amount: Money::new(100.0),
            transaction_type: TransactionType::Income,
            category: "Salaire".to_string(),
            description: "Test".to_string(),
        };

        assert!(valid_transaction.validate().is_ok());

        let invalid_transaction = Transaction {
            amount: Money::new(-100.0), // Invalid!
            ..valid_transaction
        };

        assert!(invalid_transaction.validate().is_err());
    }
}
