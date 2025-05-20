-- <outline text="Feed Name" type="rss" xmlUrl="https://example.com/rss.xml"
-- description="" htmlUrl="https://example.com/" title="Feed Name" version="RSS"/>
DEFINE TABLE IF NOT EXISTS subscriptions TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD IF NOT EXISTS text ON subscriptions TYPE string PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS type ON subscriptions TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS is_comment ON subscriptions TYPE option<bool> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS is_breakpoint ON subscriptions TYPE option<bool> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS created ON subscriptions TYPE option<datetime> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS category ON subscriptions TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS xml_url ON subscriptions TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS description ON subscriptions TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS html_url ON subscriptions TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS language ON subscriptions TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS title ON subscriptions TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS version ON subscriptions TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS url ON subscriptions TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD account_name ON subscriptions TYPE record<accounts> REFERENCE ON DELETE CASCADE PERMISSIONS FULL;
DEFINE FIELD parent_folder ON subscriptions TYPE option<record<folders>> REFERENCE ON DELETE CASCADE PERMISSIONS FULL;
DEFINE INDEX IF NOT EXISTS subscriptions_title_idx ON subscriptions FIELDS title UNIQUE;
