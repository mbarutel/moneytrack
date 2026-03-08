mod cli;
mod db;
mod import;
mod models;

use cli::config::{Command, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem with building the config: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database

    let db = db::init()?;
    let repo = db::TransactionRepository::new(db.conn());

    match config.cmd {
        Command::Summarize => println!("summarize the transactions"),
        Command::Import => {
            // TODO: File path management
            let file_path = "transactions.csv";
            let transactions = import::csv::import_from_csv(file_path)?;
            let count = repo.bulk_insert(transactions)?;
            println!("Imported {} transactions", count)
        }
        Command::List => {
            let transactions = repo.list()?;
            for t in transactions {
                println!(
                    "{} | {:.2} | {} | {}",
                    t.date, t.amount, t.category, t.description
                );
            }
        }
    }

    Ok(())
}
