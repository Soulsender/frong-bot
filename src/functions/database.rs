
use rusqlite::*;
use log::*;
use serenity::futures::stream::iter;

use crate::functions::database;

// import the Data struct from the main file
use super::super::{Error, Context};

const DB_NAME: &str = "frong.db";
struct User {
    username: String,
    discord_id: String,
    frongs: i32
}

#[poise::command(slash_command, prefix_command)]
pub async fn db_debug(ctx: Context<'_>) -> Result<(), Error> {
    
    get_entire_db();
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

pub fn get_entire_db() {
    let database = rusqlite::Connection::open(DB_NAME).unwrap();
    let mut end_string = String::new();
    let mut query = database.prepare("SELECT sql_user_id, username, discord_user_id, frongs FROM frong_count").unwrap();    

    let user_iter = query.query_map([], |row| {
        Ok(User {
            username: row.get(1)?,
            discord_id: row.get(2)?,
            frongs: row.get(3)?,
        })
    }).unwrap();

    for user in user_iter {
        let user = user.unwrap();
        end_string.push_str(&user.username);
    }

    println!("{}", end_string)
    // get amount of rows in db

    // for every row in db, append username to total string
    
}
