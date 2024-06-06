
use std::{fs, path::Path};

use crate::dal;
use rusqlite::Connection;

const TABLES: &str = include_str!("../../sql/tables.sql");

pub mod custom_emote;
pub mod settings;
pub mod chat_log;
pub mod user_character_messages;
pub mod log;
pub mod swtor_message;
pub mod migration;

pub fn get_connection() -> Connection {

    let em_dirs = dal::get_em_dirs();
    let db_path = em_dirs.get_data_dir_path("blinky.db");
    Connection::open(db_path).unwrap()

}

pub fn init() {

    let conn = get_connection();
    conn.execute_batch(TABLES).expect("Error creating tables");

}