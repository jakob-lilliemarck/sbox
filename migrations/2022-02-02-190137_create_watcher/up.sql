CREATE TABLE source_tags (
  source_id INTEGER NOT NULL,
  tag_id VARCHAR NOT NULL,
  CONSTRAINT pk_source_tags PRIMARY KEY (source_id, tag_id),
  FOREIGN KEY (source_id) REFERENCES source (id),
  FOREIGN KEY (tag_id) REFERENCES tags (id)
);

CREATE TABLE outputs (
  id SERIAL PRIMARY KEY,
  data VARCHAR NOT NULL
);

ALTER TABLE source
ADD output_id INTEGER REFERENCES outputs (id);

ALTER TABLE inputs
ADD output_id INTEGER REFERENCES outputs (id);

DROP TABLE test;
