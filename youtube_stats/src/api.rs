use std::{
    path::PathBuf,
    str::FromStr,
    sync::atomic::{AtomicUsize, Ordering},
};

use anyhow::Ok;
use iso8601;
use serde::Deserialize;

const REQUEST_THRESHOLD: usize = 9980;
const API_ENDPOINT: &str =
    "https://www.googleapis.com/youtube/v3/videos?id={ID}&part=contentDetails&key={KEY}";

pub struct KeyStore {
    pub keys: Vec<String>,
    pub request_threshold: usize,

    index: AtomicUsize,
    requests: AtomicUsize,
}

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

impl KeyStore {
    pub fn from_key(key: &str) -> Self {
        Self {
            keys: vec![key.to_owned()],
            request_threshold: REQUEST_THRESHOLD,

            index: AtomicUsize::new(0),
            requests: AtomicUsize::new(0),
        }
    }

    pub fn from_file(path: PathBuf) -> anyhow::Result<Self> {
        let keys = std::fs::read_to_string(path)?;
        let keys = keys.lines().map(|s| s.to_owned()).collect::<Vec<_>>();
        Ok(Self {
            keys,
            request_threshold: REQUEST_THRESHOLD,

            index: AtomicUsize::new(0),
            requests: AtomicUsize::new(0),
        })
    }

    pub fn key<'a>(&'a self) -> &str {
        let req = self.requests.fetch_add(1, Ordering::SeqCst);
        if req == self.request_threshold {
            self.requests.store(0, Ordering::SeqCst);
            let index = self.index.fetch_add(1, Ordering::SeqCst) + 1;
            println!("[*] Switching to key {}", index);
            return &self.keys[index.min(self.keys.len() - 1)];
        }

        let index = self.index.load(Ordering::SeqCst);
        &self.keys[index.min(self.keys.len() - 1)]
    }

    pub fn verify(&self) -> anyhow::Result<()> {
        for i in self.keys.iter() {
            let res = ureq::get(
                &API_ENDPOINT
                    .replacen("{ID}", "dQw4w9WgXcQ", 1)
                    .replacen("{KEY}", i, 1),
            )
            .call()?;

            if res.status() != 200 {
                return Err(anyhow::anyhow!("Invalid key"));
            }
        }

        Ok(())
    }
}

pub fn video_length(id: &str, key: &KeyStore) -> anyhow::Result<f32> {
    let url = API_ENDPOINT
        .replacen("{ID}", id, 1)
        .replacen("{KEY}", key.key(), 1);

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
