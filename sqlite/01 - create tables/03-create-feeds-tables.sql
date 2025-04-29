-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Feed.html
CREATE TABLE IF NOT EXISTS feeds (
    id TEXT UNIQUE NOT NULL,
    account_id INTEGER REFERENCES accounts (id) ON DELETE CASCADE,
    parent_folder VARCHAR(50) REFERENCES folders (name) ON DELETE CASCADE,
    title VARCHAR(300) NOT NULL,
    home VARCHAR(300) NOT NULL,
    logo VARCHAR(400) DEFAULT '/placeholder.png',
    icon VARCHAR(400) DEFAULT '/placeholder.png',
    published TEXT NOT NULL,
    updated TEXT NOT NULL,
    authors TEXT NOT NULL,
    description TEXT,
    language TEXT,
    rights TEXT,
    ttl INTEGER,
    feed_type VARCHAR(15) CHECK (
        feed_type IN ('Atom', 'JSON', 'RSS0', 'RSS1', 'RSS2')
    ),
    PRIMARY KEY (account_id, id)
);

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Entry.html
CREATE TABLE IF NOT EXISTS entries (
    id TEXT PRIMARY KEY,
    feed_id TEXT REFERENCES feeds (id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    authors TEXT NOT NULL,
    published TEXT NOT NULL,
    updated TEXT NOT NULL,
    summary TEXT NOT NULL,
    read_status VARCHAR(10) CHECK (read_status IN ('unread', 'read')) DEFAULT 'unread',
    starred_status VARCHAR(10) CHECK (starred_status IN ('unstarred', 'starred')) DEFAULT 'unstarred',
    source TEXT,
    rights TEXT,
    language TEXT,
    base TEXT
);

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Content.html
CREATE TABLE IF NOT EXISTS contents (
    entry_id TEXT REFERENCES entries (id) ON DELETE CASCADE,
    content_body TEXT,
    content_type VARCHAR(100) NOT NULL,
    content_length INTEGER CHECK (content_length >= 0),
    source_link TEXT
);
