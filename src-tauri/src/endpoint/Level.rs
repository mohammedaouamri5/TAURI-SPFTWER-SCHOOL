use crate::db::{user::User, Level::Level};



use rusqlite::{ Connection, Result };


#[tauri::command]
pub fn get_level_of_users(id: Vec<i32>) -> Vec<Level> {
    let conn: rusqlite::Connection = rusqlite::Connection
        ::open("db.sqlite")
        .expect("Failed to open database connection");

    // Generate placeholders for the IN clause based on the number of id_users
    let placeholders: String = (0..id.len()).map(|_| "?").collect::<Vec<_>>().join(", ");

    // Execute the query using the generated placeholders
    let query = format!(
        "SELECT Level.*
        FROM Level
        JOIN (
            SELECT id_user, id_level, MIN(theDate) AS theDate
            FROM UserLevel
            WHERE id_user IN ({})
            GROUP BY id_user   
        ) AS UserLevelMinDate
        ON Level.id = UserLevelMinDate.id_level;",
        placeholders
    );

    let mut stmt = conn.prepare(&query).expect("Failed to prepare statement");
    let result: Vec<Level> = stmt
.query_map(id.iter().map(|x| x as &dyn rusqlite::ToSql).collect::<Vec<_>>().as_slice(), |row| {
            Ok(Level {
                id: row.get(0).expect("Failed to get id "),
                name: row.get(1).expect("Failed to get name "),
                // Add other fields as needed
            })
        })
        .expect("Failed to execute query")
        .collect::<Result<Vec<Level>, rusqlite::Error>>()
        .expect("Failed to collect results");

    result
}
