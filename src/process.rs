use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::Result;
use lazy_static::lazy_static;
use scraper::{Html, Selector};

lazy_static! {
    static ref REVISION_LIST_SELECTOR: Selector =
        Selector::parse("ul:nth-child(29) > li > a").unwrap();
    static ref BADGE_SELECTOR: Selector = Selector::parse("li > strong > a").unwrap();
    static ref REQUIREMENT_TITLE_SELECTOR: Selector =
        Selector::parse("p.center > a, h3.center > a, font").unwrap();
}

const MERIT_BADGE_HOME: &str = "http://usscouts.org/usscouts/meritbadges.asp";

fn main() {
    println!("[*] Loading owned badges");
    let raw_owned = fs::read_to_string("owned.csv").unwrap();
    let owned = raw_owned
        .lines()
        .skip(1)
        .filter_map(|x| x.split_once(","))
        .collect::<Vec<_>>();
    println!(" └ Found {} badges", owned.len());

    println!("[*] Loading Badges");
    let (badges, revisions) = get_revisions().unwrap();
    println!(" ├ Found {} badges", badges.len());
    println!(" └ Found {} revisions", revisions.len());

    println!("[*] Loading revisions");
    let mut rev_dates = HashMap::new();
    let mut all_badges = Vec::new();
    for i in revisions.into_iter().filter(|x| x.year >= 2002) {
        print!(" ├ {}", i.year);
        let rev = load_revision(&i, &badges).unwrap();
        println!(" ({})", rev.len());

        for j in rev {
            all_badges.push(j.clone());
            rev_dates
                .entry(j)
                .and_modify(|x| *x = i.year.max(*x))
                .or_insert(i.year);
        }
    }

    println!("[*] Processing");
    all_badges.sort();
    all_badges.dedup();
    for i in owned
        .into_iter()
        .map(|x| (x.0, x.1.parse::<u16>().unwrap()))
    {
        let badge = match best(i.0, &all_badges) {
            Some(i) => i,
            None => continue,
        };

        let last_update = rev_dates.get(badge).unwrap();
        if i.1 >= *last_update {
            println!("Up to date: {}", i.0);
            continue;
        }

        println!("Outdated: {} ({}) [{}]", i.0, i.1, last_update);
    }
}

#[derive(Debug)]
struct Revision {
    year: u16,
    link: String,
}

fn load_revision(rev: &Revision, badges: &[String]) -> Result<HashSet<String>> {
    let raw_page = ureq::get(&format!("http://usscouts.org/usscouts/{}", rev.link))
        .call()?
        .into_string()?;
    let dom = Html::parse_document(&raw_page);

    let mut out = HashSet::new();
    for i in dom.select(&REQUIREMENT_TITLE_SELECTOR) {
        let mut name = collapse_whitespace(i.text().collect::<String>().as_str())
            .replace(|x: char| matches!(x, ',' | ':'), "");
        if name.contains('-') {
            name = name.split_once('-').unwrap().0.to_owned();
        }

        if !is_badge(badges, &name) {
            continue;
        }

        out.insert(name.trim().to_owned());
    }

    Ok(out)
}

/// Get links to all revision pages.
/// (badges, revisions)
fn get_revisions() -> Result<(Vec<String>, Vec<Revision>)> {
    let raw_page = ureq::get(MERIT_BADGE_HOME).call()?.into_string()?;
    let dom = Html::parse_document(&raw_page);

    let badges = dom
        .select(&BADGE_SELECTOR)
        .map(|x| collapse_whitespace(x.text().next().unwrap()).to_lowercase())
        .collect::<Vec<_>>();

    let revisions = dom
        .select(&REVISION_LIST_SELECTOR)
        .filter_map(|x| {
            Some(Revision {
                year: x.text().next()?.split_once(' ')?.0.parse().ok()?,
                link: x.value().attr("href")?.to_owned(),
            })
        })
        .collect();

    Ok((badges, revisions))
}

fn best<'a>(a: &'a str, b: &'a [String]) -> Option<&'a String> {
    let mut best = 0.0;
    let mut best_str = None;

    for i in b {
        let sim = similarity(a, i);
        if sim > best {
            best = sim;
            best_str = Some(i);
        }
    }

    best_str
}

fn collapse_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_badge(badges: &[String], name: &str) -> bool {
    let skip = |x: char| x.is_whitespace() || matches!(x, ',' | ':');
    let name = name.replace(skip, "").to_lowercase();
    badges
        .iter()
        .map(|x| x.replace(skip, ""))
        .any(|x| x == name)
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
