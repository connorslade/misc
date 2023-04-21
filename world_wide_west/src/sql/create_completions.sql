CREATE TABLE IF NOT EXISTS completions (
    path TEXT NOT NULL UNIQUE,
    content TEXT NOT NULL,
    type TEXT NOT NULL,
    tokens INTEGER NOT NULL,
    date INTEGER NOT NULL
)