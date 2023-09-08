use serde::Deserialize;
use serde_json::json;

use super::Checker;

pub struct Smodin;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    // variance: f32,
    // burstiness: f32,
    // average_perplexity: f32,
    ai_generated_probability: f32,
}

impl Checker for Smodin {
    fn name(&self) -> &'static str {
        "Smodin"
    }

    fn check(&self, text: &str) -> anyhow::Result<f32> {
        let res = ureq::post("https://api.smodin.io/v1/ai-detection/single")
            .send_json(json!({
                "text": text,
                "pageLangSymbol": "en",
                "language": "auto"
            }))?
            .into_json::<Response>()?;

        Ok(res.ai_generated_probability)
    }

    fn min(&self) -> u32 {
        15
    }

    fn max(&self) -> u32 {
        5_000
    }
}
