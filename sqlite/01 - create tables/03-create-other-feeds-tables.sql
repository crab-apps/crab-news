-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Person.html
CREATE TABLE IF NOT EXISTS feed_persons (
    feed_id TEXT REFERENCES feeds (id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    email TEXT,
    uri TEXT,
    role TEXT CHECK (role IN ('author', 'contributor'))
);

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Link.html
CREATE TABLE IF NOT EXISTS feed_links (
    feed_id TEXT REFERENCES feeds (id) ON DELETE CASCADE,
    href TEXT NOT NULL,
    rel TEXT,
    media_type TEXT,
    hreflang TEXT,
    title TEXT,
    length INTEGER,
);

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
    version TEXT,
);

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Category.html
CREATE TABLE IF NOT EXISTS feed_categories (
    id SERIAL PRIMARY KEY,
    feed_id TEXT REFERENCES feeds (id) ON DELETE CASCADE,
    term TEXT NOT NULL,
    scheme TEXT,
    label TEXT,
);

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Category.html
CREATE TABLE IF NOT EXISTS feed_subcategories (
    category_id INTEGER REFERENCES feed_categories (id) ON DELETE CASCADE,
    term TEXT NOT NULL,
    scheme TEXT,
    label TEXT,
);
