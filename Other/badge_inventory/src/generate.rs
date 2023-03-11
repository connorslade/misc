use std::{fs, path::Path};

use crate::badge::load_badges;
use crate::misc::best;

use anyhow::Result;
use comrak::{markdown_to_html, ComrakOptions};
use indicatif::ParallelProgressIterator;
use lazy_static::lazy_static;
use rayon::prelude::*;

lazy_static! {
    static ref COMRAK_OPTIONS: ComrakOptions = {
        let mut options = ComrakOptions::default();
        options.render.unsafe_ = true;
        options.parse.smart = true;
        options
    };
}

const OUT_DIR: &str = "out_md";
const OWNED_FILE: &str = "owned.csv";

pub fn run() -> Result<()> {
    let out_dir = Path::new(OUT_DIR);
    if !out_dir.exists() {
        fs::create_dir(out_dir)?;
    }

    if fs::read_dir(out_dir)?.count() > 0 {
        println!("Output directory is not empty, exiting");
        return Ok(());
    }

    // (book, date)
    let owned = fs::read_to_string(OWNED_FILE)?;
    let owned = owned
        .lines()
        .skip(1)
        .filter_map(|x| x.split_once(','))
        .collect::<Vec<_>>();

    println!("[*] Loading Badges");
    let badges = load_badges()?;

    println!("[*] Writing Markdown");
    owned.par_iter().progress().for_each(|x| {
        let date = x.1.parse::<u16>().unwrap();
        let badge = x.0.to_lowercase();
        let badge = best(&badge, &badges, |x| x.name.to_owned()).unwrap();
        if date >= badge.update_date {
            return;
        }

        let out = include_str!("./template.md")
            .replace("{{TITLE}}", &badge.name)
            .replace("{{IMAGE_LINK}}", &badge.icon_link)
            .replace("{{BOOK_DATE}}", date.to_string().as_str())
            .replace("{{UPDATE_DATE}}", badge.update_date.to_string().as_str())
            .replace("{{REQUIREMENTS}}", &badge.requirements);

        let rendered = markdown_to_html(&out, &COMRAK_OPTIONS);
        let mut path = out_dir.join(format!("{}-{}.html", badge.name.replace(' ', "-"), date));
        let mut i = 1;
        while path.exists() {
            path.set_file_name(format!(
                "{}-{}-{}.html",
                badge.name.replace(' ', "-"),
                date,
                i
            ));
            i += 1;
        }
        fs::write(path, rendered).unwrap();
    });

    println!("[*] Complete");
    Ok(())
}
