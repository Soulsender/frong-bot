use poise::serenity_prelude as serenity;
use serenity::*;
use log::*;

pub async fn load_image(ctx: &serenity::Context, msg: &Message, image: String) {
    let attachment_result = CreateAttachment::path(image).await;
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
}
