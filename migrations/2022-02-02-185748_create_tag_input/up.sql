CREATE TABLE input_tags (
  input_id INTEGER NOT NULL,
  tag_id VARCHAR NOT NULL,
  CONSTRAINT pk_input_tags PRIMARY KEY (input_id, tag_id),
  FOREIGN KEY (input_id) REFERENCES inputs (id),
  FOREIGN KEY (tag_id) REFERENCES tags (id)
)
