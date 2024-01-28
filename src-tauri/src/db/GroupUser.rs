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
    pub fn new(id_groupe: i32, id_user: i32, date_submission: &str) -> Self {
        GroupeUser {
            id_groupe,
            id_user,
            date_submission: date_submission.to_string(),
        }
    }

    // Get the SQL insertion string for a GroupeUser
    pub fn insert(&self) -> String {
        format!(
            "INSERT INTO GroupeUser (id_groupe, id_user, date_submission) VALUES ({}, {}, '{}');",
            self.id_groupe,
            self.id_user,
            self.date_submission
        )
    }

    // Get the SQL selection string for all GroupeUsers
    fn select_all() -> &'static str {
        "SELECT id_groupe, id_user, date_submission FROM GroupeUser;"
    }
}
