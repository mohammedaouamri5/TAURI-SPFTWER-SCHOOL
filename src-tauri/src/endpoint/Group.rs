#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{ connection::get_db, db::{ self, Group::Groupe } };

#[tauri::command]
pub fn get_all_the_actives_groups() -> Vec<db::Group::Groupe> {
    let conn = get_db().lock().unwrap(); 


    let query = "SELECT * FROM Groupe WHERE is_done = FALSE;";

    let mut stmt = conn.prepare(query).expect("Failed to prepare statement");
    let result: Vec<Groupe> = stmt
        .query_map([], |row| {
            Ok(Groupe {
                id: row.get(0).expect("Failed to get id "),
                idfrom: row.get(1).expect("Failed to get idfrom "),
                idto: row.get(2).expect("Failed to get idto "),
                idteacher: row.get(3).expect("Failed to get idteacher "),
                date_start: row.get(4).expect("Failed to get date_start "),
                date_end: row.get(5).expect("Failed to get date_end "),
                name: row.get(6).expect("Failed to get name"),
                is_done: row.get(7).expect("Failed to get is_done "),
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<Groupe>, rusqlite::Error>>()
        .expect("Failed to collect results");

    result
}

#[tauri::command]
pub fn get_all_the_groups() -> Vec<Groupe> {
    let conn = get_db().lock().unwrap(); 


    let query = "SELECT * FROM Groupe;";

    let mut stmt = conn.prepare(query).expect("Failed to prepare statement");
    let result: Vec<Groupe> = stmt
        .query_map([], |row| {
            Ok(Groupe {
                id: row.get(0).expect("Failed to get id "),
                idfrom: row.get(1).expect("Failed to get idfrom "),
                idto: row.get(2).expect("Failed to get idto "),
                idteacher: row.get(3).expect("Failed to get idteacher "),
                date_start: row.get(4).expect("Failed to get date_start "),
                date_end: row.get(5).expect("Failed to get date_end "),
                name: row.get(6).expect("Failed to get name"),
                is_done: row.get(7).expect("Failed to get is_done "),
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<Groupe>, rusqlite::Error>>()
        .expect("Failed to collect results");

    result
}

#[tauri::command]
pub fn the_groups_of_teacher(idteacher: i32) -> Vec<Groupe> {
    let conn = get_db().lock().unwrap(); 


    let ref query = format!("SELECT * FROM Groupe WHERE idteacher = {};", idteacher);

    let mut stmt = conn.prepare(query).expect("Failed to prepare statement");
    let result: Vec<Groupe> = stmt
        .query_map([], |row| {
            Ok(Groupe {
                id: row.get(0).expect("Failed to get id "),
                idfrom: row.get(1).expect("Failed to get idfrom "),
                idto: row.get(2).expect("Failed to get idto "),
                idteacher: row.get(3).expect("Failed to get idteacher "),
                date_start: row.get(4).expect("Failed to get date_start "),
                date_end: row.get(5).expect("Failed to get date_end "),
                name: row.get(6).expect("Failed to get name"),
                is_done: row.get(7).expect("Failed to get is_done "),
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<Groupe>, rusqlite::Error>>()
        .expect("Failed to collect results");

    result
}

#[tauri::command]
pub fn the_active_groups_of_teacher(idteacher: i32) -> Vec<Groupe> {
    let conn = get_db().lock().unwrap(); 


    let ref query =
        format!("SELECT * FROM Groupe WHERE idteacher = {} AND is_done = FALSE;", idteacher);

    let mut stmt = conn.prepare(query).expect("Failed to prepare statement");
    let result: Vec<Groupe> = stmt
        .query_map([], |row| {
            Ok(Groupe {
                id: row.get(0).expect("Failed to get id "),
                idfrom: row.get(1).expect("Failed to get idfrom "),
                idto: row.get(2).expect("Failed to get idto "),
                idteacher: row.get(3).expect("Failed to get idteacher "),
                date_start: row.get(4).expect("Failed to get date_start "),
                date_end: row.get(5).expect("Failed to get date_end "),
                name: row.get(6).expect("Failed to get name"),
                is_done: row.get(7).expect("Failed to get is_done "),
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<Groupe>, rusqlite::Error>>()
        .expect("Failed to collect results");

    result
}

#[tauri::command]
pub fn the_active_groups_of_student(idstudent: i32) -> Vec<Groupe> {
    let conn = get_db().lock().unwrap(); 


    let ref query =
        format!("
                SELECT Groupe.*
                FROM Groupe
                JOIN GroupeUser ON Groupe.id = GroupeUser.id_groupe
                WHERE GroupeUser.id_user = {}
                AND Groupe.is_done = FALSE ;
                ", idstudent);

    let mut stmt = conn.prepare(query).expect("Failed to prepare statement");
    let result: Vec<Groupe> = stmt
        .query_map([], |row| {
            Ok(Groupe {
                id: row.get(0).expect("Failed to get id "),
                idfrom: row.get(1).expect("Failed to get idfrom "),
                idto: row.get(2).expect("Failed to get idto "),
                idteacher: row.get(3).expect("Failed to get idteacher "),
                date_start: row.get(4).expect("Failed to get date_start "),
                date_end: row.get(5).expect("Failed to get date_end "),
                name: row.get(6).expect("Failed to get name"),
                is_done: row.get(7).expect("Failed to get is_done "),
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<Groupe>, rusqlite::Error>>()
        .expect("Failed to collect results");

    result
}

#[tauri::command]
pub fn the_groups_of_student(idstudent: i32) -> Vec<Groupe> {
    let conn = get_db().lock().unwrap(); 


    let ref query =
        format!("
                SELECT Groupe.*
                FROM Groupe
                JOIN GroupeUser ON Groupe.id = GroupeUser.id_groupe
                WHERE GroupeUser.id_user = {}
                ;
                ", idstudent);

    let mut stmt = conn.prepare(query).expect("Failed to prepare statement");
    let result: Vec<Groupe> = stmt
        .query_map([], |row| {
            Ok(Groupe {
                id: row.get(0).expect("Failed to get id "),
                idfrom: row.get(1).expect("Failed to get idfrom "),
                idto: row.get(2).expect("Failed to get idto "),
                idteacher: row.get(3).expect("Failed to get idteacher "),
                date_start: row.get(4).expect("Failed to get date_start "),
                date_end: row.get(5).expect("Failed to get date_end "),
                name: row.get(6).expect("Failed to get name"),
                is_done: row.get(7).expect("Failed to get is_done "),
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<Groupe>, rusqlite::Error>>()
        .expect("Failed to collect results");

    result
}

#[tauri::command]
pub fn create_groupe(
    idfrom: i32,
    idto: i32,
    idteacher: i32,
    date_start: &str, 
    name: &str
) -> Result<String, String> {
    let conn = get_db().lock().unwrap(); 


    let query = format!(
        "INSERT INTO Groupe 
        (idfrom,   idto,   idteacher,    date_start,    name , is_done) 
        VALUES
        ({idfrom},{idto}, {idteacher}, '{date_start}','{name}', false);" );

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
pub fn add_student_to_group(idstudent: i32, idgroup: i32, date: &str) -> Result<String, String> {
    let conn = get_db().lock().unwrap(); 


    let query = format!(
        "INSERT INTO GroupeUser (id_groupe , id_user , date_submission) 
        VALUES ({}, {}, '{}');",
        idstudent,
        idgroup,
        date
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
pub fn group_is_done(idgroup: i32, date: &str) -> Result<String, String> {
    let conn = get_db().lock().unwrap(); 


    let query = format!(
        "UPDATE Groupe
        SET date_end = '{}', is_done = true
        WHERE id = {};",
        date,
        idgroup
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
