use std::collections::{HashMap, HashSet};

use anyhow::Result;
use lazy_static::lazy_static;
use scraper::{Html, Selector};

lazy_static! {
    static ref REVISION_LIST_SELECTOR: Selector =
        Selector::parse("ul:nth-child(29) > li > a").unwrap();
    static ref BADGE_SELECTOR: Selector = Selector::parse("li > strong > a").unwrap();
    static ref REQUIREMENT_TITLE_SELECTOR: Selector =
        // Selector::parse("#requirements > h3 > a, #meritbadges > h2 > a, #content > h3 > a, font")
        Selector::parse("p.center > a, h3.center > a, font")
            .unwrap();
}

const MERIT_BADGE_HOME: &str = "http://usscouts.org/usscouts/meritbadges.asp";

fn main() {
    println!("[*] Loading Badges");
    let (badges, revisions) = get_revisions().unwrap();
    println!(" ├ Found {} badges", badges.len());
    println!(" └ Found {} revisions", revisions.len());

    println!("[*] Loading revisions");
    for i in revisions.into_iter().filter(|x| x.year >= 2002) {
        print!(" ├ {}", i.year);
        let rev = load_revision(i, &badges).unwrap();
        println!(" ({})", rev.len());
    }
}

#[derive(Debug)]
struct Revision {
    year: u16,
    link: String,
}

#[derive(Debug)]
struct State {
    year: u16,
    data: String,
}

fn load_revision(rev: Revision, badges: &[String]) -> Result<HashSet<String>> {
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
