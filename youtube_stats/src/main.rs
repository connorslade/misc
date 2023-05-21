use std::{borrow::Cow, fs, path::PathBuf, process};

use clap::Parser;
use hashbrown::HashMap;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

use crate::{api::KeyStore, history::Watch};

mod api;
mod history;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg()]
    pub input_file: PathBuf,

    #[arg()]
    pub output_file: PathBuf,
    
    #[arg(short, long)]
    pub api_key: Option<String>,

    #[arg(short, long)]
    pub api_key_file: Option<PathBuf>,
}

pub struct VideoMeta {
    pub count: usize,
    pub length: f32,
}

fn main() {
    let args = Args::parse();
    let key_store = match (args.api_key, args.api_key_file) {
        (_, Some(f)) => KeyStore::from_file(f).unwrap(),
        (Some(k), _) => KeyStore::from_key(&k),
        _ => {
            eprintln!("[-] No API key provided");
            process::exit(1);
        }
    };

    println!(
        "[*] Verifying {} API key{}",
        key_store.keys.len(),
        if key_store.keys.len() == 1 { "" } else { "s" }
    );
    key_store.verify().unwrap();
    println!(" \\ Success");

    assert_ext(&args.input_file, "json");
    assert_ext(&args.output_file, "csv");

    println!("[*] Loading watch history");
    let file = fs::read_to_string(&args.input_file).unwrap();
    let json = serde_json::from_str::<Vec<Watch>>(&file).unwrap();

    println!("[*] Watched {} videos", json.len());
    let mut videos = HashMap::<Watch, VideoMeta>::new();
    for i in json {
        let url = i.title_url.replacen("\u{003d}", "=", 1);
        if url.is_empty() || i.subtitles.is_empty() {
            continue;
        }

        let count = videos.entry(i).or_insert(VideoMeta::new());
        count.increment();
    }

    if videos.len() > key_store.request_threshold * key_store.keys.len() {
        eprintln!(
            "[-] Not enough API keys to fetch video lengths ({} keys, {} videos)",
            key_store.keys.len(),
            videos.len()
        );
        process::exit(1);
    }

    println!("[*] Sorting videos");
    let mut videos = videos.into_iter().collect::<Vec<_>>();
    videos.sort_by(|a, b| b.1.count.cmp(&a.1.count));

    println!("[*] Fetching video lengths");
    let videos = videos
        .into_par_iter()
        .progress()
        .filter_map(|x| {
            let length = match api::video_length(x.0.id(), &key_store) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("Failed to fetch video length for {}: {}", x.0.id(), e);
                    return None;
                }
            };

            Some((
                VideoMeta {
                    count: x.1.count,
                    length,
                },
                x.0,
            ))
        })
        .collect::<Vec<_>>();

    println!("[*] Writing to file");

    let mut out = String::from("title,id,watch_count,video_length\n");
    for i in videos {
        out.push_str(&format!(
            "{},{},{},{}\n",
            i.1.title.strip_prefix("Watched ").unwrap_or(&i.1.title),
            i.1.id(),
            i.0.count,
            i.0.length
        ));
    }

    fs::write(&args.output_file, out).unwrap();
}

fn assert_ext(path: &PathBuf, ext: &str) {
    if path.extension().map(|x| x.to_string_lossy()) != Some(Cow::Borrowed(ext)) {
        eprintln!("Input file must be a JSON file");
        process::exit(-1);
    }
}

impl VideoMeta {
    pub fn new() -> Self {
        Self {
            count: 0,
            length: 0.0,
        }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }
}
