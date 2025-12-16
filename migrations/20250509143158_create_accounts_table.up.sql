CREATE TABLE IF NOT EXISTS accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    -- name: to use with Rust related actions and safe db actions
    name VARCHAR(50) UNIQUE NOT NULL CHECK (
        name IN (
            'Local',
            'iCloud',
            'Live 365',
            'Google Sync',
            'Ubuntu One'
        )
    ),
    -- alias: for display (preserving account_name functionality)
    alias VARCHAR(50) DEFAULT name UNIQUE
);

CREATE INDEX IF NOT EXISTS idx_account_name ON accounts (name);
