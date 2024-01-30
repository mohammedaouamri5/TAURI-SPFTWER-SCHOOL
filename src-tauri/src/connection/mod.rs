// connection/mod.rs

use lazy_static::lazy_static;
use rusqlite::Connection;
use std::sync::Mutex;

pub struct DbManager {
    pub conn: Connection,
}

impl DbManager {
    pub fn new(db_path: &str) -> Self {
        let conn = Connection::open(db_path).expect("Failed to open database connection");
        DbManager { conn }
    }

    pub fn run(&self, query: &str) {
        self.conn.execute(query, []).expect("Failed to execute query");
    }
}

// Singleton instance using lazy_static and Mutex
lazy_static! {
    static ref DB_MANAGER: Mutex<DbManager> =
        Mutex::new(DbManager::new("./db.sqlite"));
}

pub fn get_db() -> &'static Mutex<DbManager> {
    &DB_MANAGER
}