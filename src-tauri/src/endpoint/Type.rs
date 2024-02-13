#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{connection::{self, get_db}, db::Type::Type};



#[tauri::command]
pub fn add_type(name:&str) -> Result<String, String> {
    let conn =  get_db().lock().unwrap(); 
 
    let query = format!(
        "INSERT INTO Type 
        (  name )
        VALUES 
        ( '{name}' );" 
    );

    match conn.execute_batch(&query) {
        Ok(_) => Ok(format!("OK")),
        Err(err) => {
            // Handle the error, e.g., print it or return an error result
            eprintln!("Error executing query: {}", err);
            Err(err.to_string())
        }
    }
}
pub fn get_types() ->Vec<Type>  {


    let conn =  get_db().lock().unwrap(); 
    let ref query = format!("SELECT * FROM  type;");

    let mut stmt = conn.prepare(query).expect("Failed to prepare statement");
    let result: Vec<Type> = stmt
        .query_map([], |row| {
            Ok(Type {
                id: row.get(0).expect("Failed to get id "),
                name: row.get(1).expect("Failed to get name "),
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<Type>, rusqlite::Error>>()
        .expect("Failed to collect results");

    result

    

}



#[tauri::command]
pub fn get_type_of_users(id: Vec<i32>) -> Vec<Type> {
    let conn = get_db().lock().unwrap(); 


    // Generate placeholders for the IN clause based on the number of id_users
    let placeholders: String = (0..id.len()).map(|_| "?").collect::<Vec<_>>().join(", ");

    // Execute the query using the generated placeholders
    let query = format!(
        "SELECT * FROM Type WHERE id IN  ({}); ",
        placeholders
    );

    let mut stmt = conn.prepare(&query).expect("Failed to prepare statement");
    let result: Vec<Type> = stmt
.query_map(id.iter().map(|x| x as &dyn rusqlite::ToSql).collect::<Vec<_>>().as_slice(), |row| {
            Ok(Type {
                id: row.get(0).expect("Failed to get id "),
                name: row.get(1).expect("Failed to get name "),
                // Add other fields as needed
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<Type>, rusqlite::Error>>()
        .expect("Failed to collect results");

    result
}