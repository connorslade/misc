use std::fs::{self, File};

use anyhow::{Context, Error, Result};
use chrono::NaiveDateTime;
use once_cell::sync::Lazy;
use rayon::prelude::{ParallelBridge, ParallelIterator};
use regex::Regex;
use serde::{Deserialize, Deserializer};

const STATS: &str = r"C:\Users\turtl\Downloads\my_spotify_data(2)\MyData";
static FILE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"Streaming_History_Audio_\d{4}(-\d{4})?_\d*.json").unwrap());

fn main() -> Result<()> {
    let data = fs::read_dir(STATS)?
        .into_iter()
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|x| FILE_REGEX.is_match(&x.file_name().to_string_lossy()))
        .map(|x| {
            let raw = File::open(x.path())?;
            let data = serde_json::from_reader::<_, DataFile>(&raw).with_context(|| {
                format!(
                    "Failed to parse file: `{}`",
                    x.file_name().to_string_lossy()
                )
            })?;
            Ok::<_, Error>(data.0)
        })
        .collect::<Result<Vec<_>>>()?;
    let data = data.into_iter().flatten().collect::<Vec<_>>();

    println!("Total entries: {}", data.len());
    println!(
        "Total played: {}ms",
        data.iter().map(|x| x.ms_played).sum::<u64>()
    );

    Ok(())
}

#[derive(Debug, Deserialize)]
struct DataFile(Vec<HistoryEntry>);

#[derive(Debug, Deserialize)]
struct HistoryEntry {
    #[serde(deserialize_with = "naive_date_time_from_str")]
    ts: NaiveDateTime,
    ms_played: u64,
    master_metadata_track_name: Option<String>,
    master_metadata_album_artist_name: Option<String>,
    master_metadata_album_album_name: Option<String>,
    spotify_track_uri: Option<String>,
}

fn naive_date_time_from_str<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%SZ").map_err(serde::de::Error::custom)
}
