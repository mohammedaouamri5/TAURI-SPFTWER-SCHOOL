use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct GroupeUser {
    pub id_groupe: i32, // id_groupe foreign key
    pub id_user: i32, // id_user foreign key
    pub date_submission: String,
}

impl GroupeUser {
    pub fn create() -> &'static str {
        "CREATE TABLE IF NOT EXISTS GroupeUser 
        (
            id_groupe INTEGER,
            id_user INTEGER,
            date_submission TEXT,
            FOREIGN KEY (id_groupe) REFERENCES Groupe(id),
            FOREIGN KEY (id_user) REFERENCES User(id)
        );"
    } 
}
