#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{connection, db::Type::Type};



#[tauri::command]
pub fn add_type(name:&str) -> Result<String, String> {
    let conn: rusqlite::Connection = rusqlite::Connection::open("db.sqlite").expect("Failed to open database connection");

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


    let conn: rusqlite::Connection = rusqlite::Connection::open("db.sqlite").expect("Failed to open database connection");

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