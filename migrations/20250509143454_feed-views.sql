CREATE VIEW IF NOT EXISTS feed_all_entries AS
SELECT
  f.parent_folder AS feed_parent_folder,
  f.id AS feed_name,
  e.id AS entry_name,
  e.title AS entry_title,
  e.authors AS entry_author,
  e.published AS entry_published,
  e.summary AS entry_summary,
  e.read_status AS entry_read_status,
  e.starred_status AS entry_starred_status,
  COUNT(*) FILTER (
    WHERE
      e.read_status='unread'
  ) OVER () AS unread_count
FROM
  entries AS e
  INNER JOIN feeds AS f ON e.feed_id=f.id
  INNER JOIN accounts AS a ON f.account_id=a.id
WHERE
  a.name='Local'
  AND f.id='Feed 1'
ORDER BY
  e.published DESC;

CREATE VIEW IF NOT EXISTS feed_unread_entries AS
SELECT
  f.parent_folder AS feed_parent_folder,
  f.id AS feed_name,
  e.id AS entry_name,
  e.title AS entry_title,
  e.authors AS entry_author,
  e.published AS entry_published,
  e.summary AS entry_summary,
  e.read_status AS entry_read_status,
  e.starred_status AS entry_starred_status,
  COUNT(*) OVER () AS unread_count
FROM
  entries AS e
  INNER JOIN feeds AS f ON e.feed_id=f.id
  INNER JOIN accounts AS a ON f.account_id=a.id
WHERE
  a.name='Local'
  AND f.id='Feed 1'
  AND e.read_status='unread'
ORDER BY
  e.published DESC;
