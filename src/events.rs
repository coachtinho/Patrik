use log;
use serenity::async_trait;
use serenity::client::{ Context, EventHandler };
use serenity::model::channel::{ Channel, Message };
use serenity::model::gateway::Ready;
use serenity::model::id::{ ChannelId, MessageId };
use patrik::*;

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
                "{}#{} (DM): {}",
                msg.author.name,
                msg.author.discriminator,
                msg.content_safe(&ctx).await
            ),

            Ok(Channel::Guild(channel)) => log::debug!(
                "{}#{} (#{}): {}",
                msg.author.name,
                msg.author.discriminator,
                channel.name,
                msg.content_safe(&ctx).await
            ),

            _ => log::warn!("Unkown message channel")
        }

        if msg.content.contains("hello") {
            log::info!("Message triggered :wave: reaction");

            if let Err(err) = msg.react(&ctx, '👋').await {
                log::error!("Failed adding reaction: {:?}", err);
            }
        }
    }

    // Deleted message event
    async fn message_delete(&self, ctx: Context, channel_id: ChannelId, _: MessageId) {
        match channel_id.to_channel(&ctx).await {
            Err(err) => log::error!("Failed getting deleted message channel: {:?}", err),

            Ok(Channel::Guild(channel)) => {
                log::info!("Message deleted in #{}", channel.name);

                if let Err(err) = channel_id.say(&ctx, "Quem viu viu").await {
                    log::error!("Failed sending message: {:?}", err);
                }
            },
            _ => {}
        } 
    }
}
