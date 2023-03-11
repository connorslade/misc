use std::env;

use anyhow::Result;
use comrak::ComrakOptions;
use lazy_static::lazy_static;

mod badge;
mod generate;
mod misc;
mod report;

const CACHE_FILE: &str = "badge_cache.bin";
const OWNED_FILE: &str = "owned.csv";

const HELP: &str = r#"Usage: <cmd> <command>

Commands:
    generate - Generate info pages for outdated merit badges
    report - Generate a report of removed and outdated merit badges"#;

lazy_static! {
    static ref COMRAK_OPTIONS: ComrakOptions = {
        let mut options = ComrakOptions::default();
        options.render.unsafe_ = true;
        options.parse.smart = true;
        options
    };
}

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("{}", HELP.replacen("<cmd>", &args[0], 1));
        return Ok(());
    }

    match args[1].as_str() {
        "generate" => generate::run()?,
        "report" => report::run()?,
        _ => println!("Unknown command"),
    };

    Ok(())
}
