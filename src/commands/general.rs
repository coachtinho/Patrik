use super::*;

#[group]
#[commands(ping, say, avatar)]
pub struct General;
 
#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    log::info!("Received ping from {}#{}", msg.author.name, msg.author.discriminator);

    if let Err(err) = msg.reply(ctx, "Pong!").await {
        log::error!("Failed responding to ping: {:?}", err);
    }

    Ok(())
}

// Makes bot say a certain phrase
#[command]
async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    log::info!("Received say from {}#{}", msg.author.name, msg.author.discriminator);

    let phrase = if args.rest().is_empty() {
        "No phrase provided" 
    } else {
        args.rest()
    };

    if let Err(err) = msg.channel_id.say(ctx, phrase).await {
        log::error!("Failed saying phrase: {:?}", err);
    }

    Ok(())
}

// Retrieves url of user's avatar
#[command]
async fn avatar(ctx: &Context, msg: &Message) -> CommandResult {
    log::info!("Received avatar from {}#{}", msg.author.name, msg.author.discriminator);

    let avatar = msg.author.avatar_url().unwrap_or(msg.author.default_avatar_url());

    if let Err(err) = msg.channel_id.say(ctx, avatar).await {
        log::error!("Failed getting avatar: {:?}", err);
    }

    Ok(())
}
