use super::*;

#[group]
#[checks(is_owner)]
#[commands(message, status)]
pub struct Owner;

// Checks if author of messaage is bot owner
#[check]
async fn is_owner(
    _: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    log::debug!("Checking if is owner");

    if let Ok(owner) = dotenv::var("OWNER_ID") {
        let owner = owner.parse::<u64>().unwrap_or(0);
        if owner != *msg.author.id.as_u64() {
            Err(Reason::UserAndLog {
                user: String::from("Permission denied"),
                log: String::from("User is not owner"),
            })
        } else {
            Ok(())
        }
    } else {
        Err(Reason::UserAndLog {
            user: String::from("Permission denied"),
            log: String::from("Failed to retrieve owner id"),
        })
    }
}

// Makes bot send message to a certain channel
#[command]
async fn message(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    log::info!("Received message command");

    if let Ok(id) = args.single::<u64>() {
        if !args.rest().is_empty() {
            let message = args.rest();

            // Try to send message to a channel
            let channel_id = ChannelId::from(id);

            if channel_id.say(ctx, message).await.is_ok() {
                msg.channel_id.say(ctx, "Message sent to channel").await?;
            } else {
                // Try to send message to a user
                match UserId::from(id).create_dm_channel(ctx).await {
                    Err(err) => {
                        log::error!("Failed to send message: {:?}", err);
                        msg.reply(ctx, "Invalid id").await?;
                    }
                    Ok(channel) => {
                        if let Err(err) = channel.say(ctx, message).await {
                            log::error!("Failed to send message: {:?}", err);
                            msg.reply(ctx, "Failed to send the message").await?;
                        } else {
                            msg.channel_id.say(ctx, "Message sent to user").await?;
                        }
                    }
                };
            }
        } else {
            log::error!("No message provided");
            msg.reply(ctx, "No message provided").await?;
        }
    } else {
        log::error!("Invalid id");
        msg.reply(ctx, "Invalid id").await?;
    }

    Ok(())
}

// Changes the bot's status
#[command]
async fn status(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    log::info!("Received status command");

    if let Ok(activity) = args.single::<String>() {
        if !args.rest().is_empty() {
            let status = args.rest();

            let activity = match activity.as_str() {
                "playing" => Activity::playing(status),
                "listening" => Activity::listening(status),
                "competing" => Activity::competing(status),
                _ => {
                    log::error!("Invalid activity");
                    msg.reply(&ctx, "Invalid activity. Reverting to default")
                        .await?;
                    get_default_activity()
                }
            };

            ctx.set_activity(activity).await;
            log::debug!("Status set");
            msg.channel_id.say(&ctx, "Status set").await?;
        } else {
            log::error!("No status provided");
            msg.reply(&ctx, "No status provided").await?;
        }
    } else {
        log::error!("No activity provided");
        msg.reply(&ctx, "No activity provided").await?;
    }

    Ok(())
}
