-- account_name is used to permit allowed names only and to use with Rust related actions and safe db actions
-- account_alias is used to allow renaming an account and for display (preserving account_name functionality)
CREATE TABLE IF NOT EXISTS accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) CHECK (
        name IN (
            'Local',
            'iCloud',
            'Live 365',
            'Google Sync',
            'Ubuntu One'
        )
    ) NOT NULL DEFAULT 'Local',
    alias VARCHAR(50) UNIQUE
);
