// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{ Connection, Result };

mod connection;
mod db;
mod endpoint;

#[tauri::command]
fn bruh() -> Result<i32, String> {
    if 5 == 5 { Ok(23) } else { Err(String::from("6546")) }
}
use crate::endpoint::{  user::{ * },
                        Group::{ * }, 
                        r#type::{ * }, 
                        Level::{ * },
                        UserGroup::{ * }    };
fn main() {
    let db: rusqlite::Connection = rusqlite::Connection
        ::open("db.sqlite")
        .expect("Failed to open database connection");

    println!("{}\n\n", db::user::User::create());
    println!("{}\n\n", db::GroupUser::GroupeUser::create());
    println!("{}\n\n", db::Group::Groupe::create());
    println!("{}\n\n", db::Level::Level::create());
    println!("{}\n\n", db::Type::Type::create());
    println!("{}\n\n", db::pyment::Pyment::create());
    println!("{}\n\n", db::UserLevel::UserLevel::create());
    
    db.execute_batch(db::user::User::create());
    db.execute_batch(db::GroupUser::GroupeUser::create());
    db.execute_batch(db::Group::Groupe::create());
    db.execute_batch(db::Level::Level::create());
    db.execute_batch(db::Type::Type::create());
    db.execute_batch(db::pyment::Pyment::create());
    db.execute_batch(db::UserLevel::UserLevel::create());
     



    tauri::Builder
        ::default()

        .invoke_handler(
            tauri::generate_handler![
                get_all_the_actives_groups,
                get_all_the_groups,
                the_groups_of_teacher,
                the_active_groups_of_teacher,
                the_active_groups_of_student,
                the_groups_of_student,
                create_groupe,
                add_student_to_group,
                group_is_done,

                get_all_active_the_users,
                get_user_by_id, 
                // get_level_of_user, 
                get_all_the_users,
                get_all_the_users_by_type,
                get_all_the_users_by_group,
                add_user,


                get_level_of_users , 

                get_type_of_users, 


                get_groups_of_users,  

                bruh
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
