use std::io::{stdout, Write};

use clap::Parser;
use colored::Colorize;
use lazy_static::lazy_static;
use scraper::{Html, Selector};
use ureq::{Agent, AgentBuilder};

lazy_static! {
    static ref PROBLEM_SELECTOR: Selector =
        Selector::parse("body > ul:nth-child(9) > li > a").unwrap();
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
}

struct Problem {}

fn main() {
    let args = Args::parse();

    let agent = AgentBuilder::new().redirects(0).build();

    // Get session ID
    let message = agent
        .post(&format!("{}/login", args.base_path))
        .query("uname", &args.username)
        .query("pw", &args.password)
        .call()
        .unwrap()
        .header("Location")
        .unwrap()
        .split_once("message=")
        .map(|x| x.1.replace("+", " "));

    if let Some(i) = message {
        println!("{}", format!("[-] Error: `{i}`").red());
        return;
    }

    // Get all authored problems
    let problems_page = agent
        .get(&format!("{}/author", args.base_path))
        .call()
        .unwrap()
        .into_string()
        .unwrap();

    let problems_html = Html::parse_document(&problems_page);
    let problem_ids = problems_html
        .select(&PROBLEM_SELECTOR)
        .map(|x| x.value().attr("href").unwrap().rsplit_once("/").unwrap().1);

    // Get problems
    for i in problem_ids {
        
    }

    // Save problems
}
