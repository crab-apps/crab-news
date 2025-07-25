-- this is a work in progress mockup, needs love
CREATE VIEW IF NOT EXISTS account_all_subs AS
SELECT
  a.alias AS account,
  feeds.parent_folder AS folder_name,
  feeds.id AS feed_name
FROM
  accounts AS a
  LEFT JOIN feeds ON feeds.account_id=a.id
WHERE
  a.name='Local'
ORDER BY
  account,
  folder_name,
  feed_name;
