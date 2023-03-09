//! ```cargo
//! [dependencies]
//! anyhow = "1.0.69"
//! indicatif = "0.17.3"
//! lazy_static = "1.4.0"
//! scraper = "0.15.0"
//! ureq = "2.6.2"
//! ```

use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

use indicatif::ProgressBar;
use lazy_static::lazy_static;
use scraper::{Html, Selector};

lazy_static! {
    static ref LINK_SELECTOR: Selector = Selector::parse("a").unwrap();
    static ref OUT: PathBuf = Path::new("out").to_path_buf();
}

fn main() {
    let links = include_str!("../links.txt").lines().collect::<Vec<_>>();

    if !OUT.exists() {
        fs::create_dir(&*OUT).unwrap();
    }

    let bar = ProgressBar::new(links.len() as u64);
    for i in links {
        process(i).unwrap();
        bar.inc(1);
    }
}

fn process(link: &str) -> Result<(), anyhow::Error> {
    let raw = ureq::get(link).call()?.into_string()?;
    let doc = Html::parse_document(&raw);
    let link = doc
        .select(&*LINK_SELECTOR)
        .filter(|e| e.value().attr("href").unwrap_or("").contains("filestore"))
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap();

    let file_name = link.split('/').last().unwrap();
    let mut file = ureq::get(link).call()?.into_reader();
    let mut out = File::create(OUT.join(file_name))?;
    io::copy(&mut file, &mut out)?;

    Ok(())
}
