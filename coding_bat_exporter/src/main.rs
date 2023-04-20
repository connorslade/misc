use std::{collections::HashMap, fs};

use clap::Parser;
use colored::Colorize;
use indicatif::ProgressBar;
use lazy_static::lazy_static;
use scraper::{Html, Selector};
use serde::Serialize;
use serde_json::json;
use ureq::AgentBuilder;

lazy_static! {
    // Author Page
    static ref PROBLEM_SELECTOR: Selector =
        Selector::parse("body > ul:nth-child(9) > li > a").unwrap();

    // Problem Page
    static ref PROBLEM_DOC_SELECTOR: Selector = Selector::parse("#tadoc").unwrap();
    static ref PROBLEM_HINT_SELECTOR: Selector = Selector::parse("#tahint").unwrap();
    static ref PROBLEM_CASE_SELECTOR: Selector = Selector::parse("#tacases").unwrap();
    static ref PROBLEM_CODE_SELECTOR: Selector = Selector::parse("#tacode").unwrap();
    static ref PROBLEM_TAGS_SELECTOR: Selector = Selector::parse("textarea[name=\"tags\"]").unwrap();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Codingbat base path
    #[arg(default_value = "https://codingbat.com", long, short)]
    base_path: String,

    /// Codingbat username
    #[arg(required(true))]
    username: String,

    /// Codingbat password
    #[arg(required(true))]
    password: String,

    /// Output File
    #[arg(required(true))]
    out_file: String,
}

#[derive(Serialize)]
struct Problem {
    document: String,
    hint: String,
    cases: Vec<(Vec<String>, String)>,
    code: String,
    tags: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let agent = AgentBuilder::new().redirects(0).build();

    // Get session ID
    println!("[*] Logging in");
    let message = agent
        .post(&format!("{}/login", args.base_path))
        .query("uname", &args.username)
        .query("pw", &args.password)
        .call()
        .unwrap()
        .header("Location")
        .unwrap()
        .split_once("message=")
        .map(|x| x.1.replace('+', " "));

    if let Some(i) = message {
        println!("{}", format!("[-] Error: `{i}`").red());
        return;
    }

    // Get all authored problems
    println!("[*] Getting authors problems");
    let problems_page = agent
        .get(&format!("{}/author", args.base_path))
        .call()
        .unwrap()
        .into_string()
        .unwrap();

    let problems_html = Html::parse_document(&problems_page);
    let problem_ids = problems_html
        .select(&PROBLEM_SELECTOR)
        .map(|x| x.value().attr("href").unwrap().rsplit_once('/').unwrap().1)
        .collect::<Vec<_>>();

    // Get problems
    println!("[*] Downloading Problems");
    let progress = ProgressBar::new(problem_ids.len() as u64);
    let mut final_problems = HashMap::new();

    for i in problem_ids {
        let problem_html = agent
            .get(&format!("{}/author/{i}", args.base_path))
            .call()
            .unwrap()
            .into_string()
            .unwrap();
        let problem = Html::parse_document(&problem_html);

        let document = problem
            .select(&PROBLEM_DOC_SELECTOR)
            .next()
            .unwrap()
            .inner_html();
        let hint = problem
            .select(&PROBLEM_HINT_SELECTOR)
            .next()
            .unwrap()
            .inner_html();
        let cases = problem
            .select(&PROBLEM_CASE_SELECTOR)
            .next()
            .unwrap()
            .inner_html();
        let code = problem
            .select(&PROBLEM_CODE_SELECTOR)
            .next()
            .unwrap()
            .inner_html();
        let tags = problem
            .select(&PROBLEM_TAGS_SELECTOR)
            .next()
            .unwrap()
            .inner_html();

        final_problems.insert(
            i,
            Problem {
                document,
                hint,
                cases: cases.lines().map(case_parser).collect(),
                code,
                tags: tags.split(' ').map(str::to_owned).collect(),
            },
        );

        progress.inc(1);
    }

    // Save problems
    progress.finish_and_clear();
    println!("[*] Saving");
    fs::write(args.out_file, json!(final_problems).to_string()).unwrap();
    println!("[*] Done!");
}

fn case_parser(inp: &str) -> (Vec<String>, String) {
    let mut args = Vec::new();
    let mut in_quotes = false;
    let mut in_array = false;
    let mut working = String::new();

    for i in inp.chars() {
        match i {
            '"' if !in_array => in_quotes ^= true,
            '{' if !in_array && !in_quotes => {
                in_array = true;
                working.push('{');
            }
            '}' if in_array && !in_quotes => {
                in_array = false;
                working.push('}');
            }
            ',' if !in_quotes && !in_array => {
                args.push(working.trim().to_owned());
                working.clear();
            }
            _ => working.push(i),
        }
    }

    (args, working.trim().to_owned())
}
