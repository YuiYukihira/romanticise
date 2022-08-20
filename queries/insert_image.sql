INSERT INTO image (id, hash, path)
VALUES ($1, $2, $3)
RETURNING id;
