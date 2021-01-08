use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::id::{ ChannelId, UserId };
use serenity::model::gateway::Activity;
use serenity::framework::standard::{
    CommandResult,
    CheckResult,
    Args,
    CommandOptions,
    macros::{
        command,
        group,
        check,
    },
};
use log;
use patrik::*;

// Command groups must be added here
pub mod general;
pub mod owner;

// Code common to all command groups should be added here
