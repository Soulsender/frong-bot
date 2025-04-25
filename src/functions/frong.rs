use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;
use log::*;

// import the Data struct from the main file
use super::super::{Error, Context};

#[poise::command(slash_command, prefix_command)]
pub async fn frongs(ctx: Context<'_>) -> Result<(), Error> {
    let embed = CreateEmbed::new().title("List all Frong definitions").description("/frong \n /frang \n /frongincidence \n /frongonianunits \n /unfuckwithable");
    let message: CreateReply = CreateReply {
        embeds: vec![embed],
        ..Default::default()
    };

    ctx.send(message).await?;
    trace!("frongs() called");
    Ok(())
}

// original frong definition
#[poise::command(slash_command, prefix_command)]
pub async fn frong(ctx: Context<'_>) -> Result<(), Error> {
    let embed = CreateEmbed::new().title("Frong").description("Frong (for real, on God) is an expression used in the Cosmodium Cyber Security server for humorous means. It is most commonly utilised in a fashion of agreement about a given subject. Frong (for real, on God) is a derivative of “Fr” (for real) and “ong” (on God). It is not recognized as a legitimate word in the 2022 Oxford English Dictionary but is utilized nonetheless as a cultural reference of mutual agreement. This expression was first coined by Soulsender and CØ$MØ where the two individuals were saying “Fr Fr ong” (for real, for real, on God) as a method of agreement, where after the two expressions were merged to form the new commonly used phrase. Since the debut, the phrase has found use in a server emoji of a small mammal dubbed a “gerbil” with the text “frong” (for real, on God) displayed on the bottom of the image in 2013 type-2 impact font.");
    let message: CreateReply = CreateReply {
        embeds: vec![embed],
        ..Default::default()
    };

    ctx.send(message).await?;
    trace!("ask_frong() called");
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn frongincidence(ctx: Context<'_>) -> Result<(), Error> {
    let embed = CreateEmbed::new().title("Frongincidence").description("Frongincidence (for real, on God, coincidence) is most commonly utilized in a fashion of suspicious coincidence (dubbed \"sus\") about a given subject. Frongincidence (for real, on God, coincidence) is a derivative of “Fr” (for real) and “ong” (on God), as well as the word \"coincidence.\" It is not recognized as a legitimate word in the 2022 Oxford English Dictionary but is utilized nonetheless as a cultural reference of suscoincidence. This expression was first coined by CØ$MØ where he was referencing an aspect of the media platform discord, and how it pertained to the word \"frong\" (for real, on God) as is the emoji letters were in a sequential fashion, where after the two expressions were merged to form the new commonly used phrase; Frongincidence (for real, on God, coincidence). Since its debut, the phrase has found use in a server emoji of a small mammal dubbed a “gerbil” with the text “frong” (for real, on God), with the additional context of something being of suspicious coincidence (\"sus\").");
    let message: CreateReply = CreateReply {
        embeds: vec![embed],
        ..Default::default()
    };

    ctx.send(message).await?;
    trace!("frongincidence() called");
    Ok(())
}


#[poise::command(slash_command, prefix_command)]
pub async fn frang(ctx: Context<'_>) -> Result<(), Error> {
    let embed = CreateEmbed::new().title("Frang").description("Frang (for real, on God, language) otherwise known as \"Fronlang\" is a programming language used to develop programs like FrongOS (for real, on God, operating system) as well as Google and PowerFrong. It is Comterpreted language, meaning that the user must compile it and then run the compiled binary through an interpreter. The langauge is not object oriented, because fuck that. Frang is well recognized as a programming language in the computer science community and has even received an oscar for how well developed it is. The original developers, CØ$MØ and Haze, chose the name \"Frang\" or \"Fronglang\" due to its associaton of the word \"frong\" (for real, on God).");
    let message: CreateReply = CreateReply {
        embeds: vec![embed],
        ..Default::default()
    };

    ctx.send(message).await?;
    trace!("frang() called");
    Ok(())
}


#[poise::command(slash_command, prefix_command)]
pub async fn frongonianunits(ctx: Context<'_>) -> Result<(), Error> {
    let embed = CreateEmbed::new().title("Frongonian Units").description("Think about it like this, you have 182 watermelons and each cost 6 quid (Or 7 euros for my European folks. And 7 dollars for freedom lovers). in order to convert into frongonian units, you must take the squared derivative of each seed and calculate the megahertz it generates after a 15 ton, 31-year-old mother, has sat on it. from there you can calculate the amount of politeness each person interacts with the mother. the nicer, the more Canadian. convert it back to micro coulombs squared and you will get it in light year");
    let message: CreateReply = CreateReply {
        embeds: vec![embed],
        ..Default::default()
    };

    ctx.send(message).await?;
    trace!("frongonianunits() called");
    Ok(())
}


#[poise::command(slash_command, prefix_command)]
pub async fn unfuckwithable(ctx: Context<'_>) -> Result<(), Error> {
    let embed = CreateEmbed::new().title("Unfuckwithable").description("Unfuckwithable, an old term dating back to the early-late 1900s (roughly 2 years after the CosmodiumCS debut). the term is used to describe any noun (person, place, creature, thing, or idea) that is incapable of being \"fucked with\". this term represents an older technological way of representing words via concatenative abbreviation, where the words are laid back-to-back to make a new word (of course inspired by gay seckz). this is dissimilar to words like \"frong\" (\"for real\", \"on God\") which are composed of two separate acronyms.");
    let message: CreateReply = CreateReply {
        embeds: vec![embed],
        ..Default::default()
    };

    ctx.send(message).await?;
    trace!("unfuckwithable() called");
    Ok(())
}
