use crate::models::transaction::Transaction;
use chrono::NaiveDate;
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;

pub fn import_from_csv(file_path: &str) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut transactions = Vec::new();

    for result in reader.records() {
        let record = result?;

        // Access by index: record[0], record[1], etc
        let date = NaiveDate::parse_from_str(&record[0], "%d/%m/%Y")
            .or_else(|_| NaiveDate::parse_from_str(&record[0], "%d/%m/%Y"))?;
        let amount = record[1].trim_matches('"').parse()?;
        let description = record[2].to_string();

        transactions.push(Transaction::new(
            date,
            amount,
            String::from("Miscellaneous"),
            description,
        ));
    }

    Ok(transactions)
}
