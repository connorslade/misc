use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;

mod commands;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, _ready: gateway::Ready) {
        Command::create_global_application_command(&ctx.http, |command| {
            commands::top_dads::register(command)
        })
        .await
        .unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "top-dads" => commands::top_dads::run(&command.data.options),
                _ => "Command not found!?".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Error handling command: {}", why);
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || !dadable(&msg.content) {
            return;
        }

        msg.react(&ctx.http, '🇱')
            .await
            .expect("Error reacting to message");
        msg.reply(&ctx.http, "im dad")
            .await
            .expect("Error sending message");
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Bot)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

fn dadable(msg: &str) -> bool {
    const DAD_TRIGGER: &[&str] = &["im", "i am", "i'm", "i’m", "i’m"];
    const DAD_ANTI_TRIGGER: &[&str] = &["(shut)", "(stfu)", "(no)"];

    let msg = msg.to_lowercase();
    DAD_TRIGGER.iter().any(|&x| msg.contains(x))
        && !DAD_ANTI_TRIGGER.iter().any(|&x| msg.contains(x))
}
