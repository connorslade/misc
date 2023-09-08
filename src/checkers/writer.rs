use serde::Deserialize;

use super::Checker;

pub struct Writer;

#[derive(Deserialize)]
struct Response(Vec<ResponseEntry>);

#[derive(Deserialize)]
struct ResponseEntry {
    score: f32,
}

impl Checker for Writer {
    fn name(&self) -> &'static str {
        "Writer"
    }

    fn check(&self, text: &str) -> anyhow::Result<f32> {
        let res = ureq::post("https://writer.com/wp-admin/admin-ajax.php")
            .send_form(&[
                ("action", "ai_content_detector_recaptcha"),
                ("inputs", text),
                ("token", ""),
            ])?
            .into_json::<Response>()?;

        Ok(res.0[0].score)
    }

    fn min(&self) -> u32 {
        0
    }

    fn max(&self) -> u32 {
        1_500
    }
}
