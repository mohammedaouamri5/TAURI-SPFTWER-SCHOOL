 






use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub id_type: i32,
    pub name: String,
    pub family_name: String,
    pub birth_day: String,
    pub notes: String,
} 

impl User {
    pub fn create() -> &'static str {
        "CREATE TABLE IF NOT EXISTS user 
        (
            id INTEGER PRIMARY KEY AUTOINCREMENT, 
            id_type INTEGER,
            name TEXT, 
            family_name TEXT, 
            birth_day TEXT, 
            notes TEXT,
            FOREIGN KEY (id_type) REFERENCES Level(id) 

        );"
    }
  
}