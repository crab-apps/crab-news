-- this is a work in progress mockup, needs love
CREATE VIEW IF NOT EXISTS progressbar AS
SELECT
  COUNT(f.id) AS total_feeds_number
FROM
  feeds AS f;
