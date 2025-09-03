use functions::{ask_frong, leaderboard, frong, googlethat};
use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use std::{path::Path, sync::Arc};
use serenity::*;
use log::*;
use rand::Rng;

mod functions;

struct Data {} // user data
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
const DEBUG_PREFIX: &str = "~";



// define debug slash command register
// this will only be available during runtime if the DEV=true env variable is set
#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    warn!("Debug register command called");
    Ok(())
}

// code to handle events (ie. a message was sent)
struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: serenity::Context, msg: Message) {
        trace!("Message retrieved: {:?}", msg);

        // detect if a user says "frong"
        if !msg.author.bot && msg.content.to_ascii_lowercase().contains("frong") {
            // attempt to load image
            let attachment_result = CreateAttachment::path("frong.jpg").await;
            match attachment_result {
                Ok(attachment) => {
                    let builder = CreateMessage::new()
                        .content("frong")
                        .add_file(attachment);
                    
                    if let Err(err) = msg.channel_id.send_message(&ctx.http, builder).await {
                        error!("Error replying frong: {:?}", err);
                    }
                }
                Err(err) => {
                    error!("Failed to create attachment: {:?}", err);
                    if msg.channel_id.say(&ctx.http, "frong\n-# There was an error loading the image. oops.").await.is_err() {
                        error!("Error sending fallback message");
                    }
                }
            }
            let id = msg.author.id.into();
            let user = msg.author.name;
            leaderboard::increment_user_db(id, user);
        }


        // detect if the phrase "i use arch btw" is said
        if !msg.author.bot && msg.content.to_ascii_lowercase().contains("i use arch btw") {
            let num = rand::rng().random_range(0..2);
            if num == 0 {
                // attempt to load image
                let attachment_result = CreateAttachment::path("arch_form.jpg").await;
                match attachment_result {
                    Ok(attachment) => {
                        let builder = CreateMessage::new()
                            .add_file(attachment);
                        
                        if let Err(err) = msg.channel_id.send_message(&ctx.http, builder).await {
                            error!("Error replying frong: {:?}", err);
                        }
                    }
                    Err(err) => {
                        error!("Failed to create attachment: {:?}", err);
                        if msg.channel_id.say(&ctx.http, "-# There was an error loading the image. oops.").await.is_err() {
                            error!("Error sending fallback message");
                        }
                    }
                }
            } else {
                let responses: Vec<&str> = vec![
                    "Oh you use arch? Why don’t you `sudo pacman -S some-bitches`.", 
                    "https://tenor.com/view/arch-linux-linux-open-source-arch-i-use-arch-btw-gif-25315741",
                    "https://tenor.com/view/arch-linux-i-use-arch-lonely-gif-26341678",
                    "https://tenor.com/view/me-looking-for-who-asked-looking-for-who-asked-who-asked-me-looking-gif-20318322",
                ];
                let num = rand::rng().random_range(0..responses.len());
                let response = responses[num];
                let builder = CreateMessage::new()
                            .content(response);
                if msg.channel_id.send_message(&ctx.http, builder).await.is_err() {
                            error!("Error sending fallback message");
                };          
            }
        }
        
    }
}


#[tokio::main]
async fn main() {
    // by default only log output from our code
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("frong_bot_rust=info")).init();
    info!("Compiling bot options...");
    
    // load env vars from file if it exists
    if Path::new(".env").exists() {
        dotenv().unwrap();
    }

    // set token variable
    // by default will look for a session env  before the value in the .env file
    let token = std::env::var("TOKEN").unwrap_or_else(|err| {
        error!("Token failed to be retrieved: {}", err);
        panic!()
    });
    info!("Token retrieved successfully!");
    
    // set bot intents and activity
    let intents = serenity::GatewayIntents::all();
    let activity = ActivityData { 
        name: ("calls to frongation".to_string()), 
        kind: (poise::serenity_prelude::ActivityType::Listening), 
        state: (None), 
        url: (None) 
    };

    // all the regular commands, excluding developer commands
    let mut commands = vec![
        ask_frong::ask_frong(),
        frong::frong(),
        frong::frongincidence(),
        frong::frang(),
        frong::frongonianunits(),
        frong::unfuckwithable(),
        frong::frongs(),
        googlethat::googlethat(),
        leaderboard::leaderboard(),
    ];

    // the commands if developer mode is enabled with "DEV=true" in the .env
    if std::env::var("DEV").unwrap_or_else(|_| {
        "false".to_string()
    }) == "true" {
        commands.push(register());
        warn!("Running with developer enviroment enabled!");
    }

    // setup pose framework
    // this includes slash commands and guild(s)
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // list of slash commands
            commands,
            // prefix command for debug
            // used to easily register commands
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(DEBUG_PREFIX.into()),
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
    info!("Client built successfully!");

    // initialize database
    leaderboard::create_db();

    // create bot client
    let client = serenity::ClientBuilder::new(token, intents).event_handler(Handler).framework(framework).activity(activity).await;

    // start the client
    info!("Client Started!");
    client.unwrap().start().await.unwrap();
}
