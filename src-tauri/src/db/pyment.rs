
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct Pyment {
    pub id_user: i32,   // id_user foreign key
    pub howmuch: f32,
    pub date_payment: String,
}


impl Pyment {
    
    
    
    pub fn create() -> &'static str {
        "CREATE TABLE IF NOT EXISTS Pyment (
            id_user INTEGER,
            howmuch REAL,
            date_payment TEXT,
            FOREIGN KEY (id_user) REFERENCES User(id)
        );"
    }
    
    pub fn new(id_user: i32, howmuch: f32, date_payment: &str) -> Self {
        Pyment {
            id_user,
            howmuch,
            date_payment: date_payment.to_string(),
        }
    }

    // Constructor for a new Pyment without specifying an ID (for new entries)
    pub fn new_without_id(id_user: i32, howmuch: f32, date_payment: &str) -> Self {
        Pyment {
            id_user,
            howmuch,
            date_payment: date_payment.to_string(),
        }
    }

    // Get the SQL insertion string for a Pyment
    pub fn insert(&self) -> String {
        format!(
            "INSERT INTO Pyment (id_user, howmuch, date_payment) VALUES ({}, {}, '{}');",
            self.id_user, self.howmuch, self.date_payment
        )
    }

    // Get the SQL selection string for all Pyments
    fn select_all() -> &'static str {
        "SELECT id_user, howmuch, date_payment FROM Pyment;"
    }
}