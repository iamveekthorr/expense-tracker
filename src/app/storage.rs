use std::{
    fs::{self, File, OpenOptions},
    io::{BufReader, Error},
    path::Path,
};

use csv::Writer;
use serde::{Deserialize, Serialize};

pub fn save_to_file<T>(path: &str, data: &T) -> Result<(), Error>
where
    T: Serialize,
{
    let json = serde_json::to_string_pretty(data)?;

    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, json)?;
    Ok(())
}

/// This is necessary when using serde_json::from_reader, which deserializes from a borrowed source (like a file). Rust needs to know that T can live long enough to receive borrowed data
pub fn load_from_file<T>(path: &str) -> std::io::Result<T>
where
    T: for<'de> Deserialize<'de>, // make use of lifetime
{
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let reader = BufReader::new(file);

    let expenses = serde_json::from_reader(reader)?;
    Ok(expenses)
}

pub fn export_as_csv<I, T>(path: &str, data: I)
where
    I: IntoIterator<Item = T>,
    T: Serialize,
{
    // Create file if it doesn't exist
    let file = match File::create(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to create CSV file: {err}");
            return;
        }
    };

    // write data into file
    let mut writer = Writer::from_writer(file);

    for item in data {
        if let Err(err) = writer.serialize(item) {
            eprintln!("Failed to write item to CSV: {err}");
        }
    }

    if let Err(err) = writer.flush() {
        eprintln!("Failed to flush CSV writer: {err}");
    } else {
        println!("Data exported to {path}");
    }
}
