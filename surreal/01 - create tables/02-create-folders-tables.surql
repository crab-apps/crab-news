-- name is used in Outline both title and text types. <outline text="Group Name" title="Group Name">
-- name can be NULL to match Rust's None == feed placed in Account's root
DEFINE TABLE IF NOT EXISTS folders TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD IF NOT EXISTS name ON folders TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD account_name ON folders TYPE record<accounts> REFERENCE ON DELETE CASCADE PERMISSIONS FULL;
DEFINE FIELD subscriptions ON folders TYPE references PERMISSIONS FULL;
DEFINE FIELD feeds ON folders TYPE references PERMISSIONS FULL;
DEFINE INDEX IF NOT EXISTS folders_name_idx ON folders FIELDS name UNIQUE;
