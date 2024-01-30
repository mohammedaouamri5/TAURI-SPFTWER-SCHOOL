#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{ connection, db::user::User };

#[tauri::command]
pub fn get_all_active_the_users() -> Vec<User> {
    let conn: rusqlite::Connection = rusqlite::Connection
        ::open("db.sqlite")
        .expect("Failed to open database connection");

    let ref query = format!(
        "SELECT u.*
    FROM \"user\" u
    LEFT JOIN GroupeUser gu ON u.id = gu.id_user
    LEFT JOIN Groupe g ON gu.id_groupe = g.id
    WHERE g.is_done IS NULL OR g.is_done = false;"
    );

    let mut stmt = conn.prepare(query).expect("Failed to prepare statement");
    let result: Vec<User> = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0).expect("Failed to get id "),
                id_type: row.get(1).expect("Failed to get id_type "),
                name: row.get(2).expect("Failed to get name "),
                family_name: row.get(3).expect("Failed to get family_name "),
                birth_day: row.get(4).expect("Failed to get birth_day "),
                notes: row.get(5).expect("Failed to get notes "),
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<User>, rusqlite::Error>>()
        .expect("Failed to collect results");

    result
}
#[tauri::command]
pub fn get_all_the_users() -> Vec<User> {
    let conn: rusqlite::Connection = rusqlite::Connection
        ::open("db.sqlite")
        .expect("Failed to open database connection");

    let ref query = format!("SELECT * FROM  user;");

    let mut stmt = conn.prepare(query).expect("Failed to prepare statement");
    let result: Vec<User> = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0).expect("Failed to get id "),
                id_type: row.get(1).expect("Failed to get id_type "),
                name: row.get(2).expect("Failed to get name "),
                family_name: row.get(3).expect("Failed to get family_name "),
                birth_day: row.get(4).expect("Failed to get birth_day "),
                notes: row.get(5).expect("Failed to get notes "),
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<User>, rusqlite::Error>>()
        .expect("Failed to collect results");

    result
}

#[tauri::command]
pub fn get_all_the_users_by_type(idtype: i32) -> Vec<User> {
    let conn: rusqlite::Connection = rusqlite::Connection
        ::open("db.sqlite")
        .expect("Failed to open database connection");

    let ref query = format!("SELECT * FROM  user WHERE id_type= {idtype};");

    let mut stmt = conn.prepare(query).expect("Failed to prepare statement");
    let result: Vec<User> = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0).expect("Failed to get id "),
                id_type: row.get(1).expect("Failed to get id_type "),
                name: row.get(2).expect("Failed to get name "),
                family_name: row.get(3).expect("Failed to get family_name "),
                birth_day: row.get(4).expect("Failed to get birth_day "),
                notes: row.get(5).expect("Failed to get notes "),
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<User>, rusqlite::Error>>()
        .expect("Failed to collect results");

    result
}

#[tauri::command]
pub fn get_user_by_id(id: i32) -> User {
    let conn: rusqlite::Connection = rusqlite::Connection
        ::open("db.sqlite")
        .expect("Failed to open database connection");

    let ref query = format!("SELECT * FROM  user WHERE id= {id};");

    let mut stmt = conn.prepare(query).expect("Failed to prepare statement");
    let result: Vec<User> = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0).expect("Failed to get id "),
                id_type: row.get(1).expect("Failed to get id_type "),
                name: row.get(2).expect("Failed to get name "),
                family_name: row.get(3).expect("Failed to get family_name "),
                birth_day: row.get(4).expect("Failed to get birth_day "),
                notes: row.get(5).expect("Failed to get notes "),
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<User>, rusqlite::Error>>()
        .expect("Failed to collect results");

    return result.into_iter().next().expect("REASON");
}

#[tauri::command]
pub fn add_user(
    id_type: i32,
    name: String,
    family_name: String,
    birth_day: String,
    notes: String
) -> Result<String, String> {
    let conn: rusqlite::Connection = rusqlite::Connection
        ::open("db.sqlite")
        .expect("Failed to open database connection");

    let query = format!(
        "INSERT INTO public.user 
        (id_type, name, family_name, birth_day, notes)
        VALUES 
        ({id_type}, '{name}', '{family_name}', '{birth_day}', '{notes}');"
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
