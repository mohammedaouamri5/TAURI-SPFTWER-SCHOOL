#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{ connection::{self, get_db}, db::user::{self, User} };

#[tauri::command]
pub fn get_all_active_the_users() -> Vec<User> {
    let conn = get_db().lock().unwrap(); 


    let ref query = format!("
    WITH NotDoneGroups AS (
        SELECT id
        FROM Groupe
        WHERE is_done = 0
    )
    SELECT U.*
    FROM GroupeUser GU
    JOIN User U ON GU.id_user = U.id
    JOIN NotDoneGroups NDG ON GU.id_groupe = NDG.id;
    "
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
    let conn = get_db().lock().unwrap(); 


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
    let conn = get_db().lock().unwrap(); 


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
pub fn get_all_the_users_by_group(group_id: i32) -> Vec<User> {
    let conn = get_db().lock().unwrap(); 


    let ref query = format!("
    SELECT user.id, user.id_type ,  user.name, user.family_name, user.birth_day, user.notes
    FROM GroupeUser
    JOIN Groupe ON GroupeUser.id_groupe = Groupe.id
    JOIN user ON GroupeUser.id_user = user.id
    WHERE Groupe.id =  {group_id};");

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

// #[tauri::command]
// pub fn get_level_of_user(id: i32) -> {
    
// }
#[tauri::command]
pub fn get_user_by_id(id: i32) -> User {
    let conn = get_db().lock().unwrap(); 


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
    let conn = get_db().lock().unwrap(); 


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

#[tauri::command]
pub fn get_users_by_ids(id: Vec<i32>) -> Vec<User> {
    let conn = get_db().lock().unwrap(); 


    // Generate placeholders for the IN clause based on the number of id_users
    let placeholders: String = (0..id.len()).map(|_| "?").collect::<Vec<_>>().join(", ");

    // Execute the query using the generated placeholders
    let query = format!(
        "SELECT * FROM User WHERE id IN  ({}); ",
        placeholders
    );

    let mut stmt = conn.prepare(&query).expect("Failed to prepare statement");
    let result: Vec<User> = stmt
.query_map(id.iter().map(|x| x as &dyn rusqlite::ToSql).collect::<Vec<_>>().as_slice(), |row| {
            Ok(User {
                id: row.get(0).expect("Failed to get id" ),
                id_type: row.get(1).expect("Failed to get id_type" ),
                name: row.get(2).expect("Failed to get name" ),
                family_name: row.get(3).expect("Failed to get family_name" ),
                birth_day: row.get(4).expect("Failed to get birth_day" ),
                notes: row.get(5).expect("Failed to get notes" ),
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<User>, rusqlite::Error>>()
        .expect("Failed to collect results");

    result
}