-- name is used to permit allowed names only and to use with Rust related actions and safe db actions
-- alias is used to allow renaming an account and for display (preserving account_name functionality)
DEFINE TABLE IF NOT EXISTS accounts TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD IF NOT EXISTS name ON accounts TYPE string DEFAULT 'Local' ASSERT name INSIDE [
	'Local',
	'iCloud',
	'Live 365',
	'Google Sync',
	'Ubuntu One'
]
	PERMISSIONS FULL
;
DEFINE FIELD IF NOT EXISTS alias ON accounts TYPE string DEFAULT name PERMISSIONS FULL;
DEFINE FIELD feeds ON accounts TYPE references PERMISSIONS FULL;
DEFINE FIELD folders ON accounts TYPE references PERMISSIONS FULL;
DEFINE FIELD subscriptions ON accounts TYPE references PERMISSIONS FULL;
DEFINE INDEX IF NOT EXISTS account_name_idx ON accounts FIELDS name UNIQUE;
