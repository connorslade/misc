#![allow(dead_code)]

use afire::Request;
use anyhow::Context;
use serde::Deserialize;
use serde_json::json;

use super::{Completer, Completion};

const PROMPT: &str = "Create a response to the following HTTP request. Always add lots of specific detailed information. In the case that the response is in HTML also add relative links to other relevant pages (all internal links). In your response the first line must be an HTTP content type then the lines after that will be the body.
{{METHOD}} {{PATH}}

---
Content-Type:";

pub struct OpenAI {
    key: String,
    model: String,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct CompletionResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    usage: CompletionResponseUsage,
    choices: Vec<CompletionResponseChoice>,
}

#[derive(Deserialize)]
struct CompletionResponseUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

#[derive(Deserialize)]
struct CompletionResponseChoice {
    text: String,
    index: u64,
    logprobs: Option<u64>,
    finish_reason: String,
}

impl OpenAI {
    pub fn new(key: impl AsRef<str>) -> Self {
        Self {
            key: key.as_ref().to_owned(),
            model: "text-davinci-003".to_owned(),
            temperature: 0.7,
            max_tokens: 1024,
        }
    }
}

impl Completer for OpenAI {
    fn complete(&self, req: &Request) -> anyhow::Result<Completion> {
        let prompt = PROMPT
            .replacen("{{METHOD}}", req.method.to_string().as_str(), 1)
            .replacen("{{PATH}}", &req.path, 1);

        let res = ureq::post("https://api.openai.com/v1/completions")
            .set("Authorization", &format!("Bearer {}", self.key))
            .send_json(json!(
                {
                    "model": self.model,
                    "prompt": prompt,
                    "temperature": self.temperature,
                    "max_tokens": self.max_tokens
                }
            ))
            .unwrap()
            .into_json::<CompletionResponse>()
            .unwrap();

        let text = &res.choices[0].text;
        let (content, value) = text.split_once('\n').context("Single like completion")?;
        Ok(Completion {
            content_type: content.trim().to_owned(),
            body: value.as_bytes().to_vec(),
            tokens: res.usage.prompt_tokens,
        })
    }
}
