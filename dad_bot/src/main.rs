use std::env;

use parking_lot::Mutex;
use rusqlite::Connection;
use serenity::prelude::*;

mod bot;
mod commands;
mod database;
mod misc;
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

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents).event_handler(bot).await?;

    client.start().await?;
    Ok(())
}
