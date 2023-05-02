use std::env;

use parking_lot::Mutex;
use rusqlite::Connection;
use serenity::prelude::*;

mod bot;
mod commands;
mod consts;
mod database;
use database::Database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let token = env::var("DISCORD_TOKEN")?;
    let db_path = env::var("DATABASE_PATH")?;

    let mut connection = Connection::open(db_path)?;
    connection.init()?;
    let bot = bot::Bot {
        connection: Mutex::new(connection),
    };

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;
    let mut client = Client::builder(token, intents).event_handler(bot).await?;

    client.start().await?;
    Ok(())
}
