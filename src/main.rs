mod commands;
mod events;

use env_logger::Env;
use serenity::client::{Client, Context};
use serenity::framework::standard::macros::hook;
use serenity::framework::standard::{DispatchError, Reason, StandardFramework};
use serenity::model::channel::Message;

// Command groups must be imported and added to the framework
use commands::general;
use commands::owner;

use events::Handler;

#[hook]
async fn unrecognised_hook(ctx: &Context, msg: &Message, _: &str) {
    log::info!("Unrecognised command: {}", msg.content_safe(ctx).await);
    msg.reply(ctx, "Invalid command").await.unwrap();
}

// Function to run whenever a command returns an error
#[hook]
async fn error_hook(ctx: &Context, msg: &Message, error: DispatchError) {
    if let DispatchError::CheckFailed(_, Reason::UserAndLog { user, log }) = error {
        msg.reply(ctx, user).await.unwrap();
        log::warn!("Check failed: {}", log);
    }
}

#[tokio::main]
async fn main() {
    let log_level = dotenv::var("LOG_LEVEL").unwrap_or_else(|_| String::from("patrik=info"));
    let prefix = dotenv::var("PREFIX").expect("Failed to get prefix");

    env_logger::Builder::from_env(Env::default().default_filter_or(log_level))
        .format_timestamp(None)
        .init();

    // TODO: Add help function
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(prefix.as_str()))
        .unrecognised_command(unrecognised_hook)
        .on_dispatch_error(error_hook)
        .group(&general::GENERAL_GROUP)
        .group(&owner::OWNER_GROUP);

    log::trace!("Framework created");

    let token = dotenv::var("TOKEN").expect("Failed getting token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    log::trace!("Client created");

    if let Err(why) = client.start().await {
        log::error!("Error ocurred while running the client: {:?}", why);
    }
}
