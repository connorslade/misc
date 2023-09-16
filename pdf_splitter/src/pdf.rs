use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc, sync::Arc};

use lopdf::{dictionary, Dictionary, Document, Object, ObjectId, Stream, StringFormat};

use crate::{splitter::SplitterJob, PRODUCER};

pub fn split(job: SplitterJob, old_doc: Arc<Document>, out_dir: &PathBuf) {
    let object_cache = Rc::new(RefCell::new(HashMap::new()));
    let mut doc = Document::new();
    let pages_id = doc.new_object_id();

    let mut pages = Vec::new();
    for i in job.pages {
        let page_id = old_doc.page_iter().nth(i).unwrap();
        let page = old_doc.get_page_content(page_id).unwrap();

        let content_id = doc.add_object(Stream::new(dictionary! {}, page));
        let mut dict = dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
        };

        let old_page = old_doc.get_dictionary(page_id).unwrap();
        if let Ok(resources) = old_page.get(b"Resources") {
            let this_doc = Rc::new(RefCell::new(doc));
            let resources = clone_obj(
                this_doc.clone(),
                old_doc.clone(),
                object_cache.clone(),
                resources.to_owned(),
            );
            doc = Rc::try_unwrap(this_doc).unwrap().into_inner();

            let resources_id = doc.add_object(resources);
            dict.set("Resources", resources_id);
        }

        for (key, value) in old_page.into_iter() {
            if dict.has(key) {
                continue;
            }

            let this_doc = Rc::new(RefCell::new(doc));
            let value = clone_obj(
                this_doc.clone(),
                old_doc.clone(),
                object_cache.clone(),
                value.to_owned(),
            );
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

    let old_root_catalog = old_doc
        .trailer
        .get(b"Root")
        .unwrap()
        .as_reference()
        .unwrap();
    let old_root_catalog = old_doc.get_object(old_root_catalog).unwrap();
    let old_metadata = old_root_catalog
        .as_dict()
        .unwrap()
        .get(b"Metadata")
        .unwrap();
    let old_metadata = old_doc
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
    if let Ok(i) = old_doc.trailer.get(b"Info") {
        let i = i.as_reference().unwrap();
        let i = old_doc.get_dictionary(i).unwrap();
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

    if let Err(e) = doc.save(out_dir.join(&job.filename)) {
        eprintln!("Error saving {:?}: {}", job.filename, e);
    }
}

// use hashmap to avoid including the same object twice
fn clone_obj(
    doc: Rc<RefCell<Document>>,
    old_doc: Arc<Document>,
    cache: Rc<RefCell<HashMap<ObjectId, ObjectId>>>,
    obj: Object,
) -> Object {
    match obj {
        Object::Array(array) => {
            let array = array
                .into_iter()
                .map(|x| clone_obj(doc.clone(), old_doc.clone(), cache.clone(), x))
                .collect::<Vec<_>>();
            Object::Array(array)
        }
        Object::Dictionary(dict) => {
            let mut new_dict = Dictionary::new();
            for (key, value) in dict.into_iter() {
                let value = clone_obj(
                    doc.clone(),
                    old_doc.clone(),
                    cache.clone(),
                    value.to_owned(),
                );
                new_dict.set(key.to_owned(), value);
            }
            Object::Dictionary(new_dict)
        }
        Object::Stream(stream) => {
            let mut new_dict = Dictionary::new();
            for (key, value) in stream.dict.iter() {
                let value = clone_obj(
                    doc.clone(),
                    old_doc.clone(),
                    cache.clone(),
                    value.to_owned(),
                );
                new_dict.set(key.to_owned(), value);
            }
            let stream =
                Stream::new(new_dict, stream.content).with_compression(stream.allows_compression);
            Object::Stream(stream)
        }
        Object::Reference(id) => {
            // Only add objects once per document
            // On one document I got an 88% cache rate
            if let Some(new_id) = cache.borrow().get(&id) {
                return Object::Reference(*new_id);
            }

            let old_obj = match old_doc.get_object(id) {
                Ok(i) => i.to_owned(),
                Err(_) => return Object::Null,
            };

            let obj = clone_obj(doc.clone(), old_doc, cache.clone(), old_obj);
            let obj = doc.borrow_mut().add_object(obj);
            cache.borrow_mut().insert(id, obj);
            Object::Reference(obj)
        }
        Object::Null
        | Object::Boolean(_)
        | Object::Integer(_)
        | Object::Real(_)
        | Object::Name(_)
        | Object::String(..) => obj,
    }
}
