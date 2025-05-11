use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    id: u32,
    description: String,
    amount: u32,
    category: Option<String>,
}

pub struct UpdateExpense {
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

    pub fn add_expense(&mut self, expense: Expense) -> Option<&'static str> {
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

    pub fn update_expense(&mut self, id: u32, new_expense: UpdateExpense) -> Option<&'static str> {
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
                return Some("Updated successfully!");
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_add_expense() {
        let expense = Expense {
            id: 1,
            amount: 20,
            category: Some(String::from("subscriptions")),
            description: String::from("Bought data plan from glo"),
        };

        let mut expenses = Expenses::new();
        let message = expenses.add_expense(expense);

        assert_eq!(Some("Created successfully!"), message);
    }

    #[test]
    fn it_should_delete_expense() {
        let expense = Expense {
            id: 1,
            amount: 20,
            category: Some(String::from("subscriptions")),
            description: String::from("Bought data plan from glo"),
        };

        let mut ex = Expenses::new();

        ex.add_expense(expense);

        let message = ex.delete_expense(1);

        assert_eq!(Some("Deleted successfullya"), message);
    }

    #[test]
    fn it_should_list_expenses() {
        let mut ex = Expenses::new();
        assert_eq!(ex.expenses.len(), 0);

        let expense = Expense {
            id: 1,
            amount: 20,
            category: Some(String::from("subscriptions")),
            description: String::from("Bought data plan from glo"),
        };

        ex.add_expense(expense);

        assert_eq!(ex.expenses.len(), 1);
    }
    #[test]
    fn it_should_update_expense() {}
}
