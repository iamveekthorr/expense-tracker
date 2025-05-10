mod cli;

use clap::Parser;
use cli::argument_parser::{Args, Commands};

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Add {
            description,
            amount,
            category,
        } => {
            println!(
                "Adding expense: {} - {} - {:?}",
                description, amount, category
            );
            // Call add logic here
        }

        Commands::Delete { id } => {
            println!("Deleting expense with ID: {}", id);
            // Call delete logic here
        }

        Commands::List => {
            println!("Listing all expenses");
            // Call list logic here
        }

        Commands::Summary { month } => {
            println!("Showing summary for month: {:?}", month);
            // Call summary logic here
        }
    }
}
