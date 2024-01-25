 

use rusqlite::{Connection, Result};




pub struct User {
    id: i32,
    name: String,
    family_name: String,
    birth_day: String,
    notes: String,
}

impl User {
    pub fn create() -> &'static str {
        "CREATE TABLE IF NOT EXISTS user 
        (
            id INTEGER PRIMARY KEY AUTOINCREMENT, 
            name TEXT, 
            family_name TEXT, 
            birth_day TEXT, 
            notes TEXT
        );"
    }
 
 
    // Constructor for a new User
    fn new(id: i32, name: &str, family_name: &str, birth_day: &str, notes: &str) -> Self {
        User {
            id,
            name: name.to_string(),
            family_name: family_name.to_string(),
            birth_day: birth_day.to_string(),
            notes: notes.to_string(),
        }
    }

    // Constructor for a new User without specifying an ID (for new entries)
    fn new_without_id(name: &str, family_name: &str, birth_day: &str, notes: &str) -> Self {
        User {
            id: 0, // 0 or any default value since it's AUTOINCREMENT
            name: name.to_string(),
            family_name: family_name.to_string(),
            birth_day: birth_day.to_string(),
            notes: notes.to_string(),
        }
    }

    // Get the SQL insertion string for a user
    fn insert(&self) -> String {
        format!(
            "INSERT INTO user (name, family_name, birth_day, notes) VALUES ('{}', '{}', '{}', '{}');",
            self.name, self.family_name, self.birth_day, self.notes
        )
    }

    // Get the SQL selection string for all users
    fn select_all() -> &'static str {
        "SELECT id, name, family_name, birth_day, notes FROM user;"
    }
}