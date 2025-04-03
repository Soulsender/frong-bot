use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;
use log::*;

// import the Data struct from the main file
use super::super::{Error, Context};

#[poise::command(slash_command, prefix_command)]
pub async fn googlethat(ctx: Context<'_>, query: String) -> Result<(), Error> {
    let query_formatted = query.replace(" ", "+");
    let url = format!("https://letmegooglethat.com/?q={}", query_formatted);

    ctx.say(format!("**{}:** \n{}", query, url)).await?;
    trace!("googlethat() called");
    Ok(())
}
