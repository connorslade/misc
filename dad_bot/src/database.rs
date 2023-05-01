use rusqlite::Connection;

trait Database {
    fn init(&mut self) -> anyhow::Result<()>;
    fn cleanup(&self) -> anyhow::Result<()>;
}

impl Database for Connection {
    fn init(&mut self) -> anyhow::Result<()> {
        self.pragma_update(None, "journal_mode", "WAL")?;
        self.pragma_update(None, "synchronous", "NORMAL")?;

        let trans = self.transaction()?;
        for i in [] {
            trans.execute(i, [])?;
        }
        trans.commit()?;

        Ok(())
    }

    fn cleanup(&self) -> anyhow::Result<()> {
        self.pragma_update(None, "wal_checkpoint", "TRUNCATE")?;
        Ok(())
    }
}
