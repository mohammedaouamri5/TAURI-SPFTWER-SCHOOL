

pub struct Level {
    id: i32,
    name: String,
 
}


impl Level {
    
    pub fn create() -> &'static str {
        "CREATE TABLE IF NOT EXISTS Level 
        (
            id INTEGER PRIMARY KEY AUTOINCREMENT , 
            name TEXT 
        );"
    }
    
    fn new(id: i32, name: &str ) -> Self {
        Level {
            id,
            name: name.to_string(),
        }
    }

    // Get the SQL insertion string for a user
    fn insert(&self) -> String {
        format!(
            "INSERT INTO users (id, name ) VALUES ({}, '{}' );",
            self.id, self.name 
        )
    }
}