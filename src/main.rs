use poise::serenity_prelude as serenity;
use std::env;
use dotenv::dotenv;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn test(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!("This is a test!");
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::from_path(".env").expect("Failed to load .env");
    dotenv().ok();

    let token = std::env::var("TOKEN").expect("[!] No Bot Token Provided");
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![test()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, Into::into(860417200426188820)).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents).framework(framework).await;

    client.unwrap().start().await.unwrap();
}
