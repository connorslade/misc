use std::{collections::HashMap, fs};

use anyhow::Result;
use comrak::markdown_to_html;

use crate::{
    badge::{load_badges, load_discontinued, load_owned},
    misc::best,
    COMRAK_OPTIONS,
};

const OUT_FILE: &str = "report.html";

pub fn run() -> Result<()> {
    println!("[*] Loading Badges");
    let owned = load_owned()?;
    let discontinued = load_discontinued()?;
    let badges = load_badges()?;

    println!("[*] Processing Badges");
    let mut reports = Vec::new();

    for i in owned.iter() {
        let badge = i.name.to_lowercase();
        let badge = best(&badge, &badges, |x| x.name.to_owned()).unwrap();
        if i.date >= badge.update_date {
            reports.push(BadgeReport {
                name: badge.name.to_owned(),
                status: BadgeStatus::Current,
            });
            continue;
        }

        let status = if discontinued.contains(&badge.name) {
            BadgeStatus::Removed
        } else {
            BadgeStatus::Outdated
        };

        reports.push(BadgeReport {
            name: badge.name.to_owned(),
            status,
        });
    }

    println!("[*] Generating Report");
    let discontinued_badges = filter_type(&reports, BadgeStatus::Removed);
    let outdated_badges = filter_type(&reports, BadgeStatus::Outdated);
    let current_badges = filter_type(&reports, BadgeStatus::Current);

    let markdown = include_str!("./templates/report.md")
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
        .replacen("{{OUTDATED_BADGES}}", &book_list(&outdated_badges), 1)
        .replacen("{{CURRENT_BADGES}}", &book_list(&current_badges), 1);
    let html = markdown_to_html(&markdown, &COMRAK_OPTIONS);
    fs::write(OUT_FILE, html)?;

    Ok(())
}

fn filter_type(items: &[BadgeReport], status: BadgeStatus) -> Vec<&BadgeReport> {
    items
        .iter()
        .filter(|x| x.status == status)
        .collect::<Vec<&BadgeReport>>()
}

fn book_list(items: &[&BadgeReport]) -> String {
    let mut counts = HashMap::new();

    for i in items {
        let count = counts.entry(i.name.to_owned()).or_insert(0);
        *count += 1;
    }

    let mut out = String::new();
    for (name, count) in counts {
        out.push_str(
            format!(
                "* {}{}\n",
                name,
                if count > 0 {
                    format!(" x{}", count)
                } else {
                    String::new()
                }
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
