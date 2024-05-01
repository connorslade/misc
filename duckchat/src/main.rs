use std::{
    borrow::Cow,
    io::{self, stdout, BufRead, BufReader, Write},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use ureq::AgentBuilder;

#[derive(Debug, Clone, Serialize)]
struct ChatRequest {
    model: Cow<'static, str>,
    messages: Vec<Message>,
}

#[derive(Debug, Clone, Serialize)]
struct Message {
    role: Cow<'static, str>,
    content: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
struct ChatResponse {
    role: Option<String>,
    message: Option<String>,
    created: u64,
    id: String,
    action: String,
    model: String,
}

fn main() -> Result<()> {
    let agent = AgentBuilder::new().build();

    let token = agent
        .get("https://duckduckgo.com/duckchat/v1/status")
        .set("x-vqd-accept", "1")
        .call()?;
    let mut token = token
        .header("x-vqd-4")
        .context("Token not in response")?
        .to_owned();
    println!("[*] Got token: {token}");

    let mut messages = Vec::new();

    for prompt in io::stdin().lines().filter_map(|x| x.ok()) {
        messages.push(Message {
            role: "user".into(),
            content: prompt,
        });
        let request = ChatRequest {
            model: "gpt-3.5-turbo-0125".into(), // claude-3-haiku-20240307
            messages: messages.clone(),
        };

        let response = agent
            .post("https://duckduckgo.com/duckchat/v1/chat")
            .set("x-vqd-4", &token)
            .set("content-type", "application/json")
            .send_string(&serde_json::to_string(&request)?);

        let response = match response {
            Ok(req) => req,
            Err(err) => {
                let res = err.into_response().unwrap();
                dbg!(&res);
                dbg!(res.into_string().unwrap());
                continue;
            }
        };

        token = response
            .header("x-vqd-4")
            .context("Token not in response")?
            .to_owned();

        let reader = BufReader::new(response.into_reader());
        let lines = reader.lines();
        let mut message = String::new();

        for update in lines.filter_map(|x| x.ok()) {
            if update.len() < 6 {
                continue;
            }

            let response = serde_json::from_str::<ChatResponse>(&update[6..])?;
            if response.message.is_none() {
                break;
            }

            let msg = response.message.unwrap_or_default();
            message.push_str(&msg);

            print!("{msg}");
            stdout().flush()?;
        }
        println!();

        messages.push(Message {
            role: "assistant".into(),
            content: message,
        })
    }

    Ok(())
}
