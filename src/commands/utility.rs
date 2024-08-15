use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::model::Timestamp;
use serenity::prelude::*;
use reqwest;

#[command]
#[description = "Get the status of the website."]
#[usage = "<url>"]
#[example = "https://www.google.com"]
#[min_args(1)]
#[max_args(1)]
#[bucket = "complicated"]
#[aliases("status")]
async fn ping(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = args.single::<String>()?;
    let client = reqwest::Client::new();
    let res = client.get(&url).send().await.unwrap();
    let status = res.status();
    let msg_embed = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Status");
                e.description(format!("Status: {}", status));
                e.timestamp(Timestamp::now());
                // red: 0xff0000
                // green: 0x00ff00
                e.color(0x00ff00)
            });
            m
        })
        .await;
    // msg.channel_id.say(&ctx.http, format!("Status: {}", status)).await?;
    if let Err(why) = msg_embed {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}