use std::env;

use anyhow::Result;

mod badge;
mod generate;
mod misc;

const CACHE_FILE: &str = "badge_cache.bin";
const BASE_PAGE: &str = "http://usscouts.org";
const MERIT_BADGE_HOME: &str = "http://usscouts.org/usscouts/meritbadges.asp";

const HELP: &str = r#"Usage: <cmd> <command>

Commands:
    generate - Generate info pages for outdated merit badges"#;

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("{}", HELP.replacen("<cmd>", &args[0], 1));
        return Ok(());
    }

    match args[1].as_str() {
        "generate" => generate::run()?,
        _ => println!("Unknown command"),
    };

    Ok(())
}
