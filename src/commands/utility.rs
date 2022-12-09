use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Pong!"]
#[usage = ""]
#[example = ""]
#[aliases("pong")]
#[bucket = "general"]
#[only_in(guilds)]
async fn ping(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}