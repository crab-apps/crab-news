CREATE VIEW IF NOT EXISTS folder_all_entries AS
SELECT
    feeds.parent_folder AS feed_parent_folder,
    feeds.id AS feed_name,
    e.id AS entry_id,
    e.title AS entry_title,
    e.authors AS entry_author,
    e.published AS entry_published,
    e.summary AS entry_summary,
    e.read_status AS entry_read_status,
    e.starred_status AS entry_starred_status,
    COUNT(*) FILTER (
        WHERE
            e.read_status = 'unread'
    ) OVER () AS unread_count
FROM
    entries AS e
    INNER JOIN feeds ON e.feed_id = feeds.id
    INNER JOIN folders ON feeds.parent_folder = folders.name
    INNER JOIN accounts AS a ON feeds.account_id = a.id
WHERE
    a.name = 'Local'
    AND folders.name = 'Folder 1'
ORDER BY
    e.published desc;

CREATE VIEW IF NOT EXISTS folder_unread_entries AS
SELECT
    feeds.parent_folder AS feed_parent_folder,
    feeds.id AS feed_name,
    e.id AS entry_id,
    e.title AS entry_title,
    e.authors AS entry_author,
    e.published AS entry_published,
    e.summary AS entry_summary,
    e.read_status AS entry_read_status,
    e.starred_status AS entry_starred_status,
    COUNT(*) OVER () AS unread_count
FROM
    entries AS e
    INNER JOIN feeds ON e.feed_id = feeds.id
    INNER JOIN folders ON feeds.parent_folder = folders.name
    INNER JOIN accounts AS a ON feeds.account_id = a.id
WHERE
    a.name = 'Local'
    AND folders.name = 'Folder 1'
    AND e.read_status = 'unread'
ORDER BY
    e.published desc;
