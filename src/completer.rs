use afire::Request;

pub struct Completion {
    pub content_type: String,
    pub body: Vec<u8>,
}

pub trait Completer {
    fn complete(&self, req: &Request) -> anyhow::Result<Completion>;
}

pub mod completers {
    use afire::Request;
    use anyhow::Context;
    use serde_json::{json, Value};

    use super::{Completer, Completion};

    const PROMPT: &str = "Create a response to the following http request. HTML is often returned. In the case that the response is in HTML add lots of internal relative links to other relevant pages. It is very important that these links start with a `/`. In your response the first line must be a content type (ex: text/html) then the lines after that will be the body.
{{METHOD}} {{PATH}}

---
Content-Type:";

    pub struct OpenAI {
        key: String,
        model: String,
        temperature: f32,
        max_tokens: u32,
    }

    impl OpenAI {
        pub fn new(key: impl AsRef<str>) -> Self {
            Self {
                key: key.as_ref().to_owned(),
                model: "text-davinci-003".to_owned(),
                temperature: 0.7,
                max_tokens: 512,
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
                .into_json::<Value>()
                .unwrap();
            let text = res
                .get("choices")
                .context("Invalid response")?
                .as_array()
                .unwrap()
                .get(0)
                .unwrap()
                .get("text")
                .unwrap()
                .as_str()
                .unwrap();

            let (content, value) = text.split_once("\n").context("Single like completion")?;
            Ok(Completion {
                content_type: content.to_owned(),
                body: value.as_bytes().to_vec(),
            })
        }
    }
}
