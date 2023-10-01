use std::{borrow::Cow, fs, sync::Arc};

use anyhow::Context;
use args::Args;
use clap::Parser;
use indicatif::ParallelProgressIterator;
use lopdf::Document;
use rayon::prelude::{ParallelBridge, ParallelIterator};

mod args;
mod pdf;
mod splitter;
use splitter::{jobs, Section, Special, Splitter};

const PRODUCER: &[u8] = b"pdf_splitter by Connor Slade [https://github.com/Basicprogrammer10/misc/tree/main/pdf_splitter]";

fn main() -> anyhow::Result<()> {
    let args = Arc::new(Args::parse());

    println!(
        "[*] Loading Document `{}`",
        args.input_file.to_string_lossy()
    );
    let doc = Arc::new(Document::load(&args.input_file).context("Loading Document")?);
    let splitter = ArgSplitter { args: args.clone() };
    let jobs = jobs(doc.clone(), &splitter, args.depth)?;

    if args.dry_run {
        println!("[*] Dry run, not saving files");
        for job in jobs {
            println!(
                " | [{:0>3}-{:0>3}] {}",
                job.pages.start,
                job.pages.end,
                job.filename.to_string_lossy()
            );
        }
        return Ok(());
    }

    println!(
        "[*] Creating output directory `{}`",
        args.output_dir.to_string_lossy()
    );
    fs::create_dir_all(&args.output_dir).context("Creating folder")?;

    let job_count = jobs.len() as u64;
    jobs.into_iter()
        .par_bridge()
        .progress_count(job_count)
        .for_each(|x| pdf::split(x, doc.clone(), &args.output_dir));

    Ok(())
}

struct ArgSplitter {
    args: Arc<Args>,
}

impl Splitter for ArgSplitter {
    fn name<'a>(&self, section: &'a Section) -> Cow<'a, str> {
        match section.special {
            Special::StartSlack => return Cow::Owned(self.args.start_name.to_owned()),
            Special::EndSlack => return Cow::Owned(self.args.end_name.to_owned()),
            _ => {}
        }

        let mut name = self
            .args
            .rename_captures
            .replace_all(&section.name, &self.args.rename_format);

        if !self.args.allow_unchecked {
            name = Cow::Owned(name.replace(' ', "_").replace(':', ""));
        }

        name
    }

    fn should_split(&self, section: &Section) -> bool {
        self.args.should_split.is_match(&section.name)
    }
}
