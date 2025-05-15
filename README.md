# 📊 Expense Tracker CLI in Rust

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/Status-Active-brightgreen)](https://roadmap.sh/projects/expense-tracker)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A simple, file-based command-line Expense Tracker written in Rust. It allows users to add, update, delete, list, and summarize daily expenses while storing data in a local JSON file.

> This project is based on the [Expense Tracker Challenge](https://roadmap.sh/projects/expense-tracker) on roadmap.sh.

---

## 🧠 Project Overview

Managing your personal or business expenses doesn’t have to be complicated. This CLI application allows you to:

- 📌 Record your expenses with descriptions, categories, and dates.
- 🗂 View a list of all saved expenses.
- 🧮 Get monthly/yearly expense summaries.
- 🗑 Delete expenses by ID.
- ✏️ Update existing records easily.
- 💾 Persist data locally with JSON file storage.

It’s fast, lightweight, and built with Rust for performance and safety.

---

## 📂 Project Structure

```text
expense-tracker/
├── src/
│   ├── app/
│   │   ├── build.rs              # Main CLI logic and command dispatcher
│   │   ├── expenses_definitions.rs  # Data models and expense operations
│   │   └── storage.rs            # JSON file storage utilities
├── data/
│   └── expenses.json             # File where expenses are stored
├── Cargo.toml
└── README.md
```

---

## 🚀 Getting Started

### ✅ Prerequisites

Ensure you have Rust and Cargo installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## 🛠 Installation

### Run Locally (Development)

```bash
git clone https://github.com/your-username/expense-tracker
cd expense-tracker
cargo run -- [command]
```

### Install Globally

You can install the CLI to use it globally:

```bash
cargo install --path .
```

Then run it anywhere with:

```bash
expense-tracker --help
```

---

## 💡 Example Usage

### Add an Expense

```bash
cargo run -- add -d "Glo data plan" -a 2000 -c "subscriptions"
```

### List All Expenses

```bash
cargo run -- list
```

### Update an Expense

```bash
cargo run -- update -i 1 -a 2500 -d "Updated plan" -c "internet"
```

### Delete an Expense

```bash
cargo run -- delete -i 1
```

### Show Monthly Summary

```bash
cargo run -- summary -m 5
```

---

## 🧪 Running Tests

```bash
cargo test
```

The project includes unit tests for each core feature:

- Add
- Delete
- Update
- List
- Summary

---

## 🧾 JSON Data Format

All expenses are stored in `data/expenses.json` using the following structure:

```json
[
  {
    "id": 1,
    "description": "Bought data plan from glo",
    "amount": 2000,
    "category": "subscriptions",
    "date_created": "2025-05-12",
    "date_updated": null
  }
]
```

---

## 📌 Features in Progress

- [ ] Filter by category
- [x] Export to CSV
- [ ] Import expenses
- [ ] Budget warnings

---

## 📚 References

- [Rust CLI Book](https://rust-cli.github.io/book/)
- [Serde for serialization](https://docs.rs/serde/latest/serde/)
- [Clap for CLI parsing](https://docs.rs/clap/latest/clap/)

---

## 🧭 Challenge Source

This project was built as part of the [Expense Tracker Challenge](https://roadmap.sh/projects/expense-tracker) on [roadmap.sh](https://roadmap.sh/), a platform to guide developers through real-world projects.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

## 🙌 Contributing

Contributions are welcome! Please open issues or pull requests to suggest improvements.
