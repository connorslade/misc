use std::{borrow::Cow, collections::BTreeMap, ops::Range, path::PathBuf, sync::Arc};

use anyhow::Context;
use lopdf::{Destination, Document, Outline};

pub trait Splitter {
    fn name<'a>(&self, section: &'a Section) -> Cow<'a, str>;
    fn should_split(&self, section: &Section) -> bool;
}

#[derive(Default)]
pub struct Section {
    pub special: Special,
    pub level: usize,
    pub name: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Default)]
pub enum Special {
    StartSlack,
    EndSlack,
    #[default]
    None,
}

pub struct SplitterJob {
    pub filename: PathBuf,
    pub pages: Range<usize>,
}

impl Section {
    fn as_job(&self, splitter: &dyn Splitter) -> SplitterJob {
        let filename = PathBuf::from(format!("{}.pdf", splitter.name(self)));
        let pages = self.start..self.end;

        SplitterJob { filename, pages }
    }
}

pub fn jobs(
    doc: Arc<Document>,
    splitter: &dyn Splitter,
    depth: usize,
) -> anyhow::Result<Vec<SplitterJob>> {
    let total_pages = doc.page_iter().count();
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

    // TODO: Split depth into splitter?
    for i in outlines.iter().filter(|x| x.1 == depth) {
        let title = i.0.title().unwrap().as_string().unwrap();
        let reference = i.0.page().unwrap().as_reference().unwrap();
        let page = doc
            .page_iter()
            .position(|x| x == reference)
            .context("Page not found")?;

        if let Some(i) = jobs.last_mut() {
            if i.pages.end == 0 {
                i.pages.end = page;
            }
        }

        let section = Section {
            special: Special::None,
            level: i.1,
            name: title.into_owned(),
            start: page,
            end: 0,
        };

        if splitter.should_split(&section) {
            if section.start != 0 && jobs.is_empty() {
                let section = Section {
                    special: Special::StartSlack,
                    level: i.1,
                    end: section.start,
                    ..Default::default()
                };
                jobs.push(section.as_job(splitter));
            }

            jobs.push(section.as_job(splitter));
        }
    }

    if let Some(i) = jobs.last() {
        if i.pages.end != total_pages {
            let section = Section {
                special: Special::EndSlack,
                level: 0,
                start: i.pages.end,
                end: total_pages,
                ..Default::default()
            };
            jobs.push(section.as_job(splitter));
        }
    }

    Ok(jobs)
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
