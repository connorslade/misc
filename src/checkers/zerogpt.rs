use serde::Deserialize;
use serde_json::json;
use ureq;

use super::Checker;

pub struct ZeroGPT;

#[derive(Deserialize)]
struct Response {
    data: ResponseData,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseData {
    /// From 0-100
    fake_percentage: f32,
}

impl Checker for ZeroGPT {
    fn name(&self) -> &'static str {
        "ZeroGPT"
    }

    fn check(&self, text: &str) -> anyhow::Result<f32> {
        let res = ureq::post("https://api.zerogpt.com/api/detect/detectText")
            .set(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/118.0",
            )
            .set("Origin", "https://www.zerogpt.com")
            .send_json(json!({ "input_text": text }))?
            .into_json::<Response>()?;

        Ok(res.data.fake_percentage / 100.0)
    }

    fn min(&self) -> u32 {
        0
    }

    fn max(&self) -> u32 {
        15_000
    }
}
