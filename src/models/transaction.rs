use chrono::NaiveDate;

pub struct Transaction {
    pub id: Option<i64>,
    pub date: NaiveDate,
    pub amount: f64,
    pub category: String,
    pub description: String,
}

impl Transaction {
    pub fn new(date: NaiveDate, amount: f64, category: String, description: String) -> Self {
        Transaction {
            id: None,
            date: date,
            amount: amount,
            category: category,
            description: description,
        }
    }
}
