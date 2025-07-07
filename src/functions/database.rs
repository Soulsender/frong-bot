
use rusqlite::*;
use log::*;

use crate::functions::database;

// import the Data struct from the main file
use super::super::{Error, Context};

const DB_NAME: &str = "frong.db";
struct User {
    sql_id: i32,
    username: String,
    discord_id: i32,
    frongs: i32
}

#[poise::command(slash_command, prefix_command)]
pub async fn db_debug(ctx: Context<'_>) -> Result<(), Error> {
    get_db();
    Ok(())
}

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

pub fn get_db() {
    let database = rusqlite::Connection::open(DB_NAME).unwrap();
    debug!("{}", database.execute(
        "SELECT * FROM frong_count WHERE sql_user_id = {i}",
        []
    ).unwrap_or(1));
    let i = 1;
    let mut q = database.prepare("SELECT sql_user_id username discord_user_id frongs FROM frong_count WHERE sql_user_id = {i}").unwrap();
    let iter = q.query_map(&[(":sql_user_id", sql_user_id.to_string().as_str())], |row| {
        Ok(User {
            sql_id: i,
            username: row.get(1)?,
            discord_id: row.get(2)?,
            frongs: row.get(3)?,
         })
    });
    for user in iter {
        println!("Found person {:?}", user);
    }
}
