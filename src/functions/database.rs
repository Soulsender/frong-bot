
use rusqlite::*;
use log::*;

// import the Data struct from the main file
use super::super::{Error, Context};

const DB_NAME: &str = "frong.db";

pub fn increment_user_db(user_id: i64, user: String) {
    let database = rusqlite::Connection::open(DB_NAME).unwrap();

    database.execute(
        "INSERT INTO frong_count (username, discord_user_id, frongs)
         VALUES (?1, ?2, 0)
         ON CONFLICT(discord_user_id)
         DO UPDATE SET frongs = frongs + 1",
        params![user, user_id],
    ).unwrap_or(1);

    database.execute(
        "UPDATE frong_count
         SET username = ?1
         WHERE discord_user_id = ?2",
        params![user, user_id],
    ).unwrap_or(1);
}

pub fn create_db() {
    let database = rusqlite::Connection::open(DB_NAME).unwrap();

    database.execute(
        "CREATE TABLE IF NOT EXISTS frong_count (
            sql_user_id       INTEGER PRIMARY KEY AUTOINCREMENT,
            username          TEXT NOT NULL,
            discord_user_id   TEXT NOT NULL UNIQUE,
            frongs            INTEGER NOT NULL DEFAULT 0
        )",
        [],
    ).unwrap_or(1);
}
