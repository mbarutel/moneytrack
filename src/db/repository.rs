use crate::models::transaction::Transaction;
use chrono::NaiveDateTime;
use rusqlite::{Connection, Result, params};

pub struct TransactionRepository<'a> {
    conn: &'a Connection,
}

impl<'a> TransactionRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn add(&self, transaction: &Transaction) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO transactions (date, amount, description, category)
                 VALUES (?1, ?2, ?3, ?4)",
            params![
                transaction.date.to_string(),
                transaction.amount,
                transaction.description,
                transaction.category,
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn list(&self) -> Result<Vec<Transaction>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, date, amount, description, category 
                  FROM transactions
                  ORDER BY date DESC",
        )?;

        let transaction = stmt
            .query_map([], |row| {
                Ok(Transaction {
                    id: Some(row.get(0)?),
                    date: NaiveDateTime::parse_from_str(&row.get::<_, String>(1)?, "%Y-%m-%d")
                        .map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                1,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?,
                    amount: row.get(2)?,
                    description: row.get(3)?,
                    category: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(transaction)
    }
}
