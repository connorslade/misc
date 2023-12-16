use std::collections::HashMap;

use chrono::NaiveDate;
use dotenvy::dotenv_override;
use plotters::{
    backend::SVGBackend,
    drawing::IntoDrawingArea,
    element::Rectangle,
    series::LineSeries,
    style::{Palette, Palette99},
};
use scraper::{element_ref::Select, Html};
use ureq::AgentBuilder;

const BASE_PAGE: &str = "https://parents.c2.genesisedu.net/bernardsboe";
const OUTPUT: &str = "output.svg";

#[derive(Debug)]
struct Config {
    username: String,
    password: String,
    student_id: String,
}

impl Config {
    fn from_env() -> Self {
        Self {
            username: std::env::var("USERNAME").unwrap(),
            password: std::env::var("PASSWORD").unwrap(),
            student_id: std::env::var("STUDENT_ID").unwrap(),
        }
    }
}

#[derive(Debug)]
struct Grade {
    date: NaiveDate,
    _assignment: String,
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
    dotenv_override().unwrap();
    let config = Config::from_env();

    let agent = AgentBuilder::new()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:120.0) Gecko/20100101 Firefox/120.0")
        .build();

    agent.get(&format!("{BASE_PAGE}/sis/view")).call().unwrap();
    agent
        .get(&format!("{BASE_PAGE}/sis/j_security_check"))
        .query("j_username", &config.username)
        .query("j_password", &config.password)
        .call()
        .unwrap();
    let assignments = agent.get(&format!("{BASE_PAGE}/parents?tab1=studentdata&tab2=gradebook&tab3=listassignments&action=form&studentid={}", config.student_id)).call().unwrap();

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
            _assignment: assignment.to_owned(),
            grade,
        });
    }

    let (first_date, last_date) = get_date_range(&classes);
    let min_grade = classes
        .values()
        .flatten()
        .map(|g| g.grade.0 as f64 / g.grade.1 as f64 * 100.0)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    for grades in classes.values_mut() {
        grades.sort_by_key(|g| g.date);
    }

    println!(
        "Found {} classes and {} total assignments.",
        classes.len(),
        classes.values().map(|v| v.len()).sum::<usize>()
    );

    let root = SVGBackend::new(OUTPUT, (1024, 768)).into_drawing_area();
    root.fill(&plotters::style::RGBColor(255, 255, 255))
        .unwrap();

    let mut chart = plotters::chart::ChartBuilder::on(&root)
        .margin(5)
        .caption("Grade History", ("sans-serif", 50))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(first_date..last_date, min_grade..100.0)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(30)
        .y_labels(10)
        .y_desc("Grade (%)")
        .x_desc("Date")
        .draw()
        .unwrap();

    // Draw dashed line at 90%
    chart
        .draw_series(LineSeries::new(
            vec![(first_date, 90.0), (last_date, 90.0)],
            plotters::style::RGBColor(0, 0, 0),
        ))
        .unwrap();

    let mut classes = classes.into_iter().collect::<Vec<_>>();
    classes.sort_by_key(|(class, _)| *class);

    for (idx, (class, grades)) in classes.into_iter().enumerate() {
        let mut points = Vec::new();
        let mut acc = (0, 0);
        for grade in grades {
            acc.0 += grade.grade.0;
            acc.1 += grade.grade.1;
            points.push((grade.date, acc.0 as f64 / acc.1 as f64 * 100.0));
        }

        let color = Palette99::pick(idx);
        chart
            .draw_series(LineSeries::new(points.into_iter(), &color))
            .unwrap()
            .label(class)
            .legend(move |(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &color));
    }

    chart.configure_series_labels().draw().unwrap();

    root.present().unwrap();
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

fn get_date_range(classes: &HashMap<&str, Vec<Grade>>) -> (NaiveDate, NaiveDate) {
    let (mut min, mut max) = (NaiveDate::MAX, NaiveDate::MIN);

    for grades in classes.values() {
        for grade in grades {
            min = min.min(grade.date);
            max = max.max(grade.date);
        }
    }

    (min, max)
}
