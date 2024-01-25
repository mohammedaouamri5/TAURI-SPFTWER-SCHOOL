
pub struct Pyment {
    idUser: i32,   // idUser foreign key
    howmuch: f32,
    date_payment: String,
}


impl Pyment {
    
    
    
    pub fn create() -> &'static str {
        "CREATE TABLE IF NOT EXISTS Pyment (
            idUser INTEGER,
            howmuch REAL,
            date_payment TEXT,
            FOREIGN KEY (idUser) REFERENCES User(id)
        );"
    }
    
    fn new(idUser: i32, howmuch: f32, date_payment: &str) -> Self {
        Pyment {
            idUser,
            howmuch,
            date_payment: date_payment.to_string(),
        }
    }

    // Constructor for a new Pyment without specifying an ID (for new entries)
    fn new_without_id(idUser: i32, howmuch: f32, date_payment: &str) -> Self {
        Pyment {
            idUser,
            howmuch,
            date_payment: date_payment.to_string(),
        }
    }

    // Get the SQL insertion string for a Pyment
    fn insert(&self) -> String {
        format!(
            "INSERT INTO Pyment (idUser, howmuch, date_payment) VALUES ({}, {}, '{}');",
            self.idUser, self.howmuch, self.date_payment
        )
    }

    // Get the SQL selection string for all Pyments
    fn select_all() -> &'static str {
        "SELECT idUser, howmuch, date_payment FROM Pyment;"
    }
}