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
  content_length INTEGER CHECK (content_length>=0),
  source_link TEXT
);

CREATE TABLE IF NOT EXISTS entry_persons (
  entry_id TEXT REFERENCES entries (id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  email TEXT,
  uri TEXT,
  role TEXT CHECK (role IN ('author', 'contributor')) DEFAULT 'author'
);

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Link.html
CREATE TABLE IF NOT EXISTS entry_links (
  entry_id TEXT REFERENCES entries (id) ON DELETE CASCADE,
  href TEXT NOT NULL,
  rel TEXT,
  media_type TEXT,
  hreflang TEXT,
  title TEXT,
  length INTEGER
);

CREATE INDEX IF NOT EXISTS feeds_idx ON feeds (id);

CREATE INDEX IF NOT EXISTS entries_idx ON entries (id);

CREATE INDEX IF NOT EXISTS contents_idx ON contents (entry_id);

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Person.html
-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.MediaRating.html
CREATE TABLE IF NOT EXISTS feed_media_ratings (
  feed_id TEXT REFERENCES feeds (id) ON DELETE CASCADE,
  urn TEXT NOT NULL,
  value TEXT NOT NULL
);

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Generator.html
CREATE TABLE IF NOT EXISTS feed_generators (
  feed_id TEXT REFERENCES feeds (id) ON DELETE CASCADE,
  content TEXT NOT NULL,
  uri TEXT,
  version TEXT
);

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Category.html
CREATE TABLE IF NOT EXISTS feed_categories (
  id SERIAL PRIMARY KEY,
  feed_id TEXT REFERENCES feeds (id) ON DELETE CASCADE,
  term TEXT NOT NULL,
  scheme TEXT,
  label TEXT
);

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Category.html
CREATE TABLE IF NOT EXISTS feed_subcategories (
  category_id INTEGER REFERENCES feed_categories (id) ON DELETE CASCADE,
  term TEXT NOT NULL,
  scheme TEXT,
  label TEXT
);
