use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::Result;
use geocoding::Opencage;
use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::CACHE_PATH;

#[derive(Default, Serialize, Deserialize)]
pub struct GeocodeCache {
    map: HashMap<String, GeocodeResult>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GeocodeResult {
    pub friendly_name: String,
    pub lat: f64,
    pub lon: f64,
}

impl GeocodeCache {
    pub fn load() -> Result<Self> {
        let cache_path = PathBuf::from(CACHE_PATH);
        if cache_path.exists() {
            println!("[*] Using cache");
            let cache_data = fs::read(&cache_path).unwrap();
            return Ok(bincode::deserialize::<GeocodeCache>(&cache_data)?);
        }

        Ok(GeocodeCache::default())
    }

    pub fn save(&self) -> Result<()> {
        let cache_path = PathBuf::from(CACHE_PATH);
        let cache_data = bincode::serialize(&self).unwrap();
        fs::write(&cache_path, &cache_data)?;

        Ok(())
    }
}

pub fn geocode(
    geocode: &Opencage,
    cache: &mut GeocodeCache,
    locations: &[&str],
) -> Vec<Option<GeocodeResult>> {
    let coords = locations
        .par_iter()
        .progress_count(locations.len() as u64)
        .map(|line| {
            if let Some(result) = cache.map.get(*line) {
                return Some(result.clone()); // todo: dont clone
            }

            match geocode.forward_full(line, None) {
                Ok(result) => {
                    let result = &result.results[0];
                    let friendly_name = result.formatted.to_owned();

                    let lat = *result.geometry.get("lat").unwrap();
                    let lon = *result.geometry.get("lng").unwrap();

                    Some(GeocodeResult {
                        friendly_name,
                        lat,
                        lon,
                    })
                }
                Err(_) => {
                    eprintln!("Failed to geocode: {}", line);
                    None
                }
            }
        })
        .collect::<Vec<_>>();

    for (line, result) in locations.iter().zip(coords.iter()) {
        if let Some(result) = result {
            cache.map.insert(line.to_string(), result.clone());
        }
    }

    coords
}
