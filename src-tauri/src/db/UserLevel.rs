use chrono::{Utc, Datelike};

use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct UserLevel {
    pub id_user: i32,
    pub id_level: i32,
    pub date: String,
}
 
impl UserLevel {
    
    pub fn create() -> &'static str {
        "CREATE TABLE IF NOT EXISTS UserLevel 
        (
            id_user INTEGER,
            id_level INTEGER, 
            theDate DATE DEFAULT CURRENT_DATE, 
            FOREIGN KEY (id_user) REFERENCES User(id), 
            FOREIGN KEY (id_level) REFERENCES Level(id) 
        );"
    }
    pub fn new(id_user: i32, id_level: i32) -> Self {
        let now = Utc::now();
        UserLevel {
            id_user,
            id_level,
            date: format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day()),
        }
    }

    // Get the SQL insertion string for a user
    pub fn insert(&self) -> String {
        format!(
            "INSERT INTO UserLevel (id_user,id_level, theDate ) VALUES ({}, {}, '{}');",
            self.id_user, self.id_level, self.date
        )
    }
}
