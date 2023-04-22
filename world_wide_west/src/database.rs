use rusqlite::{params, Connection};

use crate::completer::Completion;

pub trait Database {
    // == Base ==
    fn init(&mut self);

    // == Completions ==
    fn get_completion(&self, path: &str) -> Option<Completion>;
    fn set_completion(&self, path: &str, completion: &Completion);
}

impl Database for Connection {
    fn init(&mut self) {
        self.execute(include_str!("./sql/create_completions.sql"), [])
            .unwrap();
    }

    fn get_completion(&self, path: &str) -> Option<Completion> {
        self.query_row(
            "SELECT content, type, tokens FROM completions WHERE path = ? ORDER BY date DESC LIMIT 1",
            [path],
            |row| {
                Ok(Completion {
                    content_type: row.get(1)?,
                    body: row.get::<_, String>(0)?.as_bytes().to_vec(),
                    tokens: row.get(2)?,
                })
            },
        )
        .ok()
    }

    fn set_completion(&self, path: &str, completion: &Completion) {
        self.execute(
            "INSERT INTO completions VALUES (?, ?, ?, ?, strftime('%s','now'))",
            params![
                path,
                String::from_utf8_lossy(&completion.body),
                completion.content_type,
                completion.tokens
            ],
        )
        .unwrap();
    }
}
