use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GroupUserInfo {
    pub id_group: i32,
    pub id_user: i32,
    pub name: String,
}

#[tauri::command]
pub fn get_groups_of_users(id: Vec<i32>) -> Vec<GroupUserInfo> {
    let conn: rusqlite::Connection = rusqlite::Connection
        ::open("db.sqlite")
        .expect("Failed to open database connection");

    // Generate placeholders for the IN clause based on the number of id_users
    let placeholders: String = (0..id.len())
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(", ");

    // Execute the query using the generated placeholders
    let query =
        format!("SELECT Groupe.id AS id_group, 
                        GroupeUser.id_user, 
                        Groupe.name AS name 
                FROM Groupe
                JOIN GroupeUser ON Groupe.id = GroupeUser.id_groupe
                WHERE Groupe.is_done = 0 AND
                      GroupeUser.id_user IN ({}) ; ", placeholders);

    let mut stmt = conn.prepare(&query).expect("Failed to prepare statement");
    let result: Vec<GroupUserInfo> = stmt
        .query_map(
            id
                .iter()
                .map(|x| x as &dyn rusqlite::ToSql)
                .collect::<Vec<_>>()
                .as_slice(),
            |row| {
                Ok(GroupUserInfo {
                     id_group: row.get(0).expect("Failed to get id_group "),
                     id_user: row.get(1).expect("Failed to get id_user "),
                     name: row.get(2).expect("Failed to get name "),
                })
            }
        )
        .expect("Failed to execute query")
        .collect::<Result<Vec<GroupUserInfo>, rusqlite::Error>>()
        .expect("Failed to collect results");

    result
}
