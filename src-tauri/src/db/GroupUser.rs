pub struct GroupeUser {
    idGroupe: i32, // idGroupe foreign key
    idUser: i32, // idUser foreign key
    date_submission: String,
}

impl GroupeUser {
    pub fn create() -> &'static str {
        "CREATE TABLE IF NOT EXISTS GroupeUser 
        (
            idGroupe INTEGER,
            idUser INTEGER,
            date_submission TEXT,
            FOREIGN KEY (idGroupe) REFERENCES Groupe(id),
            FOREIGN KEY (idUser) REFERENCES User(id)
        );"
    }
    fn new(idGroupe: i32, idUser: i32, date_submission: &str) -> Self {
        GroupeUser {
            idGroupe,
            idUser,
            date_submission: date_submission.to_string(),
        }
    }

    // Get the SQL insertion string for a GroupeUser
    fn insert(&self) -> String {
        format!(
            "INSERT INTO GroupeUser (idGroupe, idUser, date_submission) VALUES ({}, {}, '{}');",
            self.idGroupe,
            self.idUser,
            self.date_submission
        )
    }

    // Get the SQL selection string for all GroupeUsers
    fn select_all() -> &'static str {
        "SELECT idGroupe, idUser, date_submission FROM GroupeUser;"
    }
}
