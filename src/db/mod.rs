pub mod connection;
pub mod migrations;
pub mod repository;

pub use connection::Database;
pub use repository::TransactionRepository;

use rusqlite::Result;

// Initialize daTabase and run migrations
pub fn init() -> Result<Database> {
    let db = Database::new()?;
    migrations::run_migrations(db.conn())?;
    Ok(db)
}
