mod app;
mod cli;
// mod expenses;

use app::{
    expenses_definitions::Expenses,
    storage::{load_from_file, save_to_file},
};
use clap::Parser;
use cli::argument_parser::{Args, Commands};

fn main() {
    let args = Args::parse();

    let path = "data/expenses.json";

    let mut expenses = load_from_file::<Expenses>(path).unwrap_or_else(|_| Expenses::new()); // creates an empty array if reading the expenses file fails

    match args.command {
        Commands::Add {
            description,
            amount,
            category,
        } => {
            // create expense struct

            let msg = expenses
                .add_expense(description, amount, category)
                .unwrap_or("Something went wrong while adding expense");

            // save to file
            let _ = save_to_file(path, &expenses);

            // print success
            println!("{msg}");
        }

        Commands::Delete { id } => {
            let msg = expenses
                .delete_expense(id)
                .unwrap_or("No expense found with that id");

            // save to file
            let _ = save_to_file(path, &expenses);

            println!("{msg}");
        }

        Commands::List => {
            if let Some(list) = expenses.list_expenses() {
                println!("{:#?}", list);
            };
        }

        Commands::Summary { month } => {
            println!("Showing summary for month: {:?}", month);
            // Call summary logic here
        }

        Commands::Update {
            id,
            amount,
            description,
            category,
        } => {
            let msg = expenses
                .update_expense(id, description, amount, category)
                .unwrap_or("No expense found with that id");

            // save to file
            let _ = save_to_file(path, &expenses);

            println!("{msg}");
        }
    }
}
