use poise::serenity_prelude::CreateEmbed;
use serenity::builder::CreateMessage;
use serenity::model::id::ChannelId;


// import the Data struct from the main file
use super::super::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


#[poise::command(slash_command, prefix_command)]
pub async fn frong(ctx: Context<'_>) -> Result<(), Error> {
    // Your main logic here
    let embed = CreateEmbed::new().title("Frong").description("Frong (for real, on God) is an expression used in the Cosmodium Cyber Security server for humorous means. It is most commonly utilised in a fashion of agreement about a given subject. Frong (for real, on God) is a derivative of “Fr” (for real) and “ong” (on God). It is not recognized as a legitimate word in the 2022 Oxford English Dictionary but is utilized nonetheless as a cultural reference of mutual agreement. This expression was first coined by Soulsender and CØ$MØ where the two individuals were saying “Fr Fr ong” (for real, for real, on God) as a method of agreement, where after the two expressions were merged to form the new commonly used phrase. Since the debut, the phrase has found use in a server emoji of a small mammal dubbed a “gerbil” with the text “frong” (for real, on God) displayed on the bottom of the image in 2013 type-2 impact font.");
    let builder = CreateMessage::new().embed(embed);
    
    ctx.send(|m: CreateMessage| {
        *m = builder; // Set the message
    }).await?;
    Ok(())
}