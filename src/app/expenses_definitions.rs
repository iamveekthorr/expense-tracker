use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};

use super::storage::export_as_csv;

#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    pub id: u32,
    pub description: String,
    pub amount: u32,
    pub category: Option<String>,
    pub date_created: NaiveDate,
    pub date_updated: Option<NaiveDate>,
}
#[derive(Debug)]
pub struct CreateExpense {
    pub description: String,
    pub amount: u32,
    pub category: Option<String>,
}

pub struct UpdateExpense {
    pub id: u32,
    pub description: Option<String>,
    pub amount: Option<u32>,
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Expenses {
    expenses: Vec<Expense>,
}

impl Expenses {
    pub fn new() -> Self {
        Self { expenses: vec![] }
    }

    pub fn add_expense(&mut self, data: CreateExpense) -> Option<&'static str> {
        let created_at = Local::now().naive_local().date();

        let expense = Expense {
            id: self.next_id(),
            amount: data.amount,
            description: data.description,
            category: data.category,
            date_created: created_at,
            date_updated: None,
        };

        self.expenses.push(expense);

        Some("Created successfully!")
    }

    pub fn list_expenses(&self, category: Option<String>) -> Option<Vec<&Expense>> {
        if let Some(cat) = category {
            let cat = cat.to_lowercase(); // shadow the variable
            let expenses = self
                .expenses
                .iter()
                .filter(|ex| {
                    ex.category
                        .as_ref()
                        .map(|c| c.to_lowercase() == cat)
                        .unwrap_or(false)
                })
                .collect::<Vec<&Expense>>();

            Some(expenses)
        } else {
            Some(self.expenses.iter().collect())
        }
    }

    pub fn delete_expense(&mut self, id: u32) -> Option<&'static str> {
        let original_length = self.expenses.len(); // save the original length

        // only keep the elements where the id is not the specified id
        self.expenses.retain(|el| el.id != id);

        // makes sure that an element was actually deleted
        // if None is returned, that'll mean that that there was no id
        if self.expenses.len() == original_length {
            return None;
        }

        Some("Deleted successfully")
    }

    pub fn update_expense(&mut self, update_expense: UpdateExpense) -> Option<&'static str> {
        let new_expense = UpdateExpense {
            id: update_expense.id,
            amount: update_expense.amount,
            category: update_expense.category,
            description: update_expense.description,
        };

        for expense in &mut self.expenses {
            if expense.id == update_expense.id {
                if let Some(amount) = new_expense.amount {
                    expense.amount = amount;
                }

                if let Some(description) = new_expense.description {
                    expense.description = description;
                }

                if let Some(category) = new_expense.category {
                    expense.category = Some(category);
                }

                // update the field holding the updated_at date
                expense.date_updated = Some(Local::now().naive_local().date());

                return Some("Updated successfully!");
            }
        }
        None
    }

    pub fn summary(
        &self,
        month: Option<u8>,
        year: Option<String>,
    ) -> Option<(f64, Option<&'static str>)> {
        let mut sum = 0.0;
        let mut month_name: Option<&'static str> = None;

        for expense in &self.expenses {
            let mut matches = true;

            if let Some(month_flag) = month {
                month_name = Some(Self::get_month_name(month_flag as u32));
                // cast u8 -> u32 to enable comparison
                if expense.date_created.month() != month_flag as u32 {
                    matches = false;
                }
            }

            if let Some(year_flag) = &year {
                match year_flag.parse::<i32>() {
                    Ok(parsed_year) => {
                        if expense.date_created.year() != parsed_year {
                            matches = false;
                        }
                    }
                    Err(_) => {
                        matches = false;
                    }
                }
            }

            if matches {
                sum += expense.amount as f64;
            }
        }

        Some((sum.ceil(), month_name))
    }

    pub fn export_data_using_file_format(&self, format: Option<&str>) {
        let path_for_csv = "data/expenses.csv";
        let data = &self.expenses;

        match format {
            Some("csv") => export_as_csv(path_for_csv, data),
            _ => panic!("Invalid file format"),
        }
    }

    fn next_id(&self) -> u32 {
        self.expenses.iter().map(|item| item.id).max().unwrap_or(0) + 1
    }

    fn get_month_name(month: u32) -> &'static str {
        match month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_add_expense() {
        let expense = CreateExpense {
            amount: 20,
            category: Some(String::from("subscriptions")),
            description: String::from("Bought data plan from glo"),
        };

        let mut ex = Expenses::new();

        let message = ex.add_expense(expense);

        assert_eq!(Some("Created successfully!"), message);
    }

    #[test]
    fn it_should_delete_expense() {
        let id = 1;

        let expense = CreateExpense {
            amount: 20,
            category: Some(String::from("subscriptions")),
            description: String::from("Bought data plan from glo"),
        };
        let mut ex = Expenses::new();

        ex.add_expense(expense);

        let message = ex.delete_expense(id);

        assert_eq!(Some("Deleted successfully"), message);
    }

    #[test]
    fn it_should_list_expenses() {
        let mut ex = Expenses::new();
        assert_eq!(ex.expenses.len(), 0);

        let expense = CreateExpense {
            amount: 20,
            category: None,
            description: String::from("Bought data plan from glo"),
        };

        ex.add_expense(expense);

        let expenses = ex.list_expenses(None).unwrap();

        assert_eq!(expenses.len(), ex.expenses.len());
    }

    #[test]
    fn it_should_list_expenses_by_category() {
        let mut ex = Expenses::new();
        assert_eq!(ex.expenses.len(), 0);

        let category = Some(String::from("subscriptions"));
        let expense = CreateExpense {
            amount: 20,
            category,
            description: String::from("Bought data plan from glo"),
        };

        ex.add_expense(expense);

        let expenses = ex
            .list_expenses(Some(String::from("subscriptions")))
            .unwrap();

        assert_eq!(ex.expenses.len(), expenses.len());
    }

    #[test]
    fn it_should_update_expense() {
        let id = 1;

        let expense = CreateExpense {
            amount: 20,
            category: Some(String::from("subscriptions")),
            description: String::from("Bought data plan from glo"),
        };

        let mut ex = Expenses::new();

        // add an expense
        ex.add_expense(expense);

        let amount = Some(50);
        let category = Some(String::from("Miscellaneous"));
        let description = Some(String::from("Updated value"));

        let update_expense = UpdateExpense {
            id,
            description,
            amount,
            category,
        };

        // update the expense
        let message = ex.update_expense(update_expense);

        assert_eq!(Some("Updated successfully!"), message);
    }

    #[test]
    fn it_should_sum_expenses_by_month() {
        let expense = CreateExpense {
            amount: 20,
            category: Some(String::from("subscriptions")),
            description: String::from("Bought data plan from glo"),
        };

        let mut ex = Expenses::new();

        // add an expense
        ex.add_expense(expense);

        // show summary by month
        let month = Some(5); // 5th month is May

        let (result, month_name) = ex.summary(month, None).unwrap();

        // show 20.0 as float
        assert_eq!(result, 20.0);

        // show month name
        assert_eq!(month_name.unwrap(), "May")
    }

    #[test]
    fn it_should_sum_expenses() {
        let expense = CreateExpense {
            amount: 20,
            category: Some(String::from("subscriptions")),
            description: String::from("Bought data plan from glo"),
        };

        let mut ex = Expenses::new();

        // add an expense
        ex.add_expense(expense);

        let expense_2 = CreateExpense {
            amount: 100,
            category: Some(String::from("New CAT")),
            description: String::from("New Description"),
        };

        ex.add_expense(expense_2);

        let (result, _) = ex.summary(None, None).unwrap();

        // show 120.0 as float
        assert_eq!(result, 120.0);
    }
}
