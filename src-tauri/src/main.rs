// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
  
use std::process::id;

use tauri::{State, Manager, AppHandle};
use rusqlite::{ffi::Error, Connection, Result};


mod connection; 
mod db;
mod endpoint;
 
#[tauri::command]
fn bruh() -> Result<i32, String> {
    if 5 == 5 {
        Ok(23)
    } else {
        Err(String::from("6546"))
    }
}

#[tauri::command]
fn num() -> Vec<db::Type::Type> {
    let conn: Connection = Connection::open("db.sqlite").expect("Failed to open database connection");

    let query = "SELECT id, name FROM Type";

    let mut stmt = conn.prepare(query).expect("Failed to prepare statement");
    let result: Vec<db::Type::Type> = stmt.query_map([], |row| {
        Ok(db::Type::Type {
            id: row.get(0).expect("Failed to get id"),
            name: row.get(1).expect("Failed to get name"),
        })
    })
    .expect("Failed to execute query")
    .collect::<Result<Vec<db::Type::Type>>>()
    .expect("Failed to collect results");

    result
}


fn main() {

    let db = connection::DB_MANAGER.lock().unwrap();

    db.run( db::user::User::create() ) ; 
    db.run( db::GroupUser::GroupeUser::create() ) ; 
    db.run( db::Group::Groupe::create() ) ; 
    db.run( db::Level::Level::create() ) ; 
    db.run( db::Type::Type::create() ) ; 
    db.run( db::Type::Type::create() ) ; 

    tauri::Builder::default()
    
       
    .invoke_handler(tauri::generate_handler![
         num,
         bruh,
         endpoint::Group::get_all_the_actives_groups, 
         endpoint::Group::get_all_the_groups,
         endpoint::Group::the_groups_of_teacher, 
         endpoint::Group::the_active_groups_of_teacher, 
        //  endpoint::Group::get_all_the_actives_groups, 
        ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}
 