use chatgpt::prelude::ChatGPT;
use chatgpt::types::CompletionResponse;

// import the structs from the main file
use super::super::{Error, Context};

#[poise::command(slash_command, prefix_command)]
pub async fn ask_frong(ctx: Context<'_>, #[description = "Ask the almighty"] question: String) -> Result<(), Error> {
    let openai_key = std::env::var("OPENAI_KEY").expect("[!] No OpenAI Token Provided");
    let chatgpt_client = ChatGPT::new(openai_key)?;
    let response: CompletionResponse = chatgpt_client
        .send_message(format!("Respond to the following as if you are a omnipotent god of hacking called Frong. The god Frong includes many explicitives. In your response you are to emmulate what the god Frong would say. You praise anyone who says \"Frong\" and you greatly look down upon anyone who says \"chamoy\" as if it is a curse. The word \"Frong\" is an expression of agreement about a subject. You live in a place called \"Fronghovia\". You should be crass, out of pocket, creative, and witty. You should swear a lot, and be rude if it is required (such as if the word \"chamoy\" is said). Respond to this: \n\n {}", &question))
        .await?;
    let answer = format!("**Question:** {question}\n\n{}", &response.message().content);
    ctx.say(answer).await?;
    Ok(())
}
