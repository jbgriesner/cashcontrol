use clap::{arg, command, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "budget")]
#[command(about = "Un gestionnaire de budget personnel simple")]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(short, long)]
        amount: f64,
        #[arg(short, long)]
        category: String,
        #[arg(short, long)]
        description: String,
        #[arg(long)]
        income: bool,
    },
    List,
    Balance,
    Category {
        name: String,
    },
    Stats,
}
