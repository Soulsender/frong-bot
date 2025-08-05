use poise::CreateReply;
use rusqlite::*;
use log::*;
use poise::serenity_prelude::{CreateEmbed};

// import the Data struct from the main file
use super::super::{Error, Context};

const DB_NAME: &str = "./data/frong.db";
struct User {
    username: String,
    discord_id: String,
    frongs: i32
}

#[poise::command(slash_command, prefix_command)]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let embed = CreateEmbed::new().title(":crown: Leaderboard")
        .field(":speech_balloon: Top Frongers", user_frongs(get_entire_db()), false)
        .field(":loudspeaker: Total Frongs", format!("{}", total_frongs(get_entire_db())), false);
    let message: CreateReply = CreateReply {
        embeds: vec![embed],
        ..Default::default()
    };

    ctx.send(message).await?;
    trace!("ask_frong() called");
    Ok(())
}

pub fn increment_user_db(user_id: i64, user: String) {
    let database = rusqlite::Connection::open(DB_NAME).unwrap();

    database.execute(
        "INSERT INTO frong_count (username, discord_user_id, frongs)
         VALUES (?1, ?2, 1)
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

fn get_entire_db() -> Vec<User> {
    let database = rusqlite::Connection::open(DB_NAME).unwrap();
    let mut query = database.prepare("SELECT sql_user_id, username, discord_user_id, frongs FROM frong_count").unwrap();
    let mut user_list = vec![];  
    let user_iter = query.query_map([], |row| {
        Ok(User {
            username: row.get(1)?,
            discord_id: row.get(2)?,
            frongs: row.get(3)?,
        })
    }).unwrap();

    // iterates over every user struct returned by the sql query
    for user in user_iter {
        let user = user.unwrap();
        user_list.push(user);
    }
    
    user_list
}

fn user_frongs(mut user_list: Vec<User>) -> String {
    let mut end_string = String::new();

    // sort in descending order
    user_list.sort_by(|a, b| b.frongs.cmp(&a.frongs));

    // only iterate 10 times, because we only need the first 10 users
    for user in user_list.iter().take(10) {
        end_string.push_str(format!("**{}:** {}\n", user.username, user.frongs).as_str());
    };

    end_string
}

fn total_frongs(user_list: Vec<User>) -> i32 {
    let total_frongs: i32 = user_list.iter().map(|user| user.frongs).sum();
    total_frongs
}
