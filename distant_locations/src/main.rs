use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use geocoding::Opencage;

use geocode::{geocode, GeocodeCache};

const OPENCAGE_API_KEY: &str = "";
const CACHE_PATH: &str = "geocode_cache.bin";

mod geocode;

#[derive(Parser)]
struct Args {
    data: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let data = fs::read_to_string(args.data)?;
    let data = data.lines().collect::<Vec<_>>();

    let mut cache = GeocodeCache::load()?;
    let opencage = Opencage::new(OPENCAGE_API_KEY.into());

    let coords = geocode(opencage, &mut cache, &data);

    cache.save()?;

    if coords.iter().any(|result| result.is_none()) {
        println!("[-] Some locations could not be geocoded, exiting.");
        return Ok(());
    }

    for result in coords.iter().flatten() {
        println!("{}: {}, {}", result.friendly_name, result.lat, result.lon);
    }

    Ok(())
}
