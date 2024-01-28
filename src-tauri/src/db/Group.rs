 
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
            FOREIGN KEY (idfrom) REFERENCES Type(id), 
            FOREIGN KEY (idto) REFERENCES Type(id), 
            FOREIGN KEY (idteacher) REFERENCES User(id) 
        );"
    }
    // Constructor for a new Groupe
    pub fn new(id: i32, idfrom: i32, idto: i32, idteacher: i32, date_start: &str, date_end: &str , is_done:bool) -> Self {
        Groupe {
            id,
            idfrom,
            idto,
            idteacher: idteacher ,
            date_start: date_start.to_string() ,
            date_end: date_end.to_string() ,
            is_done: is_done 
        }
    }

    // Constructor for a new Groupe without specifying an ID (for new entries)
    pub fn new_without_id(idfrom: i32, idto: i32, idteacher: i32, date_start: &str, date_end: &str, is_done:bool) -> Self {
        Groupe {
            id: 0, // 0 or any default value since it's AUTOINCREMENT
            idfrom,
            idto,
            idteacher: idteacher ,
            date_start: date_start.to_string(),
            date_end: date_end.to_string(),
            is_done: is_done 
        }
    }

    // Get the SQL insertion string for a Groupe
    pub fn insert(&self) -> String {
        format!(
            "INSERT INTO Groupe (idfrom, idto, idteacher, date_start, date_end) VALUES ({}, {}, '{}', '{}', '{}');",
            self.idfrom, self.idto, self.idteacher, self.date_start, self.date_end
        )
    }

    // Get the SQL selection string for all Groupes
    fn select_all() -> &'static str {
        "SELECT id, idfrom, idto, idteacher, date_start, date_end FROM Groupe;"
    }
} 