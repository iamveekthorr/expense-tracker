use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a new expense
    Add {
        #[arg(short, long)]
        description: String,

        #[arg(short, long)]
        category: Option<String>,

        #[arg(short, long)]
        amount: u32,
    },

    /// Delete an expense by ID
    Delete {
        #[arg(short, long)]
        id: u32,
    },

    Update {
        #[arg(short, long)]
        id: u32,

        #[arg(short, long)]
        amount: Option<u32>,

        #[arg(short, long)]
        description: Option<String>,

        #[arg(short, long)]
        category: Option<String>,
    },

    /// List all expenses
    List,

    /// Show monthly summary
    Summary {
        #[arg(short, long)]
        month: Option<u8>,
    },
}
