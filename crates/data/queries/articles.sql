--! create_article
INSERT INTO article (id, slug, title, description, body, author_id, created_at, updated_at)
VALUES (:id, :slug, :title, :description, :body, :author_id, :created_at, :created_at)
RETURNING *;

--! get_article_by_slug
SELECT * FROM article WHERE slug = :slug;

--! get_article_by_id
SELECT * FROM article WHERE id = :id;

--! update_article
UPDATE article
SET slug = COALESCE(:slug, slug),
    title = COALESCE(:title, title),
    description = COALESCE(:description, description),
    body = COALESCE(:body, body),
    updated_at = :updated_at
WHERE id = :id
RETURNING *;

--! delete_article
DELETE FROM article WHERE id = :id;

--! favorite_article
INSERT INTO article_favorite (appuser_id, article_id)
VALUES (:user_id, :article_id)
ON CONFLICT DO NOTHING;

--! unfavorite_article
DELETE FROM article_favorite
WHERE appuser_id = :user_id AND article_id = :article_id;

--! is_favorited
SELECT EXISTS(
    SELECT 1 FROM article_favorite
    WHERE appuser_id = :user_id AND article_id = :article_id
);

--! get_tags
SELECT name FROM tag;

--! create_tag
INSERT INTO tag (id, name) VALUES (:id, :name) ON CONFLICT (name) DO NOTHING;

--! get_tag_by_name
SELECT * FROM tag WHERE name = :name;

--! add_tag_to_article
INSERT INTO article_tag (article_id, tag_id) VALUES (:article_id, :tag_id) ON CONFLICT DO NOTHING;

--! remove_tags_from_article
DELETE FROM article_tag WHERE article_id = :article_id;

--! get_article_tags
SELECT t.name
FROM tag t
JOIN article_tag at ON t.id = at.tag_id
WHERE at.article_id = :article_id;

--! list_articles
SELECT a.id, a.slug, a.title, a.description, a.body, a.author_id, a.created_at, a.updated_at,
       u.username as author_username, 
       u.bio as author_bio, 
       u.img as author_image,
       EXISTS(SELECT 1 FROM appuser_follows WHERE follower_id = :viewer_id AND followee_id = a.author_id) as following_author,
       EXISTS(SELECT 1 FROM article_favorite WHERE appuser_id = :viewer_id AND article_id = a.id) as favorited,
       (SELECT COUNT(*) FROM article_favorite WHERE article_id = a.id) as favorites_count,
       ARRAY(SELECT t.name FROM tag t JOIN article_tag at ON t.id = at.tag_id WHERE at.article_id = a.id) as tag_list
FROM article a
JOIN appuser u ON a.author_id = u.id
WHERE (:author::text IS NULL OR a.author_id = (SELECT id FROM appuser WHERE username = :author))
  AND (:tag::text IS NULL OR EXISTS(SELECT 1 FROM article_tag at JOIN tag t ON at.tag_id = t.id WHERE at.article_id = a.id AND t.name = :tag))
  AND (:favorited::text IS NULL OR EXISTS(SELECT 1 FROM article_favorite af JOIN appuser u2 ON af.appuser_id = u2.id WHERE af.article_id = a.id AND u2.username = :favorited))
ORDER BY a.created_at DESC
LIMIT :limit OFFSET :offset;

--! count_articles
SELECT COUNT(*)
FROM article a
WHERE (:author::text IS NULL OR a.author_id = (SELECT id FROM appuser WHERE username = :author))
  AND (:tag::text IS NULL OR EXISTS(SELECT 1 FROM article_tag at JOIN tag t ON at.tag_id = t.id WHERE at.article_id = a.id AND t.name = :tag))
  AND (:favorited::text IS NULL OR EXISTS(SELECT 1 FROM article_favorite af JOIN appuser u2 ON af.appuser_id = u2.id WHERE af.article_id = a.id AND u2.username = :favorited));

--! feed_articles
SELECT a.id, a.slug, a.title, a.description, a.body, a.author_id, a.created_at, a.updated_at,
       u.username as author_username, 
       u.bio as author_bio, 
       u.img as author_image,
       true as following_author,
       EXISTS(SELECT 1 FROM article_favorite WHERE appuser_id = :viewer_id AND article_id = a.id) as favorited,
       (SELECT COUNT(*) FROM article_favorite WHERE article_id = a.id) as favorites_count,
       ARRAY(SELECT t.name FROM tag t JOIN article_tag at ON t.id = at.tag_id WHERE at.article_id = a.id) as tag_list
FROM article a
JOIN appuser u ON a.author_id = u.id
JOIN appuser_follows af ON af.followee_id = a.author_id
WHERE af.follower_id = :viewer_id
ORDER BY a.created_at DESC
LIMIT :limit OFFSET :offset;

--! count_feed_articles
SELECT COUNT(*)
FROM article a
JOIN appuser_follows af ON af.followee_id = a.author_id
WHERE af.follower_id = :viewer_id;
