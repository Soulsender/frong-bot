use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;
use log::*;

// import the Data struct from the main file
use super::super::{Error, Context};

#[poise::command(slash_command, prefix_command)]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let string = String::new();
    let embed = CreateEmbed::new().title("Frong Leaderboard").description(string);
    let message: CreateReply = CreateReply {
        embeds: vec![embed],
        ..Default::default()
    };

    for 

    ctx.send(message).await?;
    trace!("leaderboard() called");
    Ok(())
}