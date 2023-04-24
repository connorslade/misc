use std::{borrow::Cow, collections::HashMap, fs};

use anyhow::Result;
use comrak::markdown_to_html;

use crate::{
    badge::{load_badges, load_discontinued, load_owned},
    misc::{best, t},
    COMRAK_OPTIONS,
};

const OUT_FILE: &str = "report.html";

pub fn run() -> Result<()> {
    println!("[*] Loading Badges");
    let owned = load_owned()?;
    let badges = {
        let mut out = Vec::new();
        out.extend(load_discontinued()?.into_iter().map(Badge::Discontinued));
        out.extend(
            load_badges()?
                .into_iter()
                .map(|x| Badge::Existing(x.name, x.update_date)),
        );
        out
    };

    println!("[*] Processing Badges");
    let mut reports = Vec::new();

    for i in owned.iter() {
        let badge = i.name.to_lowercase();
        let badge = best(&badge, &badges, |x| Cow::Borrowed(x.name())).unwrap();

        if matches!(badge, Badge::Discontinued(_)) {
            reports.push(BadgeReport {
                name: badge.name().to_owned(),
                status: BadgeStatus::Removed,
            });
            continue;
        }

        if badge.outdated(i.date) {
            reports.push(BadgeReport {
                name: badge.name().to_owned(),
                status: BadgeStatus::Outdated,
            });
            continue;
        }

        reports.push(BadgeReport {
            name: badge.name().to_owned(),
            status: BadgeStatus::Current,
        });
    }

    println!("[*] Generating Report");
    let discontinued_badges = filter_type(&reports, BadgeStatus::Removed);
    let outdated_badges = filter_type(&reports, BadgeStatus::Outdated);
    let current_badges = filter_type(&reports, BadgeStatus::Current);
    let mut badges = owned.clone();
    badges.dedup_by(|a, b| a.name.eq_ignore_ascii_case(&b.name));

    let markdown = include_str!("./templates/report.md")
        .replacen("{{TOTAL_BADGES}}", badges.len().to_string().as_str(), 1)
        .replacen("{{TOTAL_BOOKS}}", owned.len().to_string().as_str(), 1)
        .replacen(
            "{{OUTDATED_BOOKS}}",
            outdated_badges.len().to_string().as_str(),
            1,
        )
        .replacen(
            "{{DISCONTINUED_BOOKS}}",
            discontinued_badges.len().to_string().as_str(),
            1,
        )
        .replacen(
            "{{DISCONTINUED_BADGES}}",
            &book_list(&discontinued_badges),
            1,
        )
        .replacen(
            "{{CURRENT_BOOKS}}",
            current_badges.len().to_string().as_str(),
            1,
        )
        .replacen("{{OUTDATED_BADGES}}", &book_list(&outdated_badges), 1)
        .replacen("{{CURRENT_BADGES}}", &book_list(&current_badges), 1);
    let html = markdown_to_html(&markdown, &COMRAK_OPTIONS);
    fs::write(OUT_FILE, html)?;

    Ok(())
}

fn filter_type(items: &[BadgeReport], status: BadgeStatus) -> Vec<&BadgeReport> {
    items.iter().filter(|x| x.status == status).collect()
}

fn book_list(items: &[&BadgeReport]) -> String {
    let mut counts = HashMap::new();

    for i in items {
        let count = counts.entry(i.name.to_owned()).or_insert(0);
        *count += 1;
    }

    let mut out = String::new();
    let mut counts = counts.into_iter().collect::<Vec<_>>();
    counts.sort();

    for (name, count) in counts {
        out.push_str(
            format!(
                "* {}{}\n",
                name,
                t(count > 1, format!(" x{count}"), String::new())
            )
            .as_str(),
        );
    }

    out
}

struct BadgeReport {
    name: String,
    status: BadgeStatus,
}

#[derive(PartialEq)]
enum BadgeStatus {
    Removed,
    Outdated,
    Current,
}

enum Badge {
    Existing(String, u16),
    Discontinued(String),
}

impl Badge {
    fn name(&self) -> &str {
        match self {
            Badge::Existing(name, _) => name,
            Badge::Discontinued(name) => name,
        }
    }

    fn outdated(&self, year: u16) -> bool {
        match self {
            Badge::Existing(_, update_year) => year < *update_year,
            Badge::Discontinued(_) => unreachable!(),
        }
    }
}
