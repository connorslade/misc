use rusqlite::{params, Connection};

use crate::completer::Completion;

pub trait Database {
    // == Base ==
    fn init(&mut self);
    fn cleanup(&self);

    // == Completions ==
    fn get_completion(&self, path: &str) -> Option<Completion>;
    fn set_completion(&self, path: &str, completion: &Completion);
}

impl Database for Connection {
    fn init(&mut self) {
        self.pragma_update(None, "journal_mode", "WAL").unwrap();
        self.pragma_update(None, "synchronous", "NORMAL").unwrap();

        self.execute(include_str!("./sql/create_completions.sql"), [])
            .unwrap();
    }

    fn cleanup(&self) {
        self.pragma_update(None, "wal_checkpoint", "TRUNCATE")
            .unwrap();
    }

    fn get_completion(&self, path: &str) -> Option<Completion> {
        self.query_row(
            "SELECT content, type FROM completions WHERE path = ?",
            [path],
            |row| {
                Ok(Completion {
                    content_type: row.get(1)?,
                    body: row.get::<_, String>(0)?.as_bytes().to_vec(),
                })
            },
        )
        .ok()
    }

    fn set_completion(&self, path: &str, completion: &Completion) {
        self.execute(
            "INSERT INTO completions (path, content, type) VALUES (?, ?, ?)",
            params![
                path,
                String::from_utf8_lossy(&completion.body),
                completion.content_type,
            ],
        )
        .unwrap();
    }
}
