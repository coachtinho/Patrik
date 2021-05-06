use patrik::*;
use rand::Rng;
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::{Channel, Message};
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, GuildId, MessageId};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // On ready event
    async fn ready(&self, ctx: Context, ready: Ready) {
        log::info!("Connected as {}", ready.user.name);

        ctx.set_activity(get_default_activity()).await;
        log::debug!("Set status");
    }

    // Received message event
    async fn message(&self, ctx: Context, msg: Message) {
        match msg.channel_id.to_channel(&ctx).await {
            Err(err) => log::error!("Failed getting message channel: {:?}", err),

            Ok(Channel::Private(_)) => log::debug!(
                "{} (DM): {}",
                msg.author.tag(),
                msg.content_safe(&ctx).await
            ),

            Ok(Channel::Guild(channel)) => log::debug!(
                "{} (#{}): {}",
                msg.author.tag(),
                channel.name,
                msg.content_safe(&ctx).await
            ),

            _ => log::warn!("Unkown message channel"),
        }

        if msg.content.contains("hello") {
            log::info!("Message triggered :wave: reaction");

            if let Err(err) = msg.react(&ctx, '👋').await {
                log::error!("Failed adding reaction: {:?}", err);
            }
        }
    }

    // Deleted message event
    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        _: MessageId,
        _: Option<GuildId>,
    ) {
        match channel_id.to_channel(&ctx).await {
            Err(err) => log::error!("Failed getting deleted message channel: {:?}", err),

            Ok(Channel::Guild(channel)) => {
                log::debug!("Message deleted in #{}", channel.name);
                let random = rand::thread_rng().gen_range(1..=100);
                log::debug!("Random number generated: {}", random);

                if random < 2 {
                    log::info!("Reacting to deleted message in #{}", channel.name);

                    if let Err(err) = channel_id.say(&ctx, "I saw what you deleted").await {
                        log::error!("Failed sending message: {:?}", err);
                    }
                }
            }
            _ => {}
        }
    }
}
