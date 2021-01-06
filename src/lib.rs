use serenity::model::gateway::Activity;

pub fn get_default_activity() -> Activity {
    Activity::playing("with 🦀")
}

