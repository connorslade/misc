use std::{fs, path::Path};

use crate::misc::collapse_whitespace;
use crate::{BASE_PAGE, CACHE_FILE, MERIT_BADGE_HOME};

use anyhow::{Context, Ok, Result};
use indicatif::ParallelProgressIterator;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref LAST_UPDATE_REGEX: Regex =
        Regex::new(r"Requirements last updated in: (\d{4})").unwrap();
    static ref BADGE_SELECTOR: Selector = Selector::parse("li > strong > a").unwrap();
    static ref CONTENT_SELECTOR: Selector = Selector::parse("#requirements > ol").unwrap();
    static ref VERSION_SELECTOR: Selector = Selector::parse("#version > p:nth-child(2)").unwrap();
    static ref TITLE_SELECTOR: Selector =
        Selector::parse("table.center > tbody > tr > td > h1").unwrap();
    static ref ICON_SELECTOR: Selector =
        Selector::parse("table.center > tbody > tr > td:nth-child(2) > img, table.center > tbody > tr > td:nth-child(2) > p > img").unwrap();
}

pub fn load_badges() -> Result<Vec<BadgeData>> {
    let path = Path::new(CACHE_FILE);
    if path.exists() {
        let raw = fs::read(path)?;
        return Ok(bincode::deserialize(&raw)?);
    };

    let badges = get_badges()?
        .par_iter()
        .progress()
        .filter_map(|x| load_badge(&x.1).ok())
        .collect::<Vec<_>>();
    let out = bincode::serialize(&badges)?;
    fs::write(path, out)?;
    Ok(badges)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeData {
    pub name: String,
    pub icon_link: String,
    pub update_date: u16,
    pub requirements: String,
}

// (Name, Link)
fn get_badges() -> Result<Vec<(String, String)>> {
    let raw_page = ureq::get(MERIT_BADGE_HOME).call()?.into_string()?;
    let dom = Html::parse_document(&raw_page);
    let mut out = Vec::new();

    for i in dom.select(&BADGE_SELECTOR) {
        let mut link = i
            .value()
            .attr("href")
            .with_context(|| "No href value on link")?
            .to_owned();
        let name = collapse_whitespace(i.text().next().with_context(|| "No text content on link")?)
            .to_lowercase();

        if !link.starts_with('/') {
            link = format!("/{link}");
        }

        out.push((name, format!("{BASE_PAGE}{link}")));
    }

    Ok(out)
}

fn load_badge(link: &str) -> Result<BadgeData> {
    let raw_page = ureq::get(link).call()?.into_string()?;
    let dom = Html::parse_document(&raw_page);

    let content = collapse_whitespace(
        &dom.select(&CONTENT_SELECTOR)
            .next()
            .with_context(|| "No content found")?
            .html(),
    );

    let name = collapse_whitespace(
        &dom.select(&TITLE_SELECTOR)
            .next()
            .with_context(|| "No title found")?
            .text()
            .collect::<String>(),
    );

    let icon_link = dom
        .select(&ICON_SELECTOR)
        .next()
        .with_context(|| "No icon found")?
        .value()
        .attr("src")
        .with_context(|| "No src attribute on icon")?
        .to_owned();

    let version = collapse_whitespace(
        &dom.select(&VERSION_SELECTOR)
            .next()
            .with_context(|| "No version found")?
            .text()
            .collect::<String>(),
    );
    let version = LAST_UPDATE_REGEX
        .captures(&version)
        .with_context(|| "No update date found")?
        .get(1)
        .with_context(|| "Unable to extract update date")?
        .as_str()
        .parse::<u16>()?;

    Ok(BadgeData {
        name,
        icon_link: format!(
            "{BASE_PAGE}/mb{}{}",
            if icon_link.starts_with('/') { "" } else { "/" },
            icon_link
        ),
        update_date: version,
        requirements: content,
    })
}
