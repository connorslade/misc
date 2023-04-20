use std::fs;

use parking_lot::{Mutex, MutexGuard};
use rusqlite::Connection;

use crate::{
    completer::{completers::OpenAI, Completer},
    database::Database,
};

pub struct App {
    pub completer: Box<dyn Completer + Send + Sync + 'static>,
    database: Mutex<Connection>,
}

impl App {
    pub fn new() -> Self {
        let token = fs::read_to_string("./openai.key").unwrap();
        let mut db = Connection::open("./data.db").unwrap();
        db.init();

        Self {
            completer: Box::new(OpenAI::new(token.trim())),
            database: Mutex::new(db),
        }
    }

    pub fn db(&self) -> MutexGuard<Connection> {
        self.database.lock()
    }
}
