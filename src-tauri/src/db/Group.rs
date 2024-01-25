 

pub struct Groupe {
    id: i32,             // Primary key
    idfrom: i32,         // Foreign key referencing id in Type table
    idto: i32,           // Foreign key referencing id in Type table
    idteacher: String,   // Foreign key referencing id in User table
    date_start: String,
    date_end: String,
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
            FOREIGN KEY (idfrom) REFERENCES Type(id), 
            FOREIGN KEY (idto) REFERENCES Type(id), 
            FOREIGN KEY (idteacher) REFERENCES User(id) 
        );"
    }
    // Constructor for a new Groupe
    fn new(id: i32, idfrom: i32, idto: i32, idteacher: &str, date_start: &str, date_end: &str) -> Self {
        Groupe {
            id,
            idfrom,
            idto,
            idteacher: idteacher.to_string(),
            date_start: date_start.to_string(),
            date_end: date_end.to_string(),
        }
    }

    // Constructor for a new Groupe without specifying an ID (for new entries)
    fn new_without_id(idfrom: i32, idto: i32, idteacher: &str, date_start: &str, date_end: &str) -> Self {
        Groupe {
            id: 0, // 0 or any default value since it's AUTOINCREMENT
            idfrom,
            idto,
            idteacher: idteacher.to_string(),
            date_start: date_start.to_string(),
            date_end: date_end.to_string(),
        }
    }

    // Get the SQL insertion string for a Groupe
    fn insert(&self) -> String {
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