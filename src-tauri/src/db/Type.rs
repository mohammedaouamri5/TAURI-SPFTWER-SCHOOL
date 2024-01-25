 

pub struct Type {
    id:i32, 
    name: String,

}
 
impl Type {

    pub fn create() -> &'static str {
        "CREATE TABLE IF NOT EXISTS Type 
        (
            id INTEGER PRIMARY KEY AUTOINCREMENT , 
            name TEXT 
        );"
    }

    fn new(id: i32, name: &str ) -> Self {
        Type {
            id,
            name: name.to_string(),
        }
    }
    fn insert(&self) -> String {
        format!(
            "INSERT INTO Type (id, name ) VALUES ({}, '{}' );",
            self.id, self.name 
        )
    }
    
}