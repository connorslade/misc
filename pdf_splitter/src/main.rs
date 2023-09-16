use std::{fs, sync::Arc};

use anyhow::Context;
use args::Args;
use clap::Parser;
use indicatif::ParallelProgressIterator;
use lopdf::Document;
use rayon::prelude::{ParallelBridge, ParallelIterator};

mod args;
mod splitter;
use splitter::{jobs, split};

const PRODUCER: &[u8] = b"pdf_splitter by Connor Slade [https://github.com/Basicprogrammer10/misc/tree/main/pdf_splitter]";

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!(
        "[*] Loading Document `{}`",
        args.input_file.to_string_lossy()
    );
    let doc = Arc::new(Document::load(&args.input_file).context("Loading Document")?);
    let jobs = jobs(doc.clone(), &args)?;

    if args.dry_run {
        println!("[*] Dry run, not saving files");
        for job in jobs {
            println!(" | {}", job.filename.to_string_lossy());
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
        .for_each(|x| split(x, &args.output_dir));

    Ok(())
}

// #[derive(Default)]
// struct TestSplitter {
//     last_name: RefCell<Option<String>>,
// }

// impl Splitter for TestSplitter {
//     fn name<'a>(&self, section: &'a Section) -> Cow<'a, str> {
//         let regex = Regex::new(r"Ch (\d+): (.*)").unwrap();

//         if let Some(caps) = regex.captures(&section.name) {
//             let chapter = caps.get(1).unwrap().as_str();
//             let title = caps.get(2).unwrap().as_str();

//             return Cow::Owned(format!("Ch{}-P{}-{}", chapter, section.start, title));
//         }

//         Cow::Owned(section.name.replace(' ', "-").replace(':', ""))
//     }

//     fn should_split(&self, section: &Section) -> bool {
//         let mut res = false;

//         if section.name.starts_with("Ch ") {
//             res = true;
//         }

//         if let Some(last_name) = &*self.last_name.borrow() {
//             if last_name.starts_with("Ch ") {
//                 res = true;
//             }
//         }

//         self.last_name.replace(Some(section.name.clone()));
//         res
//     }
// }
