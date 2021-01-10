use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::id::{ ChannelId, UserId };
use serenity::model::gateway::Activity;
use serenity::framework::standard::{
    CommandResult,
    Args,
    CommandOptions,
    Reason,
    macros::{
        command,
        group,
        check,
    },
};
use log;
use patrik::*;
use select::document::Document;
use select::predicate::{ Predicate, Class, Not };

// Command groups must be added here
pub mod general;
pub mod owner;

// Code common to all command groups should be added here

// Futbin helper functions
// Get's player fut price from futbin
pub async fn player_price(query: String) -> Result<String, String> {
    let mut prices = String::new();
    let url = format!("{}{}", "https://www.futbin.com/21/players?page=1&search=", query.as_str());

    match reqwest::get(&url).await {
        Ok(body) => match body.text().await {
            Ok(res) => {
                let doc = Document::from(res.as_str());
                let players = doc
                    .find(Class("player_tr_1").or(Class("player_tr_2")))
                    .into_selection();

                for player in players.iter() {
                    let name = player
                        .find(Class("player_name_players_table"))
                        .into_selection()
                        .first()
                        .unwrap()
                        .text();

                    let  rating = player
                        .find(Class("rating").and(Not(Class("player_img"))))
                        .into_selection()
                        .first()
                        .unwrap()
                        .text();

                    let version = player
                        .find(Class("mobile-hide-table-col"))
                        .into_selection()
                        .first()
                        .unwrap()
                        .text();

                    let price = player
                        .find(Class("ps4_color").and(Class("font-weight-bold")))
                        .into_selection()
                        .first()
                        .unwrap()
                        .text();

                    prices.push_str(&format!("{} {} {}: {}\n", name, rating, version, price));
                }

                Ok(prices)
            },

            Err(err) => Err(err.to_string())
        },

        Err(err) => Err(err.to_string())
    }
}
