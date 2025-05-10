use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    id: u32,
    description: String,
    amount: u32,
    category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Expenses {
    expenses: Vec<Expense>,
}

impl Expenses {
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
}
