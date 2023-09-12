use std::{borrow::Cow, cell::RefCell, collections::BTreeMap, fs, ops::Range};

use anyhow::Context;
use indicatif::ProgressIterator;
use lopdf::{Destination, Document, Outline};
use splitter::Splitter;

mod splitter;

const INP_FILE: &str =
    r"V:\Downloads\Ron Larson - Precalculus with Limits-Cengage Learning (2013).pdf";
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
    let total_pages = doc.page_iter().count();

    let splitter = TestSplitter::default();
    let mut jobs: Vec<SplitterJob> = Vec::new();

    let mut destinations = BTreeMap::new();
    let bookmarks = doc
        .get_outlines(None, None, &mut destinations)
        .context("Getting outlines")?
        .context("No Outlines")?;

    let mut outlines = Vec::new();
    for i in bookmarks {
        get_outlines(&mut outlines, &i, 0);
    }

    for i in outlines.iter().filter(|x| x.1 == 0) {
        let title = i.0.title().unwrap().as_string().unwrap();
        let reference = i.0.page().unwrap().as_reference().unwrap();
        let page = doc
            .page_iter()
            .position(|x| x == reference)
            .context("Page not found")?;

        if let Some(j) = jobs.last_mut() {
            j.pages.end = page;
        }

        let section = Section {
            level: i.1,
            name: title.into_owned(),
            start: page,
            end: 0,
        };

        if splitter.should_split(&section) {
            let filename = format!("{}/{}.pdf", OUT_DIR, section.name);
            let pages = section.start..section.end;

            jobs.push(SplitterJob {
                doc: doc.clone(),
                filename,
                pages,
            });
        }
    }

    if let Some(j) = jobs.last_mut() {
        j.pages.end = total_pages;
    }

    let job_count = jobs.len() as u64;
    // jobs.into_par_iter()
    //  .progress_count(job_count)
    jobs.into_iter().progress_count(job_count).for_each(|job| {
        let mut doc = Document::new();

        for i in job.pages {
            let page_id = job.doc.page_iter().nth(i).unwrap();
            let page = job.doc.get_page_content(page_id).unwrap();
            doc.add_page_contents(page_id, page).unwrap();
        }

        if let Err(e) = doc.save(&job.filename) {
            eprintln!("Error saving {}: {}", job.filename, e);
        }
    });

    Ok(())
}

fn get_outlines(outlines: &mut Vec<(Destination, usize)>, outline: &Outline, depth: usize) {
    match outline {
        Outline::Destination(dest) => {
            outlines.push((dest.clone(), depth));
        }
        Outline::SubOutlines(dest) => {
            for i in dest {
                get_outlines(outlines, i, depth + 1);
            }
        }
    }
}

#[derive(Default)]
struct TestSplitter {
    last_name: RefCell<Option<String>>,
}

impl Splitter for TestSplitter {
    fn name<'a>(&self, section: &'a Section) -> Cow<'a, str> {
        Cow::Borrowed(&section.name)
    }

    fn should_split(&self, section: &Section) -> bool {
        if section.name.starts_with("Ch ") {
            return true;
        }

        if let Some(last_name) = &*self.last_name.borrow() {
            if last_name.starts_with("Ch ") {
                return true;
            }
        }

        self.last_name.replace(Some(section.name.clone()));
        false
    }
}
