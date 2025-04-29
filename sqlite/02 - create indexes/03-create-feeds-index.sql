CREATE INDEX IF NOT EXISTS feeds_idx ON feeds (id);

CREATE INDEX IF NOT EXISTS entries_idx ON entries (id);

CREATE INDEX IF NOT EXISTS contents_idx ON contents (entry_id);
