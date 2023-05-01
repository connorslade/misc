use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;

use crate::commands;
use crate::misc::dadable;

pub struct Bot;

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
                eprintln!("Error handling command: {}", why);
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
