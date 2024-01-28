


use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct Groupe {
    pub id: i32,             // Primary key
    pub idfrom: i32,         // Foreign key referencing id in Type table
    pub idto: i32,           // Foreign key referencing id in Type table
    pub idteacher: i32,   // Foreign key referencing id in User table
    pub date_start: String,
    pub date_end: String,
    pub is_done:bool,
}
 

impl Groupe {
    
    pub fn create() -> &'static str {
        "CREATE TABLE IF NOT EXISTS Groupe 
        ( 
            id INTEGER PRIMARY KEY AUTOINCREMENT, 
            idfrom INTEGER, 
            idto INTEGER, 
            idteacher INTEGER , 
            date_start TEXT, 
            date_end TEXT, 
            is_done bool, 
            FOREIGN KEY (idfrom) REFERENCES Level(id), 
            FOREIGN KEY (idto) REFERENCES Level(id), 
            FOREIGN KEY (idteacher) REFERENCES User(id) 
        );"
    }
} 