use std::collections::HashMap;

use chrono::NaiveDate;
use scraper::{element_ref::Select, Html};
use ureq::AgentBuilder;

const BASE_PAGE: &str = "https://parents.c2.genesisedu.net/bernardsboe";

const USERNAME: &str = "";
const PASSWORD: &str = "";
const STUDENT_ID: &str = "";

#[derive(Debug)]
struct Grade {
    date: NaiveDate,
    assignment: String,
    grade: (u32, u32),
}

macro_rules! selector {
    ($raw:expr) => {{
        static SELECTOR: once_cell::sync::OnceCell<scraper::Selector> =
            once_cell::sync::OnceCell::new();
        SELECTOR.get_or_init(|| scraper::Selector::parse($raw).unwrap())
    }};
}

fn main() {
    let agent = AgentBuilder::new()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:120.0) Gecko/20100101 Firefox/120.0")
        .build();

    agent.get(&format!("{BASE_PAGE}/sis/view")).call().unwrap();
    agent
        .get(&format!("{BASE_PAGE}/sis/j_security_check"))
        .query("j_username", USERNAME)
        .query("j_password", PASSWORD)
        .call()
        .unwrap();
    let assignments = agent.get(&format!("https://parents.c2.genesisedu.net/bernardsboe/parents?tab1=studentdata&tab2=gradebook&tab3=listassignments&action=form&studentid={STUDENT_ID}")).call().unwrap();

    let document = Html::parse_document(&assignments.into_string().unwrap());
    let items = document.select(selector!("table.list tr[style]"));

    // Date: `.cellCenter div:nth-child(2)`
    // Class: `.cellLeft[height] div:nth-child(1)`
    // Assignment: `.cellLeft:not([height]) b`
    // Grade: `.cellLeft[nowrap]`
    let mut classes = HashMap::new();
    for item in items {
        let Some(date) = parse_date(get_text(
            item.select(selector!(".cellCenter div:nth-child(2)")),
        )) else {
            continue;
        };
        let class = get_text(item.select(selector!(".cellLeft[height] div:nth-child(1)")));
        let assignment = get_text(item.select(selector!(".cellLeft:not([height]) b")));
        let Some(grade) = parse_grade(get_text(item.select(selector!(".cellLeft[nowrap]")))) else {
            continue;
        };

        classes.entry(class).or_insert(Vec::new()).push(Grade {
            date,
            assignment: assignment.to_owned(),
            grade,
        });
    }

    dbg!(classes);
}

fn get_text<'a, 'b>(mut item: Select<'a, 'b>) -> &'a str {
    item.next().unwrap().text().next().unwrap()
}

fn parse_grade(grade: &str) -> Option<(u32, u32)> {
    let mut parts = grade.split_ascii_whitespace();
    let numerator = parts.next()?.parse().ok()?;
    if parts.next()? != "/" {
        return None;
    }
    let denominator = parts.next()?.parse().ok()?;

    Some((numerator, denominator))
}

fn parse_date(date: &str) -> Option<NaiveDate> {
    let (month, day) = date.split_once("/")?;
    NaiveDate::from_ymd_opt(2023, month.parse().ok()?, day.parse().ok()?)
}
