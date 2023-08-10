use anyhow::{Context, Ok, Result};

mod boat_info;
use boat_info::BoatInfo;
use serde::Deserialize;
use serde_json::Value;

const SEARCH_PATH: &str = "https://www.boatinfoworld.com/getsearchresults.asp";
const STATE_CODES: &[&str] = &[
    "AL", "KY", "OH", "AK", "LA", "OK", "AZ", "ME", "OR", "AR", "MD", "PA", "AS", "MA", "PR", "CA",
    "MI", "RI", "CO", "MN", "SC", "CT", "MS", "SD", "DC", "MT", "TX", "DE", "MO", "TN", "FL", "NE",
    "TT", "GA", "NV", "UT", "GU", "NH", "VT", "HI", "NJ", "VA", "ID", "NM", "VI", "IL", "NY", "WA",
    "IN", "NC", "WV", "IA", "ND", "WI", "KS", "MP", "WY",
];

fn main() {
    dbg!(download_state("NJ"));
}

fn download_state(state: &str) -> Result<Vec<BoatInfo>> {
    let init = raw_download(state, 0)?;
    let all = raw_download(state, init.records_total)?;
    let out = all.data.into_iter().map(BoatInfo::from_raw).collect();

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

    let res = ureq::post(SEARCH_PATH)
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_string(&body)?;

    Ok(res.into_json()?)
}
