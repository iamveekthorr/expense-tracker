use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    id: u32,
    description: String,
    amount: u32,
    category: Option<String>,
    date_created: NaiveDate,
    date_updated: Option<NaiveDate>,
}

struct UpdateExpense {
    description: Option<String>,
    amount: Option<u32>,
    category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Expenses {
    expenses: Vec<Expense>,
}

impl Expenses {
    pub fn new() -> Self {
        Self { expenses: vec![] }
    }

    pub fn add_expense(
        &mut self,
        description: String,
        amount: u32,
        category: Option<String>,
    ) -> Option<&'static str> {
        let created_at = Local::now().naive_local().date();

        let expense = Expense {
            id: self.next_id(),
            amount,
            description,
            category,
            date_created: created_at,
            date_updated: None,
        };

        self.expenses.push(expense);

        Some("Created successfully!")
    }

    pub fn list_expenses(&self) -> Option<&[Expense]> {
        Some(&self.expenses)
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

    pub fn update_expense(
        &mut self,
        id: u32,
        description: Option<String>,
        amount: Option<u32>,
        category: Option<String>,
    ) -> Option<&'static str> {
        let new_expense = UpdateExpense {
            amount,
            category,
            description,
        };

        for expense in &mut self.expenses {
            if expense.id == id {
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
        let amount = 20;
        let category = Some(String::from("subscriptions"));
        let description = String::from("Bought data plan from glo");

        let mut ex = Expenses::new();

        let message = ex.add_expense(description, amount, category);

        assert_eq!(Some("Created successfully!"), message);
    }

    #[test]
    fn it_should_delete_expense() {
        let id = 1;

        let amount = 20;
        let category = Some(String::from("subscriptions"));
        let description = String::from("Bought data plan from glo");

        let mut ex = Expenses::new();

        ex.add_expense(description, amount, category);

        let message = ex.delete_expense(id);

        assert_eq!(Some("Deleted successfully"), message);
    }

    #[test]
    fn it_should_list_expenses() {
        let mut ex = Expenses::new();
        assert_eq!(ex.expenses.len(), 0);

        let amount = 20;
        let category = Some(String::from("subscriptions"));
        let description = String::from("Bought data plan from glo");

        ex.add_expense(description, amount, category);

        assert_eq!(ex.expenses.len(), 1);
    }

    #[test]
    fn it_should_update_expense() {
        let id = 1;

        let amount = 20;
        let category = Some(String::from("subscriptions"));
        let description = String::from("Bought data plan from glo");

        let mut ex = Expenses::new();

        // add an expense
        ex.add_expense(description, amount, category);

        let amount = Some(50);
        let category = Some(String::from("Miscellaneous"));
        let description = Some(String::from("Updated value"));

        // update the expense
        let message = ex.update_expense(id, description, amount, category);

        assert_eq!(Some("Updated successfully!"), message);
    }
}
