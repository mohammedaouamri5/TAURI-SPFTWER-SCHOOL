use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct Type {
    pub id:i32, 
    pub name: String,

}
 
impl Type {

    pub fn create() -> &'static str {
        "CREATE TABLE IF NOT EXISTS Type 
        (
            id INTEGER PRIMARY KEY AUTOINCREMENT , 
            name TEXT UNIQUE
        );"
    }
 
    
}