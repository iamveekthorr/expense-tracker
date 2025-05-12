use clap::Parser;

use crate::cli::argument_parser::{Args, Commands};

use super::{
    expenses_definitions::Expenses,
    storage::{load_from_file, save_to_file},
};

pub fn run() {
    let args = Args::parse();

    let path = "data/expenses.json";

    let mut expenses = load_from_file::<Expenses>(path).unwrap_or_else(|_| Expenses::new());
    // creates an empty array if reading the expenses file fails

    match args.command {
        Commands::Add {
            description,
            amount,
            category,
        } => {
            if amount < 1 {
                panic!("Amount cannot be less than 1 got {amount}");
            }

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
                .unwrap_or("No expense found with id");

            // save to file
            let _ = save_to_file(path, &expenses);

            println!("{msg}: {id}");
        }

        Commands::List => {
            if let Some(list) = expenses.list_expenses() {
                // format to readable json string
                let json_string = serde_json::to_string_pretty(list)
                    .map_err(|_| format!("unable to format to JSON"))
                    .unwrap_or(String::from("Something went wrong"));

                println!("{}", json_string);
            };
        }

        Commands::Summary { month, year } => {
            // Call summary logic here
            let (summary, month_name) = expenses.summary(month, year).unwrap_or((0.0, None));

            if let Some(name_of_month) = month_name {
                println!("Total Expenses for {name_of_month}: NGN {:?}", summary);
            } else {
                println!("Total Expenses: NGN {:?}", summary);
            }
        }

        Commands::Update {
            id,
            amount,
            description,
            category,
        } => {
            if let Some(amt) = amount {
                if amt < 1 {
                    panic!("Amount cannot be less than 1 got {amt}");
                }
            }

            let msg = expenses
                .update_expense(id, description, amount, category)
                .unwrap_or("No expense found with that id");

            // save to file
            let _ = save_to_file(path, &expenses);

            println!("{msg}");
        }
    }
}
