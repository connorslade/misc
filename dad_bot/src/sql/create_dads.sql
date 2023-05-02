CREATE TABLE IF NOT EXISTS dads (
    -- Info about the user who replied 'im dad'
    dad_user INTEGER NOT NULL,
    dad_message INTEGER NOT NULL,

    -- Info about the user who was replied to
    daded_user INTEGER NOT NULL,
    daded_message INTEGER NOT NULL,
    
    -- Common info
    channel_id INTEGER NOT NULL,
    guild_id INTEGER NOT NULL,
    date INTEGER NOT NULL
)