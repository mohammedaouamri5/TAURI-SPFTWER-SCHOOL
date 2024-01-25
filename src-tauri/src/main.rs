// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
  
use tauri::{State, Manager, AppHandle};
use tauri_plugin_sql::{ Migration , MigrationKind , Builder  };
use rusqlite::{Connection, Result};


mod connection; 
mod db;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(  name: &str) -> String {
    // Should handle errors instead of unwrapping here
 
    format!("Your name log: {}", name)
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
    
       
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}
 