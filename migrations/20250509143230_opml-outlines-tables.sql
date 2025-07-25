-- folder_name is used in Outline both title and text types. <outline text="Group Name" title="Group Name">
-- folder_name can be NULL to match Rust's None == feed placed in Account's root
CREATE TABLE IF NOT EXISTS folders (
  account_id INTEGER REFERENCES accounts (id) ON DELETE CASCADE,
  name VARCHAR(50) UNIQUE DEFAULT NULL,
  PRIMARY KEY (account_id, name)
);

-- <outline text="Feed Name" type="rss" xmlUrl="https://example.com/rss.xml"
-- description="" htmlUrl="https://example.com/" title="Feed Name" version="RSS"/>
CREATE TABLE IF NOT EXISTS subscriptions (
  account_id INTEGER REFERENCES accounts (id) ON DELETE CASCADE,
  parent_folder VARCHAR(50) REFERENCES folders (name) ON DELETE CASCADE,
  xml_url VARCHAR(255) UNIQUE NOT NULL,
  html_url VARCHAR(255) UNIQUE NOT NULL,
  title VARCHAR(255) UNIQUE NOT NULL,
  TEXT VARCHAR(255) UNIQUE NOT NULL,
  description TEXT,
  type VARCHAR(255),
  version VARCHAR(255),
  is_comment BOOLEAN,
  is_breakpoint BOOLEAN,
  created TEXT,
  category TEXT,
  language TEXT,
  PRIMARY KEY (account_id, xml_url)
);

CREATE INDEX IF NOT EXISTS folders_name_idx ON folders (name);

CREATE INDEX IF NOT EXISTS subscriptions_title_idx ON subscriptions (title);
