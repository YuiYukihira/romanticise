CREATE TABLE ImageTag
(
  image_id INTEGER NOT NULL REFERENCES Image(id),
  tag_id INTEGER NOT NULL REFERENCES Tag(id)
);