use anyhow::{bail, Result};

mod boat_info;
use boat_info::BoatInfo;
use csv::Writer;
use indicatif::ParallelProgressIterator;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde::Deserialize;

const SEARCH_PATH: &str = "https://www.boatinfoworld.com/getsearchresults.asp";
const STATE_CODES: &[&str] = &[
    "AL", "KY", "OH", "AK", "LA", "OK", "AZ", "ME", "OR", "AR", "MD", "PA", "AS", "MA", "PR", "CA",
    "MI", "RI", "CO", "MN", "SC", "CT", "MS", "SD", "DC", "MT", "TX", "DE", "MO", "TN", "FL", "NE",
    "TT", "GA", "NV", "UT", "GU", "NH", "VT", "HI", "NJ", "VA", "ID", "NM", "VI", "IL", "NY", "WA",
    "IN", "NC", "WV", "IA", "ND", "WI", "KS", "MP", "WY",
];

fn main() -> Result<()> {
    let mut writer = Writer::from_path("out.csv")?;
    let data = STATE_CODES
        .par_iter()
        .progress()
        .cloned()
        .map(download_state)
        .filter_map(Result::ok)
        .flatten()
        .collect::<Vec<_>>();

    for i in data {
        if let Err(e) = writer.serialize(i) {
            eprintln!("Error serializing info: {e:?}");
        }
    }
    writer.flush()?;

    Ok(())
}

fn download_state(state: &str) -> Result<Vec<BoatInfo>> {
    let init = match raw_download(state, 0) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error getting {state} size: {e:?}");
            bail!(e);
        }
    };
    let all = match raw_download(state, init.records_total) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error getting {state} records: {e:?}");
            bail!(e);
        }
    };

    let out = all
        .data
        .into_iter()
        .map(|x| BoatInfo::from_raw(state, x))
        .collect();

    Ok(out)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawDownload {
    records_total: u64,
    data: Vec<Vec<String>>,
}

fn raw_download(state: &str, count: u64) -> Result<RawDownload> {
    let body = include_str!("../search.txt")
        .replacen("{{STATE}}", state, 1)
        .replacen("{{COUNT}}", &count.to_string(), 1);

    let res = minreq::post(SEARCH_PATH)
        .with_header("Content-Type", "application/x-www-form-urlencoded")
        .with_body(body.as_bytes())
        .send()?;

    Ok(res.json()?)
}
