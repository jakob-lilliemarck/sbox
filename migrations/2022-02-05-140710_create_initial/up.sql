CREATE TABLE owner (
  id SERIAL PRIMARY KEY,
  name VARCHAR(200) NOT NULL
);

CREATE TABLE tag (
  id SERIAL PRIMARY KEY,
  value VARCHAR(32) NOT NULL,
  is_public BOOLEAN NOT NULL DEFAULT false,
  owner_id INTEGER, -- should be nullable to allow orphan tags
  FOREIGN KEY (owner_id) REFERENCES owner (id)
);

CREATE TABLE script (
  id SERIAL PRIMARY KEY,
  source VARCHAR NOT NULL,
  owner_id INTEGER, -- should be nullable to allow orphan scripts
  FOREIGN KEY (owner_id) REFERENCES owner (id)
);

CREATE TABLE data (
  id SERIAL PRIMARY KEY,
  value VARCHAR NOT NULL,
  input_id INTEGER,
  script_id INTEGER,
  FOREIGN KEY (input_id) REFERENCES data (id),
  FOREIGN KEY (script_id) REFERENCES script (id)
);

-- junction tables
CREATE TABLE data_tag (
  data_id INTEGER NOT NULL,
  tag_id INTEGER NOT NULL,
  CONSTRAINT pk_data_tag PRIMARY KEY (data_id, tag_id),
  FOREIGN KEY (data_id) REFERENCES data (id),
  FOREIGN KEY (tag_id) REFERENCES tag (id)
);

CREATE TABLE script_tag (
  script_id INTEGER NOT NULL,
  tag_id INTEGER NOT NULL,
  -- is_output describes if this tag is for tagging script results or tagging the script itself.
  is_output BOOLEAN NOT NULL,
  CONSTRAINT pk_script_tag PRIMARY KEY (script_id, tag_id),
  FOREIGN KEY (script_id) REFERENCES script (id),
  FOREIGN KEY (tag_id) REFERENCES tag (id)
);

CREATE TABLE owner_tag (
  owner_id INTEGER NOT NULL,
  tag_id INTEGER NOT NULL,
  CONSTRAINT pk_owner_tag PRIMARY KEY (owner_id, tag_id),
  FOREIGN KEY (owner_id) REFERENCES owner (id),
  FOREIGN KEY (tag_id) REFERENCES tag (id)
)
