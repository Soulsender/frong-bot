use poise::serenity_prelude::{CreateEmbed};

// import the Data struct from the main file
use super::super::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


#[poise::command(slash_command, prefix_command)]
pub async fn ask_frong(ctx: Context<'_>) -> Result<(), Error> {
    // Your main logic here
    let embed = CreateEmbed::new().title("This is an embed").description("With a description");

    
    ctx.say("asd").await?;
    Ok(())
}