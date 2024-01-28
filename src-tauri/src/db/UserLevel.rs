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
}
