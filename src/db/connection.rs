use std::path::PathBuf;

use rusqlite::{Connection, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    // Creates/opens database at ~/.moneytrack/transactions.db
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path()?;

        let conn = Connection::open(&db_path)?;

        Ok(Self { conn })
    }

    fn get_db_path() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| rusqlite::Error::InvalidPath("Could not find home directory".into()))?;

        let db_path = home.join(".moneytrack").join("transactions.db");

        // Create the directory if it does not exist
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|err| {
                rusqlite::Error::InvalidPath(format!("Failed to create directory: {}", err).into())
            })?;
        };

        Ok(db_path)
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }
}
