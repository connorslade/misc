use std::{collections::HashMap, fs, path::Path};

use anyhow::{Context, Result};
use comrak::{markdown_to_html, ComrakOptions};
use indicatif::ParallelProgressIterator;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref COMRAK_OPTIONS: ComrakOptions = {
        let mut options = ComrakOptions::default();
        options.render.unsafe_ = true;
        options.parse.smart = true;
        options
    };
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

const BASE_PAGE: &str = "http://usscouts.org";
const MERIT_BADGE_HOME: &str = "http://usscouts.org/usscouts/meritbadges.asp";

const OUT_DIR: &str = "out_md";
const OWNED_FILE: &str = "owned.csv";
const CACHE_FILE: &str = "badge_cache.bin";
// wkhtmltopdf --page-height 8.5in --page-width 5.5in "American Business-2008.html" out.pdf

fn main() -> Result<()> {
    let out_dir = Path::new(OUT_DIR);
    if !out_dir.exists() {
        fs::create_dir(out_dir)?;
    }

    // (book, date)
    let owned = fs::read_to_string(OWNED_FILE)?;
    let owned = owned
        .lines()
        .skip(1)
        .filter_map(|x| x.split_once(','))
        .collect::<Vec<_>>();

    println!("[*] Loading Badges");
    let path = Path::new(CACHE_FILE);
    let badges = if path.exists() {
        let raw = fs::read(path)?;
        bincode::deserialize(&raw)?
    } else {
        let badges = get_badges()?
            .par_iter()
            .progress()
            .filter_map(|x| load_badge(&x.1).ok())
            .collect::<Vec<_>>();
        let out = bincode::serialize(&badges)?;
        fs::write(path, out)?;
        badges
    };

    println!("[*] Writing Markdown");

    owned.par_iter().progress().for_each(|x| {
        let date = x.1.parse::<u16>().unwrap();
        let badge = x.0.to_lowercase();
        let badge = best(&badge, &badges, |x| x.name.to_owned()).unwrap();
        if date >= badge.update_date {
            return;
        }

        let out = include_str!("./template.md")
            .replace("{{TITLE}}", &badge.name)
            .replace("{{IMAGE_LINK}}", &badge.icon_link)
            .replace("{{BOOK_DATE}}", date.to_string().as_str())
            .replace("{{UPDATE_DATE}}", badge.update_date.to_string().as_str())
            .replace("{{REQUIREMENTS}}", &badge.requirements);

        let rendered = markdown_to_html(&out, &COMRAK_OPTIONS);
        let mut path = out_dir.join(format!("{}-{}.html", badge.name.replace(' ', "-"), date));
        let mut i = 1;
        while path.exists() {
            path.set_file_name(format!(
                "{}-{}-{}.html",
                badge.name.replace(' ', "-"),
                date,
                i
            ));
            i += 1;
        }
        fs::write(path, rendered).unwrap();
    });

    Ok(())
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

#[derive(Debug, Serialize, Deserialize)]
struct BadgeData {
    name: String,
    icon_link: String,
    update_date: u16,
    requirements: String,
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

fn collapse_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn best<'a, T>(a: &'a str, b: &'a [T], transformer: fn(&T) -> String) -> Option<&'a T> {
    let mut best = 0.0;
    let mut best_str = None;

    for i in b {
        let sim = similarity(a, &transformer(i));
        if sim > best {
            best = sim;
            best_str = Some(i);
        }
    }

    best_str
}

pub fn similarity(str1: &str, str2: &str) -> f64 {
    let a = str1.replace(' ', "");
    let b = str2.replace(' ', "");
    // Check some simple cases
    if a == b {
        return 1.0;
    }
    if a.len() < 2 || b.len() < 2 {
        return 0.0;
    }
    let mut first_bigrams: HashMap<&str, i32> = HashMap::new();
    for i in 0..a.len() - 1 {
        let bigram = &a[i..i + 2];
        let count = first_bigrams.get(bigram).unwrap_or(&0) + 1;
        first_bigrams.insert(bigram, count);
    }
    let mut intersection_size = 0;
    for i in 0..b.len() - 1 {
        let bigram = &b[i..i + 2];
        let count = *first_bigrams.get(bigram).unwrap_or(&0);
        if count > 0 {
            first_bigrams.insert(bigram, count - 1);
            intersection_size += 1;
        }
    }
    (2.0 * intersection_size as f64) / (str1.len() + str2.len() - 2) as f64
}
