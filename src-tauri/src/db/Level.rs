  
use serde::Serialize;
#[derive(Debug, Serialize , PartialEq, Eq, PartialOrd, Ord)]
pub struct Level {
    pub id: i32,
    pub name: String,
 
}


impl Level {
    
    pub fn create() -> &'static str {
        "CREATE TABLE IF NOT EXISTS Level 
        (
            id INTEGER PRIMARY KEY AUTOINCREMENT , 
            name TEXT  UNIQUE
        );"
    }
    
     
}