use std::fs;

use crate::completer::{completers::GPT3, Completer};

pub struct App {
    pub completer: Box<dyn Completer + Send + Sync + 'static>,
}

impl App {
    pub fn new() -> Self {
        let token = fs::read_to_string("./gpt3.key").unwrap();
        Self {
            completer: Box::new(GPT3::new(token.trim())),
        }
    }
}
