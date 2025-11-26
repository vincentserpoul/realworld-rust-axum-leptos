--! create_comment
INSERT INTO comment (body, article_id, author_id, created_at, updated_at)
VALUES (:body, :article_id, :author_id, :created_at, :created_at)
RETURNING *;

--! get_comments_by_article
SELECT * FROM comment WHERE article_id = :article_id ORDER BY created_at DESC;

--! delete_comment
DELETE FROM comment WHERE id = :id;

--! get_comment_by_id
SELECT * FROM comment WHERE id = :id;
