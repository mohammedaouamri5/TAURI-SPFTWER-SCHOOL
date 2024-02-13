// connection/mod.rs

use lazy_static::lazy_static;
use rusqlite::Connection;
use std::sync::Mutex;
 

// Singleton instance using lazy_static and Mutex
lazy_static! {
     
    pub static ref DB: Mutex<rusqlite::Connection> =  Mutex::new( 
        Connection::open(
            "./db.sqlite"
        ).expect(
            "Failed to open database connection"
        )
    );

}

#[warn(private_interfaces)]
pub fn get_db()-> &'static DB{
     &DB 
}