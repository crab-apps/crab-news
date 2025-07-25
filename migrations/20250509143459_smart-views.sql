CREATE VIEW IF NOT EXISTS today_view AS
SELECT
  e.id AS entry_id,
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
WHERE
  DATE(e.published)=CURRENT_DATE
ORDER BY
  e.published DESC;

CREATE VIEW IF NOT EXISTS unread_view AS
SELECT
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
WHERE
  e.read_status='unread'
ORDER BY
  e.published DESC;

CREATE VIEW IF NOT EXISTS starred_view AS
SELECT
  e.id AS entry_id,
  e.title AS entry_title,
  e.authors AS entry_author,
  e.published AS entry_published,
  e.summary AS entry_summary,
  e.read_status AS entry_read_status,
  e.starred_status AS entry_starred_status,
  COUNT(*) OVER () AS starred_count
FROM
  entries AS e
WHERE
  e.starred_status='starred'
ORDER BY
  e.published DESC;
