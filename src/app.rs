use std::fs;

use crate::completer::{completers::OpenAI, Completer};

pub struct App {
    pub completer: Box<dyn Completer + Send + Sync + 'static>,
}

impl App {
    pub fn new() -> Self {
        let token = fs::read_to_string("./openai.key").unwrap();
        Self {
            completer: Box::new(OpenAI::new(token.trim())),
        }
    }
}
