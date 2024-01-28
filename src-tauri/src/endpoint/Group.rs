#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{ State, Manager, AppHandle };
use rusqlite::{ Connection, Result };

use crate::{ connection, db::{ self, Group::Groupe } };


#[tauri::command]
pub fn get_all_the_actives_groups() -> Vec<db::Group::Groupe> {
    let conn: Connection = Connection::open("db.sqlite").expect("Failed to open database connection");

    
    let query = "SELECT * FROM Groupe WHERE is_done = FALSE;";

    let mut stmt = conn.prepare(query).expect("Failed to prepare statement");
    let result: Vec<db::Group::Groupe> = stmt
        .query_map([], |row| {
            Ok(db::Group::Groupe {
                id: row.get(0).expect("Failed to get id "),
                idfrom: row.get(1).expect("Failed to get idfrom "),
                idto: row.get(2).expect("Failed to get idto "),
                idteacher: row.get(3).expect("Failed to get idteacher "),
                date_start: row.get(4).expect("Failed to get date_start "),
                date_end: row.get(5).expect("Failed to get date_end "),
                is_done: row.get(6).expect("Failed to get is_done "),
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<db::Group::Groupe>>>()
        .expect("Failed to collect results");

    result
}

#[tauri::command]
pub fn get_all_the_groups() -> Vec<Groupe> {
    let conn = connection::DB_MANAGER.lock().unwrap() ;

    
    let query = "SELECT * FROM Groupe;";

    let mut stmt = conn.conn.prepare(query).expect("Failed to prepare statement");
    let result: Vec<Groupe> = stmt
        .query_map([], |row| {
            Ok(Groupe {
                id: row.get(0).expect("Failed to get id "),
                idfrom: row.get(1).expect("Failed to get idfrom "),
                idto: row.get(2).expect("Failed to get idto "),
                idteacher: row.get(3).expect("Failed to get idteacher "),
                date_start: row.get(4).expect("Failed to get date_start "),
                date_end: row.get(5).expect("Failed to get date_end "),
                is_done: row.get(6).expect("Failed to get is_done "),
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<Groupe>>>()
        .expect("Failed to collect results");

    result
}

#[tauri::command]
pub fn the_groups_of_teacher(idteacher:i32) -> Vec<Groupe> {
    let conn = connection::DB_MANAGER.lock().unwrap() ;

     let ref query = format!("SELECT * FROM Groupe WHERE idteacher = {};" , idteacher );

    let mut stmt = conn.conn.prepare(query).expect("Failed to prepare statement");
    let result: Vec<Groupe> = stmt
        .query_map([], |row| {
            Ok(Groupe {
                id: row.get(0).expect("Failed to get id "),
                idfrom: row.get(1).expect("Failed to get idfrom "),
                idto: row.get(2).expect("Failed to get idto "),
                idteacher: row.get(3).expect("Failed to get idteacher "),
                date_start: row.get(4).expect("Failed to get date_start "),
                date_end: row.get(5).expect("Failed to get date_end "),
                is_done: row.get(6).expect("Failed to get is_done "),
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<Groupe>>>()
        .expect("Failed to collect results");

    result
}

#[tauri::command]
pub fn the_active_groups_of_teacher(idteacher:i32) -> Vec<Groupe> {
    let conn = connection::DB_MANAGER.lock().unwrap() ;

    let ref query = format!("SELECT * FROM Groupe WHERE idteacher = {} AND is_done = FALSE;" , idteacher );
    
    let mut stmt = conn.conn.prepare(query).expect("Failed to prepare statement");
    let result: Vec<Groupe> = stmt
        .query_map([], |row| {
            Ok(Groupe {
                id: row.get(0).expect("Failed to get id "),
                idfrom: row.get(1).expect("Failed to get idfrom "),
                idto: row.get(2).expect("Failed to get idto "),
                idteacher: row.get(3).expect("Failed to get idteacher "),
                date_start: row.get(4).expect("Failed to get date_start "),
                date_end: row.get(5).expect("Failed to get date_end "),
                is_done: row.get(6).expect("Failed to get is_done "),
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<Groupe>>>()
        .expect("Failed to collect results");

    result
}

