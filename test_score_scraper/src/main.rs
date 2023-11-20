use std::{collections::HashMap, fs::File, path::Path};

use anyhow::{Context, Result};
use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};

const AUTH_TOKEN: &str = ""; // include_str!("../auth.token")

const OUT_FILE: &str = "data.json";
const DATA_CACHE: &str = "data_cache.bin";
const SCATTERGRAM_CACHE: &str = "scattergram_cache.bin";

const SCATTERGRAM_ADDRESS: &str = "https://blue-ridge-api.naviance.com/college/scattergram";
const APPLICATION_HISTORY_ADDRESS: &str =
    "https://blue-ridge-api.naviance.com/application-statistics/uuid/";

fn main() -> Result<()> {
    let mut scattergram = get_scattergram().context("Getting scattergram")?;
    scattergram.sort_by_key(|college| college.total_applying());
    let all_applying = scattergram
        .iter()
        .map(|college| college.total_applying())
        .sum::<u32>();

    println!("[I] Total student records: {}", all_applying);
    println!("[I] Total colleges: {}", scattergram.len());

    let mut data = get_previous_data().context("Getting previous data")?;
    let bar = ProgressBar::new(scattergram.len() as u64);
    bar.inc(data.len() as u64);
    for college in scattergram.into_iter().rev() {
        if data.contains_key(&college.id) {
            continue;
        }

        let Some(mapping) = &college.core_mapping else {
            continue;
        };

        let stats = match get_college_stats(&mapping.uuid) {
            Ok(stats) => stats,
            Err(err) => {
                println!(
                    "[E] Failed to get stats for {}: {}\nWhile {}",
                    college.name,
                    err,
                    err.chain()
                        .map(|i| i.to_string())
                        .collect::<Vec<_>>()
                        .join("\n  > ")
                );
                break;
            }
        };

        data.insert(college.id.to_owned(), (college.clone(), stats));
        bar.inc(1);
    }

    let cache = File::create(DATA_CACHE)?;
    bincode::serialize_into(cache, &data)?;

    let out = File::create(OUT_FILE)?;
    serde_json::to_writer(out, &data)?;

    Ok(())
}

fn get_previous_data() -> Result<HashMap<String, (College, CollegeStats)>> {
    let cache = Path::new(DATA_CACHE);
    if cache.exists() {
        let cache = File::open(cache)?;
        let data = bincode::deserialize_from(cache)?;
        return Ok(data);
    }

    Ok(HashMap::new())
}

fn get_scattergram() -> Result<Vec<College>> {
    let cache = Path::new(SCATTERGRAM_CACHE);
    if cache.exists() {
        let cache = File::open(cache)?;
        let scattergram = bincode::deserialize_from(cache)?;
        return Ok(scattergram);
    }

    let scattergram = minreq::get(SCATTERGRAM_ADDRESS)
        .with_header("Authorization", AUTH_TOKEN)
        .send()?
        .json::<Vec<College>>()?;

    let cache = File::create(cache)?;
    bincode::serialize_into(cache, &scattergram)?;

    Ok(scattergram)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct College {
    id: String,
    name: String,
    core_mapping: Option<CoreMapping>,
    total_applying: Option<u32>,
}

impl College {
    fn total_applying(&self) -> u32 {
        self.total_applying.unwrap_or(0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CoreMapping {
    uuid: String,
}

fn get_college_stats(uuid: &str) -> Result<CollegeStats> {
    let res = minreq::get(APPLICATION_HISTORY_ADDRESS.to_owned() + uuid)
        .with_header("Authorization", AUTH_TOKEN)
        .send()
        .context("Getting college stats")?;

    if res.status_code != 200 {
        return Err(anyhow::anyhow!(
            "Failed to get college stats: {}",
            res.status_code
        ));
    }

    Ok(res
        .json::<CollegeStats>()
        .context("Parsing college stats")?)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CollegeStats {
    scattergrams: Option<Scattergrams>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Scattergrams {
    weighted_gpa: WeightedGpa,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WeightedGpa {
    gpa_count: u32,
    gpa_sum: f32,
    gpa_avg: Option<f32>,
    gpa_conv_sum: f32,
    gpa_conv_avg: Option<f32>,
    act: Option<Test>,
    sat: Option<Test>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Test {
    count: u32,
    sum: u32,
    avg: Option<f32>,
    gpa_count: u32,
    gpa_sum: f32,
    gpa_avg: Option<f32>,
    gpa_conv_sum: f32,
    gpa_conv_avg: Option<f32>,
    apps: Applications,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Applications {
    accepted: Option<Vec<Student>>,
    denied: Option<Vec<Student>>,
    waitlisted_unknown: Option<Vec<Student>>,
    waitlisted_accepted: Option<Vec<Student>>,
    waitlisted_denied: Option<Vec<Student>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Student {
    current_student: bool,
    type_name: Option<String>,
    act_composite: u32,
    act_composite_student: u32,
    highest_combo_sat: u32,
    student_sat_1600_composite: Option<u32>,
    is_test_optional: Option<u32>,
    gpa: f32,
}
