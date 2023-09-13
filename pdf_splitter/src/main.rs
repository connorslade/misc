use std::{
    borrow::Cow,
    cell::{Ref, RefCell},
    collections::BTreeMap,
    fs,
    ops::Range,
    path::PathBuf,
    sync::Arc,
};

use anyhow::Context;
use indicatif::ProgressIterator;
use lopdf::{dictionary, Destination, Dictionary, Document, Object, Outline, Stream, StringFormat};
use regex::Regex;
use splitter::Splitter;

mod splitter;

const INP_FILE: &str = r"precalc.pdf";
const OUT_DIR: &str = "./output/";

struct Section {
    pub level: usize,
    pub name: String,
    pub start: usize,
    pub end: usize,
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
    // jobs.into_par_iter()
    //  .progress_count(job_count)
    jobs.into_iter().progress_count(job_count).for_each(|job| {
        let mut doc = Document::new();
        let pages_id = doc.new_object_id();

        // == todo
        // let font_id = doc.add_object(dictionary! {
        //     // type of dictionary
        //     "Type" => "Font",
        //     // type of font, type1 is simple postscript font
        //     "Subtype" => "Type1",
        //     // basefont is postscript name of font for type1 font.
        //     // See PDF reference document for more details
        //     "BaseFont" => "Courier",
        // });

        // let resources_id = doc.add_object(dictionary! {
        //     // fonts are actually triplely nested dictionaries. Fun!
        //     "Font" => dictionary! {
        //         // F1 is the font name used when writing text.
        //         // It must be unique in the document. It does not
        //         // have to be F1
        //         "F1" => font_id,
        //     },
        // });
        // ==

        let mut pages = Vec::new();
        for i in job.pages {
            let page_id = job.doc.page_iter().nth(i).unwrap();
            let page = job.doc.get_page_content(page_id).unwrap();


            let resources = job.doc.get_dictionary(page_id).unwrap().get(b"Resources").unwrap().to_owned();
           let resources = clone_obj_inner(RefCell::new(doc.clone()), resources).unwrap();
            
            let resources_id = doc.add_object(resources);
            let content_id = doc.add_object(Stream::new(dictionary! {}, page));
            let mut dict = dictionary! {
                "Type" => "Page",
                "Parent" => pages_id,
                "Contents" => content_id,
                "Resources" => resources_id,
            };

            if let Some(i) = job.doc.get_page_resources(page_id).0 {
                for (key, value) in i.iter() {
                    if dict.has(key) {
                        continue;
                    }

                    println!("{}: {:?}", String::from_utf8_lossy(key), value);
                    dict.set(key.to_owned(), value.to_owned());
                }
            }

            let page_id = doc.add_object(dict);
            pages.push(page_id.into());
        }

        let pages = dictionary! {
            "Type" => "Pages",
            "Count" => pages.len() as u32,
            "Kids" => pages,
            // "Resources" => resources_id,
            // a rectangle that defines the boundaries of the physical or digital media. This is the
            // "Page Size"
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
        };

        doc.objects.insert(pages_id, Object::Dictionary(pages));

        dbg!(&job.doc.trailer);
        let old_root_catalog = job.doc.trailer.get(b"Root").unwrap().as_reference().unwrap();
        let old_root_catalog = job.doc.get_object(old_root_catalog).unwrap();
        let old_metadata = old_root_catalog
            .as_dict()
            .unwrap()
            .get(b"Metadata")
            .unwrap();
        let old_metadata = job.doc.get_object(old_metadata.as_reference().unwrap()).unwrap().to_owned();
        let metadata = doc.add_object(dbg!(old_metadata));
        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
            "Metadata" => metadata,
        });

    
        let mut info = dictionary! {
            "Type" => "Info",
            "Producer" => Object::String(b"pdf_splitter by Connor Slade [https://github.com/Basicprogrammer10/misc/tree/main/pdf_splitter]".to_vec(), StringFormat::Literal),
            // ^ todo fix
        };
        if let Ok(i) = job.doc.trailer.get(b"Info") {
            let i = i.as_reference().unwrap();
            let i = job.doc.get_object(i).unwrap().as_dict().unwrap();
            for (key, value) in i.iter() {
                if info.has(key) {
                    continue;
                }

                info.set(key.to_owned(), value.to_owned());
            }
        }
        let info_id = doc.add_object(info);

        doc.trailer.set("Root", catalog_id);
        doc.trailer.set("Info", info_id);
        // doc.compress();

        if let Err(e) = doc.save(&job.filename) {
            eprintln!("Error saving {:?}: {}", job.filename, e);
        }
        ::std::process::exit(0);
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

fn clone_obj(doc: &Document, obj: impl Fn(&Document) -> &Object) -> anyhow::Result<Object> {
    let obj = obj(doc).to_owned();
    let doc = RefCell::new(doc.clone());
    clone_obj_inner(doc, obj)
}

fn clone_obj_inner(doc: RefCell<Document>, obj: Object) -> anyhow::Result<Object> {
    if let Object::Reference(id) = obj {
        let old_obj = doc.borrow().get_object(id).unwrap().to_owned();
        let obj = clone_obj_inner(doc.clone(), old_obj)?;
        let obj = doc.borrow_mut().add_object(obj);
        return Ok(Object::Reference(obj));
    }

    Ok(obj)
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

            return Cow::Owned(format!("Ch{}-{}", chapter, title));
        }

        Cow::Owned(section.name.replace(' ', "-"))
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
