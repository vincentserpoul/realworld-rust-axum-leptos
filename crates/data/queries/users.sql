--! create_user
INSERT INTO appuser (id, email, username, pwd, created_at, updated_at)
VALUES (:id, :email, :username, :pwd, :created_at, :created_at)
RETURNING *;

--! get_user_by_email
SELECT * FROM appuser WHERE email = :email;

--! get_user_by_username
SELECT * FROM appuser WHERE username = :username;

--! get_user_by_id
SELECT * FROM appuser WHERE id = :id;

--! update_user
UPDATE appuser
SET email = COALESCE(:email, email),
    username = COALESCE(:username, username),
    pwd = COALESCE(:pwd, pwd),
    img = COALESCE(:img, img),
    bio = COALESCE(:bio, bio),
    updated_at = :updated_at
WHERE id = :id
RETURNING *;

--! follow_user
INSERT INTO appuser_follows (follower_id, followee_id)
VALUES (:follower_id, :followee_id)
ON CONFLICT DO NOTHING;

--! unfollow_user
DELETE FROM appuser_follows
WHERE follower_id = :follower_id AND followee_id = :followee_id;

--! is_following
SELECT EXISTS(
    SELECT 1 FROM appuser_follows
    WHERE follower_id = :follower_id AND followee_id = :followee_id
);
