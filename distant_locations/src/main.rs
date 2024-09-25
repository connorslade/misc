use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use geo::{GeodesicDistance, Point};
use geocoding::Opencage;

use geocode::{geocode, GeocodeCache};

const OPENCAGE_API_KEY: &str = "";
const CACHE_PATH: &str = "geocode_cache.bin";

mod geocode;

#[derive(Parser)]
struct Args {
    layer_a: PathBuf,
    layer_b: PathBuf,

    /// Distance in miles
    #[clap(short, long, default_value = "30.0")]
    distance: f64,
}

fn to_lines(data: &str) -> Vec<&str> {
    data.lines()
        .filter(|line| !line.trim().is_empty())
        .collect()
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Load data
    let a_data = fs::read_to_string(&args.layer_a)?;
    let a_data = to_lines(&a_data);

    let b_data = fs::read_to_string(&args.layer_b)?;
    let b_data = to_lines(&b_data);

    // Geocode locations
    let mut cache = GeocodeCache::load()?;
    let opencage = Opencage::new(OPENCAGE_API_KEY.into());

    let a_coords = geocode(&opencage, &mut cache, &a_data);
    let b_coords = geocode(&opencage, &mut cache, &b_data);
    cache.save()?;

    // Check if any locations could not be geocoded
    if a_coords.iter().chain(b_coords.iter()).any(|x| x.is_none()) {
        println!("[-] Some locations could not be geocoded, exiting.");
        return Ok(());
    }

    let a_coords = a_coords.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>();
    let b_coords = b_coords.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>();

    // List all b_coords that are not within args.distance miles of any a_coords
    for b in b_coords {
        let b_point = Point::new(b.lat, b.lon);

        for a in &a_coords {
            let a_point = Point::new(a.lat, a.lon);
            let distance = a_point.geodesic_distance(&b_point) / 1609.344;
            if distance < args.distance {
                continue;
            }
        }

        println!(
            "{} is not within {} miles of any a_coords",
            b.friendly_name, args.distance
        );
    }

    Ok(())
}
