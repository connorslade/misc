use std::{borrow::Cow, fs, ops::Range};

use anyhow::Context;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use lopdf::Document;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use splitter::Splitter;

mod splitter;

const INP_FILE: &str = r"V:\Downloads\MathYouMissed.pdf";
const OUT_DIR: &str = "output";

struct Section {
    pub level: usize,
    pub name: String,
    pub start: usize,
    pub end: usize,
}

struct SplitterJob {
    doc: Document,
    filename: String,
    pages: Range<usize>,
}

fn main() -> anyhow::Result<()> {
    fs::create_dir_all(OUT_DIR).context("Creating folder")?;

    let doc = Document::load(INP_FILE).context("Loading Document")?;
    let last_page = doc.get_pages().len();
    let toc = doc.get_toc().context("Getting TOC")?;

    let splitter = TestSplitter;

    let mut jobs: Vec<SplitterJob> = Vec::new();
    let mut prev = None;
    for i in &toc.toc {
        println!("{} {}", i.level, i.title);
        if let Some(j) = jobs.last_mut() {
            j.pages.end = i.page as usize;
        }

        let section = Section {
            level: i.level,
            name: i.title.clone(),
            start: i.page as usize,
            end: 0,
        };

        if splitter.should_split(&prev, &section) {
            let filename = format!("{}/{}.pdf", OUT_DIR, section.name);
            let pages = section.start..section.end;

            jobs.push(SplitterJob {
                doc: doc.clone(),
                filename,
                pages,
            });
        }

        prev = Some(section);
    }

    if let Some(j) = jobs.last_mut() {
        j.pages.end = last_page;
    }

    let job_count = jobs.len() as u64;
    // jobs.into_par_iter()
    //  .progress_count(job_count)
    jobs.into_iter().progress_count(job_count).for_each(|job| {
        let mut doc = job.doc;
        doc.delete_pages(&job.pages.map(|x| x as u32).collect::<Vec<_>>());
        if let Err(e) = doc.save(&job.filename) {
            eprintln!("Error saving {}: {}", job.filename, e);
        }
    });

    Ok(())
}

struct TestSplitter;

impl Splitter for TestSplitter {
    fn name<'a>(&self, section: &'a Section) -> Cow<'a, str> {
        Cow::Borrowed(&section.name)
    }

    fn should_split(&self, prev: &Option<Section>, section: &Section) -> bool {
        true
    }
}
