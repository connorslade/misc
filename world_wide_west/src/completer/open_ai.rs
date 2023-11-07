#![allow(dead_code)]

use afire::Request;
use anyhow::Context;
use serde::Deserialize;
use serde_json::json;

use super::{Completer, Completion};

// PROOMPT
const PROMPT: &str = "Create a response to the following HTTP request. Always add lots of specific detailed information, boilerplate or placeholder information is never acceptable. In the case that the response is in HTML also add relative links to other relevant pages (all internal links) but avoid referencing css or javascript, lets keep these pages very 90s. In the case that you want to add css, it must be with inline styles of a style tag. In your response the first line must be an HTTP content type then a line break, the lines after that will be the document body. Add as much detail as possible.
{{METHOD}} {{PATH}}

---
Content-Type:";

pub struct OpenAI {
    key: String,
    model: String,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Deserialize, Debug)]
struct CompletionResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<CompletionResponseChoice>,
    usage: CompletionResponseUsage,
}

#[derive(Deserialize, Debug)]
struct CompletionResponseUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

#[derive(Deserialize, Debug)]
struct CompletionResponseChoice {
    index: u64,
    message: CompletionResponseChoiceMessage,
    finish_reason: String,
}

#[derive(Deserialize, Debug)]
struct CompletionResponseChoiceMessage {
    role: String,
    content: String,
}

impl OpenAI {
    pub fn new(key: impl AsRef<str>) -> Self {
        Self {
            key: key.as_ref().to_owned(),
            model: "gpt-4-1106-preview".to_owned(),
            // model: "gpt-3.5-turbo-16k".to_owned(),
            temperature: 1.0,
            max_tokens: 4096,
            // max_tokens: 16_000,
        }
    }
}

impl Completer for OpenAI {
    fn complete(&self, req: &Request) -> anyhow::Result<Completion> {
        let prompt = PROMPT
            .replacen("{{METHOD}}", req.method.to_string().as_str(), 1)
            .replacen("{{PATH}}", &req.path, 1);

        let res = ureq::post("https://api.openai.com/v1/chat/completions")
            .set("Authorization", &format!("Bearer {}", self.key))
            .send_json(json!(
                {
                    "messages": [
                        {
                            "role": "system",
                            "content": prompt,
                        }
                    ],
                    "model": self.model,
                    "temperature": self.temperature,
                    "max_tokens": self.max_tokens,
                    "top_p": 1,
                    "frequency_penalty": 0,
                    "presence_penalty": 0
                }
            ))
            .unwrap()
            .into_json::<CompletionResponse>()
            .unwrap();

        let text = &res.choices[0].message.content;
        let (content, value) = text.split_once('\n').context("Single line completion")?;
        Ok(Completion {
            content_type: content.trim().to_owned(),
            body: value.as_bytes().to_vec(),
            tokens: res.usage.completion_tokens,
        })
    }
}
