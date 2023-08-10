use anyhow::{bail, Error, Result};

mod boat_info;
use boat_info::BoatInfo;
use csv::Writer;
use indicatif::ParallelProgressIterator;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use retry::{delay::Exponential, retry};
use serde::Deserialize;

const CHUNK_SIZE: usize = 100;
const SEARCH_PATH: &str = "https://www.boatinfoworld.com/getsearchresults.asp";
const STATE_CODES: &[&str] = &[
    "AL", "KY", "OH", "AK", "LA", "OK", "AZ", "ME", "OR", "AR", "MD", "PA", "AS", "MA", "PR", "CA",
    "MI", "RI", "CO", "MN", "SC", "CT", "MS", "SD", "DC", "MT", "TX", "DE", "MO", "TN", "FL", "NE",
    "TT", "GA", "NV", "UT", "GU", "NH", "VT", "HI", "NJ", "VA", "ID", "NM", "VI", "IL", "NY", "WA",
    "IN", "NC", "WV", "IA", "ND", "WI", "KS", "MP", "WY",
];

fn main() -> Result<()> {
    let counts = STATE_CODES
        .par_iter()
        .progress()
        .map(|x| {
            retry(Exponential::from_millis(10).take(3), || {
                Ok::<_, Error>((x, raw_download(x, 0, 0)?.records_total))
            })
        })
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    println!(
        "Counted {} total boats.",
        counts.iter().fold(0, |acc, x| acc + x.1)
    );

    let jobs = counts
        .into_iter()
        .flat_map(|(&state, total)| {
            (0..=total)
                .collect::<Vec<_>>()
                .chunks(CHUNK_SIZE)
                .map(|x| x.len())
                .enumerate()
                .map(|(i, x)| Job {
                    state: state.to_owned(),
                    start: (i * CHUNK_SIZE) as u64,
                    count: x as u64,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("Split into {} jobs.", jobs.len());

    let data = jobs
        .par_iter()
        .progress()
        .map(|x| {
            retry(Exponential::from_millis(10).take(3), || {
                Ok::<_, Error>(download_state(&x.state, x.start, x.count))
            })
        })
        .filter_map(Result::ok)
        .filter_map(Result::ok)
        .flatten()
        .collect::<Vec<_>>();

    let mut writer = Writer::from_path("out.csv")?;
    for i in data {
        if let Err(e) = writer.serialize(i) {
            eprintln!("Error serializing info: {e:?}");
        }
    }
    writer.flush()?;

    Ok(())
}

fn download_state(state: &str, start: u64, count: u64) -> Result<Vec<BoatInfo>> {
    let all = match raw_download(state, start, count) {
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

fn raw_download(state: &str, start: u64, count: u64) -> Result<RawDownload> {
    let body = include_str!("../search.txt")
        .replacen("{{STATE}}", state, 1)
        .replacen("{{START}}", &start.to_string(), 1)
        .replacen("{{COUNT}}", &count.to_string(), 1);

    let res = minreq::post(SEARCH_PATH)
        .with_header("Content-Type", "application/x-www-form-urlencoded")
        .with_body(body.as_bytes())
        .send()?;

    Ok(res.json()?)
}

#[derive(Debug)]
struct Job {
    state: String,
    start: u64,
    count: u64,
}
