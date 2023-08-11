use std::{
    collections::HashMap,
    fs::{self, File},
};

use anyhow::{Context, Error, Result};
use chrono::{Datelike, NaiveDateTime};
use once_cell::sync::Lazy;
use plotters::{
    prelude::{ChartBuilder, IntoDrawingArea, IntoSegmentedCoord, SVGBackend},
    series::Histogram,
    style::{Color, BLUE, RED, WHITE},
};
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

    let year_data = data.iter().map(|x| x.ts.year() as u32).collect::<Vec<_>>();
    let year_range = *year_data.iter().min().unwrap()..=*year_data.iter().max().unwrap();
    {
        let counts = &year_range
            .clone()
            .map(|x| year_data.iter().filter(|y| **y == x).count() as u32)
            .collect::<Vec<_>>();
        let count_range = 0..*counts.iter().max().unwrap() + 1;

        let root = SVGBackend::new("out.svg", (640 * 2, 480 * 2)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption("Songs per Year", ("sans-serif", 50.0))
            .build_cartesian_2d(
                (*year_range.start()..*year_range.end()).into_segmented(),
                0..(count_range.end as f32 * 1.1) as u32,
            )?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .y_desc("Count")
            .x_desc("Year")
            .axis_desc_style(("sans-serif", 15))
            .draw()?;

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(RED.mix(0.5).filled())
                .data(year_data.iter().map(|x: &u32| (*x, 1))),
        )?;
        root.present()?;
    }

    {
        let time_per_year = year_range
            .clone()
            .map(|x| {
                (
                    x,
                    data.iter()
                        .filter(|y| y.ts.year() as u32 == x)
                        .map(|y| (y.ms_played / 1000 / 60) as u32)
                        .sum::<u32>(),
                )
            })
            .collect::<HashMap<_, _>>();
        let max_time = *time_per_year.values().max().unwrap();

        let root = SVGBackend::new("out2.svg", (640 * 2, 480 * 2)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption("Time per Year", ("sans-serif", 50.0))
            .build_cartesian_2d(
                (*year_range.start()..*year_range.end()).into_segmented(),
                0..(max_time as f32 * 1.1) as u32,
            )?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .y_desc("Time (m)")
            .x_desc("Year")
            .axis_desc_style(("sans-serif", 15))
            .draw()?;

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(BLUE.mix(0.5).filled())
                .data(time_per_year),
        )?;
        root.present()?;
    }

    {
        let avg_song_len_per_year = year_range
            .clone()
            .map(|x| {
                (
                    x,
                    data.iter()
                        .filter(|y| y.ts.year() as u32 == x)
                        .map(|y| (y.ms_played / 1000) as u32)
                        .sum::<u32>()
                        / data.iter().filter(|y| y.ts.year() as u32 == x).count() as u32,
                )
            })
            .collect::<HashMap<_, _>>();

        let max_time = *avg_song_len_per_year.values().max().unwrap();

        let root = SVGBackend::new("out3.svg", (640 * 2, 480 * 2)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption("Time per Song per Year", ("sans-serif", 50.0))
            .build_cartesian_2d(
                (*year_range.start()..*year_range.end()).into_segmented(),
                0..(max_time as f32 * 1.1) as u32,
            )?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .y_desc("Time (s)")
            .x_desc("Year")
            .axis_desc_style(("sans-serif", 15))
            .draw()?;

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(BLUE.mix(0.5).filled())
                .data(avg_song_len_per_year),
        )?;
        root.present()?;
    }

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
