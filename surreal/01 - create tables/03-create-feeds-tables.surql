-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Feed.html
DEFINE TABLE IF NOT EXISTS feeds TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD IF NOT EXISTS feed_type ON feeds TYPE string ASSERT type INSIDE [ 'Atom', 'JSON', 'RSS0', 'RSS1', 'RSS2' ] PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS feed_id ON feeds TYPE string PERMISSIONS FULL ;
DEFINE FIELD IF NOT EXISTS title ON feeds TYPE option<string> PERMISSIONS FULL ;
DEFINE FIELD IF NOT EXISTS updated ON feeds TYPE option<datetime> PERMISSIONS FULL ;
DEFINE FIELD IF NOT EXISTS authors ON feeds TYPE array<persons> PERMISSIONS FULL ;
DEFINE FIELD IF NOT EXISTS description ON feeds TYPE option<string> PERMISSIONS FULL ;
DEFINE FIELD IF NOT EXISTS links ON feeds TYPE array<links> PERMISSIONS FULL ;
DEFINE FIELD IF NOT EXISTS categories ON feeds TYPE array<categories> PERMISSIONS FULL ;
DEFINE FIELD IF NOT EXISTS contributors ON feeds TYPE array<persons> PERMISSIONS FULL ;
DEFINE FIELD IF NOT EXISTS logo ON feeds TYPE option<string> PERMISSIONS FULL ;
DEFINE FIELD IF NOT EXISTS icon ON feeds TYPE option<string> PERMISSIONS FULL ;
DEFINE FIELD IF NOT EXISTS published ON feeds TYPE option<datetime> PERMISSIONS FULL ;
DEFINE FIELD IF NOT EXISTS language ON feeds TYPE option<string> PERMISSIONS FULL ;
DEFINE FIELD IF NOT EXISTS rights ON feeds TYPE option<string> PERMISSIONS FULL ;
DEFINE FIELD IF NOT EXISTS ttl ON feeds TYPE option<int> PERMISSIONS FULL ;
DEFINE FIELD account_name ON feeds TYPE record<accounts> REFERENCE ON DELETE CASCADE PERMISSIONS FULL ;
DEFINE FIELD parent_folder ON feeds TYPE option<record<folders>> REFERENCE ON DELETE CASCADE PERMISSIONS FULL ;
DEFINE FIELD entries ON feeds TYPE references PERMISSIONS FULL;

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Entry.html
DEFINE TABLE IF NOT EXISTS entries TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD IF NOT EXISTS entry_id ON entries TYPE string PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS title ON entries TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS updated ON entries TYPE option<datetime> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS authors ON entries TYPE array<persons> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS content ON entries TYPE option<contents> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS links ON entries TYPE array<links> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS summary ON entries TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS categories ON entries TYPE array<categories> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS contributors ON entries TYPE array<persons> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS published ON entries TYPE option<datetime> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS source ON entries TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS rights ON entries TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS media ON entries TYPE array<media_object> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS language ON entries TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS base ON entries TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS read_status ON entries TYPE string ASSERT read_status INSIDE ['unread', 'read'] DEFAULT 'unread';
DEFINE FIELD IF NOT EXISTS starred_status ON entries TYPE string ASSERT starred_status INSIDE ['unstarred', 'starred'] DEFAULT 'unstarred';
DEFINE FIELD feed_id ON entries TYPE record<feeds> REFERENCE ON DELETE CASCADE PERMISSIONS FULL ;
DEFINE FIELD content ON entries TYPE references PERMISSIONS FULL;

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Content.html
DEFINE TABLE IF NOT EXISTS contents TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD IF NOT EXISTS body ON contents TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS content_type ON contents TYPE string PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS length ON contents TYPE option<int> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS src ON contents TYPE option<links> PERMISSIONS FULL;
-- DEFINE FIELD entry_id ON contents TYPE record<feeds> REFERENCE ON DELETE CASCADE PERMISSIONS FULL;

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Person.html
DEFINE TABLE IF NOT EXISTS persons TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD IF NOT EXISTS name ON persons TYPE string PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS uri ON persons TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS email ON persons TYPE option<string> PERMISSIONS FULL;
-- DEFINE FIELD feed_id ON persons TYPE record<feeds> REFERENCE ON DELETE CASCADE PERMISSIONS FULL;

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Link.html
DEFINE TABLE IF NOT EXISTS links TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD IF NOT EXISTS href ON links TYPE string PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS rel ON links TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS media_type ON links TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS href_lang ON links TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS title ON links TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS length ON links TYPE option<int> PERMISSIONS FULL;

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Generator.html
DEFINE TABLE IF NOT EXISTS generators TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD IF NOT EXISTS content ON generators TYPE string PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS uri ON generators TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS version ON generators TYPE option<string> PERMISSIONS FULL;

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.Category.html
DEFINE TABLE IF NOT EXISTS categories TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD IF NOT EXISTS term ON categories TYPE string PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS scheme ON categories TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS label ON categories TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS subcategories ON categories TYPE array<record<categories>> PERMISSIONS FULL;

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.MediaObject.html
DEFINE TABLE IF NOT EXISTS media_objects TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD IF NOT EXISTS title ON MediaObject TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS content ON MediaObject TYPE array<media_content> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS duration ON MediaObject TYPE option<Duration> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS thumbnails ON MediaObject TYPE array<MediaThumbnail> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS texts ON MediaObject TYPE array<MediaText> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS description ON MediaObject TYPE option<Text> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS community ON MediaObject TYPE option<MediaCommunity> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS credits ON MediaObject TYPE array<MediaCredit> PERMISSIONS FULL;

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.MediaRating.html
DEFINE TABLE IF NOT EXISTS media_ratings TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD IF NOT EXISTS urn ON media_ratings TYPE string PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS value ON media_ratings TYPE string PERMISSIONS FULL;

-- https://docs.rs/feed-rs/2.3.1/feed_rs/model/struct.MediaContent.html
DEFINE TABLE IF NOT EXISTS media_contents TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD IF NOT EXISTS url ON media_contents TYPE string PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS content_type ON media_contents TYPE string PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS height ON media_contents TYPE option<int> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS width ON media_contents TYPE option<int> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS duration ON media_contents TYPE option<Duration> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS size ON media_contents TYPE option<int> PERMISSIONS FULL;
DEFINE FIELD IF NOT EXISTS rating ON media_contents TYPE option<media_rating> PERMISSIONS FULL;
