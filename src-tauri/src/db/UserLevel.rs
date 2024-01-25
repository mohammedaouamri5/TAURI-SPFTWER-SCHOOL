use chrono::{Utc, Datelike};

pub struct UserLevel {
    idUser: i32,
    idLevel: i32,
    date: String,
}
 
impl UserLevel {
    
    pub fn create() -> &'static str {
        "CREATE TABLE IF NOT EXISTS UserLevel 
        (
            idUser INTEGER,
            idLevel INTEGER, 
            theDate DATE DEFAULT CURRENT_DATE, 
            FOREIGN KEY (idUser) REFERENCES User(id), 
            FOREIGN KEY (idLevel) REFERENCES Level(id) 
        );"
    }
    fn new(idUser: i32, idLevel: i32) -> Self {
        let now = Utc::now();
        UserLevel {
            idUser,
            idLevel,
            date: format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day()),
        }
    }

    // Get the SQL insertion string for a user
    fn insert(&self) -> String {
        format!(
            "INSERT INTO UserLevel (idUser,idLevel, theDate ) VALUES ({}, {}, '{}');",
            self.idUser, self.idLevel, self.date
        )
    }
}
