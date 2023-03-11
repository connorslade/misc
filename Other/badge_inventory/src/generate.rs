use std::{fs, path::Path};

use crate::badge::{load_badges, load_owned};
use crate::misc::best;
use crate::COMRAK_OPTIONS;

use anyhow::Result;
use comrak::markdown_to_html;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

const OUT_DIR: &str = "out_md";

pub fn run() -> Result<()> {
    let out_dir = Path::new(OUT_DIR);
    if !out_dir.exists() {
        fs::create_dir(out_dir)?;
    }

    if fs::read_dir(out_dir)?.count() > 0 {
        println!("Output directory is not empty, exiting");
        return Ok(());
    }

    println!("[*] Loading Badges");
    let owned = load_owned()?;
    let badges = load_badges()?;

    println!("[*] Writing Markdown");
    owned.par_iter().progress().for_each(|x| {
        let badge = x.name.to_lowercase();
        let badge = best(&badge, &badges, |x| x.name.to_owned()).unwrap();
        if x.date >= badge.update_date {
            return;
        }

        let out = include_str!("./templates/generate.md")
            .replace("{{TITLE}}", &badge.name)
            .replace("{{IMAGE_LINK}}", &badge.icon_link)
            .replace("{{BOOK_DATE}}", x.date.to_string().as_str())
            .replace("{{UPDATE_DATE}}", badge.update_date.to_string().as_str())
            .replace("{{REQUIREMENTS}}", &badge.requirements);

        let rendered = markdown_to_html(&out, &COMRAK_OPTIONS);
        let mut path = out_dir.join(format!("{}-{}.html", badge.name.replace(' ', "-"), x.date));
        let mut i = 1;
        while path.exists() {
            path.set_file_name(format!(
                "{}-{}-{}.html",
                badge.name.replace(' ', "-"),
                x.date,
                i
            ));
            i += 1;
        }
        fs::write(path, rendered).unwrap();
    });

    println!("[*] Complete");
    Ok(())
}
