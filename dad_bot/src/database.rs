use rusqlite::{params, Connection, Error};
use serenity::model::prelude::{ChannelId, GuildId, Message, MessageId, UserId};

pub trait Database {
    // == Base ==
    fn init(&mut self) -> anyhow::Result<()>;
    fn cleanup(&self) -> anyhow::Result<()>;

    // == Dad ==
    fn add_dadable(&self, msg: &Message) -> anyhow::Result<()>;
    fn get_dadable(&self, guild_id: u64, channel_id: u64) -> anyhow::Result<Option<Dadable>>;
}

impl Database for Connection {
    fn init(&mut self) -> anyhow::Result<()> {
        self.pragma_update(None, "journal_mode", "WAL")?;
        self.pragma_update(None, "synchronous", "NORMAL")?;

        let trans = self.transaction()?;
        for i in [
            include_str!("./sql/create_dads.sql"),
            include_str!("./sql/create_dadable.sql"),
        ] {
            trans.execute(i, [])?;
        }
        trans.commit()?;

        Ok(())
    }

    fn cleanup(&self) -> anyhow::Result<()> {
        self.pragma_update(None, "wal_checkpoint", "TRUNCATE")?;
        Ok(())
    }

    fn add_dadable(&self, msg: &Message) -> anyhow::Result<()> {
        self.execute(
            "INSERT INTO dadable VALUES (?, ?, ?, ?, strftime('%s','now'))",
            params![
                msg.guild_id.unwrap_or_default().0,
                msg.channel_id.0,
                msg.id.0,
                msg.author.id.0
            ],
        )?;
        Ok(())
    }

    fn get_dadable(&self, guild_id: u64, channel_id: u64) -> anyhow::Result<Option<Dadable>> {
        let query = self.query_row(
                "SELECT * FROM dadable WHERE guild_id = ? AND channel_id = ? ORDER BY date DESC LIMIT 1",
                params![guild_id, channel_id],
                |row| {
                    Ok(Dadable {
                        guild_id: GuildId(row.get(0)?),
                        channel_id: ChannelId(row.get(1)?),
                        message_id: MessageId(row.get(2)?),
                        author_id: UserId(row.get(3)?),
                        timestamp: row.get(4)?,
                    })
                },
            );

        match query {
            Ok(dadable) => Ok(Some(dadable)),
            Err(Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

pub struct Dadable {
    pub guild_id: GuildId,
    pub channel_id: ChannelId,
    pub message_id: MessageId,
    pub author_id: UserId,
    pub timestamp: u64,
}
