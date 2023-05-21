use std::str::FromStr;

use iso8601;
use serde::Deserialize;

const API_ENDPOINT: &str =
    "https://www.googleapis.com/youtube/v3/videos?id={ID}&part=contentDetails&key={KEY}";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Videos {
    _kind: String,
    _etag: String,
    items: Vec<VideoItem>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VideoItem {
    id: String,
    _kind: String,
    _etag: String,
    content_details: ContentDetails,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContentDetails {
    duration: String,
    _dimension: String,
    _definition: String,
    _caption: String,
    _licensed_content: bool,
}

pub fn video_length(id: &str, key: &str) -> anyhow::Result<f32> {
    let url = API_ENDPOINT
        .replacen("{ID}", id, 1)
        .replacen("{KEY}", key, 1);

    let resp = ureq::get(&url).call()?.into_string()?;
    let json = serde_json::from_str::<Videos>(&resp)?;

    if json.items.is_empty() {
        return Err(anyhow::anyhow!("No video found"));
    }

    if json.items[0].id != id {
        return Err(anyhow::anyhow!("Video ID mismatch"));
    }

    let duration = &json.items[0].content_details.duration;
    let duration =
        iso8601::Duration::from_str(duration).map_err(|_| anyhow::anyhow!("Invalid duration"))?;

    Ok(duration.as_seconds())
}

trait AsSeconds {
    fn as_seconds(&self) -> f32;
}

impl AsSeconds for iso8601::Duration {
    #[rustfmt::skip]
    fn as_seconds(&self) -> f32 {
        match self {
            iso8601::Duration::YMDHMS {
                year,
                month,
                day,
                hour,
                minute,
                second,
                millisecond,
            } => {
                (*millisecond as f32 / 1000.0)
                    + *second as f32
                    + *minute as f32 * 60.0
                    + *hour   as f32 * 60.0 * 60.0
                    + *day    as f32 * 24.0 * 60.0 * 60.0
                    + *month  as f32 * 30.0 * 24.0 * 60.0 * 60.0
                    + *year   as f32 * 365.0 * 24.0 * 60.0 * 60.0
            }
            iso8601::Duration::Weeks(w) => (w * 7 * 24 * 60 * 60) as f32,
        }
    }
}
