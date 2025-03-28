use functions::ask_frong;
use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use std::sync::Arc;

mod functions;

pub struct Data {} // user data
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// define debug slash command register
#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // get token from .env file
    dotenv::from_path(".env").expect("[!] No path found to .env");
    dotenv().expect("[!] Error loading .env file");

    // set token variable
    let token = std::env::var("TOKEN").expect("[!] No Bot Token Provided");

    // set bot intents
    let intents = serenity::GatewayIntents::all();

    // setup pose framework
    // this includes slash commands and guild(s)
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // list of slash commands
            commands: vec![
                register(),
                ask_frong::ask_frong()
            ],
            // prefix command for debug
            // used to easily register commands
                prefix_options: poise::PrefixFrameworkOptions {
                    prefix: Some("~debug".into()),
                    edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(std::time::Duration::from_secs(3600)))),
                    case_insensitive_commands: true,
                    ..Default::default()
                },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, Into::into(860417200426188820)).await?;
                Ok(Data {})
            })
        })
        .build();

    // create bot client
    let client = serenity::ClientBuilder::new(token, intents).framework(framework).await;

    // start the client
    client.unwrap().start().await.unwrap();
}
