use serenity::command::{Args, CommandResult};
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Clear the number of messages specified"]
#[usage = "<number>"]
#[example = "10"]
#[min_args(1)]
#[max_args(1)]
#[required_permissions("MANAGE_MESSAGES")]
#[bucket = "complicated"]
async fn clear(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let number = args.single::<u64>()?;
    let channel = msg.channel_id;
    let messages = channel
        .messages(&ctx.http, |retriever| {
            retriever.before(msg.id).limit(number)
        })
        .await?;
    channel.delete_messages(&ctx.http, messages).await?;
    channel.say(&ctx.http, "Messages deleted").await?;
    channel
        .delete_messages(&ctx.http, vec![msg.id])
        .await
        .expect("Error deleting message");
    Ok(())
}

#[command]
#[description = "Create a channel with the name specified"]
#[usage = "<name>"]
#[example = "test"]
#[min_args(1)]
#[max_args(1)]
#[required_permissions("MANAGE_CHANNELS")]
#[aliases("cc")]
#[bucket = "complicated"]
async fn create_channel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = args.single::<String>()?;
    let guild = msg.guild_id.unwrap();
    let guild = guild.to_partial_guild(&ctx.http).await?;
    if let Err(why) = guild
        .create_channel(&ctx.http, |c| {
            c.name(name)
        })
        .await
    {
        println!("Error creating channel: {:?}", why);
    } else {
        msg.channel_id.say(&ctx.http, "Channel created").await?;
    }


    Ok(())
}

#[command]
#[description = "Delete a channel with the name specified"]
#[usage = "<name>"]
#[example = "test"]
#[min_args(1)]
#[max_args(1)]
#[required_permissions("MANAGE_CHANNELS")]
#[aliases("dc")]
#[bucket = "complicated"]
async fn delete_channel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = args.single::<String>()?;
    let guild = msg.guild_id.unwrap();
    let guild = guild.to_partial_guild(&ctx.http).await?;
    let channels = guild.channels(&ctx.http).await?;
    for channel in channels {
        if channel.1.name == name {
            if let Err(why) = channel.1.delete(&ctx.http).await {
                println!("Error deleting channel: {:?}", why);
            } else {
                msg.channel_id.say(&ctx.http, "Channel deleted").await?;
            }
        }
    }
    Ok(())
}

#[command]
#[description = "Set this channel slowmode time"]
#[usage = "<time>"]
#[example = "10"]
#[min_args(1)]
#[max_args(1)]
#[required_permissions("MANAGE_CHANNELS")]
#[bucket = "complicated"]
async fn slowmode(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let time = args.single::<u64>()?;
    let channel = msg.channel_id;
    if let Err(why) = channel.edit(&ctx.http, |c| c.rate_limit_per_user(time)).await {
        println!("Error setting slowmode: {:?}", why);
    } else {
        msg.channel_id.say(&ctx.http, format!("Slowmode set to {} seconds", time)).await?;
    }
    Ok(())
}