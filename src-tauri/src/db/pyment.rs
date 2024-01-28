
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
    

}