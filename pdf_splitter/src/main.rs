use std::{
    borrow::Cow, cell::RefCell, collections::BTreeMap, fs, ops::Range, path::PathBuf, rc::Rc,
    sync::Arc,
};

use anyhow::Context;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use lopdf::{dictionary, Destination, Dictionary, Document, Object, Outline, Stream, StringFormat};
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use regex::Regex;
use splitter::Splitter;

mod splitter;

const PRODUCER: &[u8] = b"pdf_splitter by Connor Slade [https://github.com/Basicprogrammer10/misc/tree/main/pdf_splitter]";
const INP_FILE: &str =
    r"V:\Downloads\Ron Larson - Precalculus with Limits-Cengage Learning (2013).pdf";
const OUT_DIR: &str = "./output/";

struct Section {
    pub special: Special,
    pub level: usize,
    pub name: String,
    pub start: usize,
    pub end: usize,
}

enum Special {
    StartSlack,
    EndSlack,
    None,
}

struct SplitterJob {
    doc: Arc<Document>,
    filename: PathBuf,
    pages: Range<usize>,
}

fn main() -> anyhow::Result<()> {
    fs::create_dir_all(OUT_DIR).context("Creating folder")?;

    let doc = Arc::new(Document::load(INP_FILE).context("Loading Document")?);
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

        if jobs.len() > 1 {
            jobs.last_mut().unwrap().pages.end = page;
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
                let title_section = Section {
                    special: Special::StartSlack,
                    level: 0,
                    name: "Title".to_owned(),
                    start: 0,
                    end: section.start,
                };
                let filename =
                    PathBuf::from(OUT_DIR).join(format!("{}.pdf", splitter.name(&title_section)));

                jobs.push(SplitterJob {
                    doc: doc.clone(),
                    filename,
                    pages: 0..section.start,
                });
            }

            let filename = PathBuf::from(OUT_DIR).join(format!("{}.pdf", splitter.name(&section)));
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
    jobs.into_iter()
        .par_bridge()
        .progress_count(job_count)
        .for_each(|job| {
            let mut doc = Document::new();
            let pages_id = doc.new_object_id();

            let mut pages = Vec::new();
            for i in job.pages {
                let page_id = job.doc.page_iter().nth(i).unwrap();
                let page = job.doc.get_page_content(page_id).unwrap();

                let content_id = doc.add_object(Stream::new(dictionary! {}, page));
                let mut dict = dictionary! {
                    "Type" => "Page",
                    "Parent" => pages_id,
                    "Contents" => content_id,
                };

                let old_page = job.doc.get_dictionary(page_id).unwrap();
                if let Ok(resources) = old_page.get(b"Resources") {
                    let this_doc = Rc::new(RefCell::new(doc));
                    let resources =
                        clone_obj(this_doc.clone(), job.doc.clone(), resources.to_owned());
                    doc = Rc::try_unwrap(this_doc).unwrap().into_inner();

                    let resources_id = doc.add_object(resources);
                    dict.set("Resources", resources_id);
                }

                for (key, value) in old_page.into_iter() {
                    if dict.has(key) {
                        continue;
                    }

                    let this_doc = Rc::new(RefCell::new(doc));
                    let value = clone_obj(this_doc.clone(), job.doc.clone(), value.to_owned());
                    doc = Rc::try_unwrap(this_doc).unwrap().into_inner();

                    dict.set(key.to_owned(), value);
                }

                let page_id = doc.add_object(dict);
                pages.push(page_id.into());
            }

            doc.objects.insert(
                pages_id,
                Object::Dictionary(dictionary! {
                    "Type" => "Pages",
                    "Count" => pages.len() as u32,
                    "Kids" => pages,
                }),
            );

            let old_root_catalog = job
                .doc
                .trailer
                .get(b"Root")
                .unwrap()
                .as_reference()
                .unwrap();
            let old_root_catalog = job.doc.get_object(old_root_catalog).unwrap();
            let old_metadata = old_root_catalog
                .as_dict()
                .unwrap()
                .get(b"Metadata")
                .unwrap();
            let old_metadata = job
                .doc
                .get_object(old_metadata.as_reference().unwrap())
                .unwrap()
                .to_owned();
            let metadata = doc.add_object(old_metadata);
            let catalog_id = doc.add_object(dictionary! {
                "Type" => "Catalog",
                "Pages" => pages_id,
                "Metadata" => metadata,
            });

            let mut info = dictionary! {
                "Type" => "Info",
            };
            if let Ok(i) = job.doc.trailer.get(b"Info") {
                let i = i.as_reference().unwrap();
                let i = job.doc.get_dictionary(i).unwrap();
                for (key, value) in i.iter() {
                    if info.has(key) {
                        continue;
                    }

                    info.set(key.to_owned(), value.to_owned());
                }
            }
            info.set(
                "Producer",
                Object::String(PRODUCER.to_vec(), StringFormat::Literal),
            );
            let info_id = doc.add_object(info);

            doc.trailer.set("Root", catalog_id);
            doc.trailer.set("Info", info_id);
            doc.compress();

            if let Err(e) = doc.save(&job.filename) {
                eprintln!("Error saving {:?}: {}", job.filename, e);
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

// use hashmap to avoid including the same object twice
fn clone_obj(doc: Rc<RefCell<Document>>, old_doc: Arc<Document>, obj: Object) -> Object {
    match obj {
        Object::Array(array) => {
            let array = array
                .into_iter()
                .map(|x| clone_obj(doc.clone(), old_doc.clone(), x))
                .collect::<Vec<_>>();
            Object::Array(array)
        }
        Object::Dictionary(dict) => {
            let mut new_dict = Dictionary::new();
            for (key, value) in dict.into_iter() {
                let value = clone_obj(doc.clone(), old_doc.clone(), value.to_owned());
                new_dict.set(key.to_owned(), value);
            }
            Object::Dictionary(new_dict)
        }
        Object::Stream(stream) => {
            let mut new_dict = Dictionary::new();
            for (key, value) in stream.dict.iter() {
                let value = clone_obj(doc.clone(), old_doc.clone(), value.to_owned());
                new_dict.set(key.to_owned(), value);
            }
            let stream =
                Stream::new(new_dict, stream.content).with_compression(stream.allows_compression);
            Object::Stream(stream)
        }
        Object::Reference(id) => {
            let old_obj = match old_doc.get_object(id) {
                Ok(i) => i.to_owned(),
                Err(_) => return Object::Null,
            };

            let obj = clone_obj(doc.clone(), old_doc, old_obj);
            let obj = doc.borrow_mut().add_object(obj);
            return Object::Reference(obj);
        }
        Object::Null
        | Object::Boolean(_)
        | Object::Integer(_)
        | Object::Real(_)
        | Object::Name(_)
        | Object::String(..) => obj,
    }
}

#[derive(Default)]
struct TestSplitter {
    last_name: RefCell<Option<String>>,
}

impl Splitter for TestSplitter {
    fn name<'a>(&self, section: &'a Section) -> Cow<'a, str> {
        let regex = Regex::new(r"Ch (\d+): (.*)").unwrap();

        if let Some(caps) = regex.captures(&section.name) {
            let chapter = caps.get(1).unwrap().as_str();
            let title = caps.get(2).unwrap().as_str();

            return Cow::Owned(format!("Ch{}-P{}-{}", chapter, section.start, title));
        }

        Cow::Owned(section.name.replace(' ', "-").replace(':', ""))
    }

    fn should_split(&self, section: &Section) -> bool {
        let mut res = false;

        if section.name.starts_with("Ch ") {
            res = true;
        }

        if let Some(last_name) = &*self.last_name.borrow() {
            if last_name.starts_with("Ch ") {
                res = true;
            }
        }

        self.last_name.replace(Some(section.name.clone()));
        res
    }
}
