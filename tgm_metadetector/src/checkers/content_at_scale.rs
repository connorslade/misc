use serde::Deserialize;

use super::Checker;

pub struct ContentAtScale;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    score: String,
}

impl Checker for ContentAtScale {
    fn name(&self) -> &'static str {
        "Content At Scale"
    }

    fn check(&self, text: &str) -> anyhow::Result<f32> {
        let res = ureq::post("https://contentatscale.ai/ai-content-detector/")
            .set("Referer", "https://contentatscale.ai/ai-content-detector/")
            .send_form(&[("content", text), ("action", "checkaiscore")])?
            .into_json::<Response>()?;

        Ok((100.0 - res.score.parse::<f32>()?) / 100.0)
    }

    fn min(&self) -> u32 {
        0
    }

    fn max(&self) -> u32 {
        25_000
    }
}
