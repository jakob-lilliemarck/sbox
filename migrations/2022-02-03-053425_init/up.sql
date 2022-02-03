CREATE TABLE script (
  id SERIAL PRIMARY KEY,
  lang VARCHAR NOT NULL,
  source VARCHAR NOT NULL
);

CREATE TABLE input (
  id SERIAL PRIMARY KEY,
  data VARCHAR NOT NULL
);

CREATE TABLE tag (
  id VARCHAR(32) PRIMARY KEY
);

-- Junction tables. Output is a junction that holds own data.
CREATE TABLE output (
  data VARCHAR NOT NULL,
  script_id INTEGER NOT NULL,
  input_id INTEGER NOT NULL,
  CONSTRAINT pk_output PRIMARY KEY (script_id, input_id),
  FOREIGN KEY (script_id) REFERENCES script (id),
  FOREIGN KEY (input_id) REFERENCES input (id)
);

CREATE TABLE input_tag (
  input_id INTEGER NOT NULL,
  tag_id VARCHAR NOT NULL,
  CONSTRAINT pk_input_tag PRIMARY KEY (input_id, tag_id),
  FOREIGN KEY (input_id) REFERENCES input (id),
  FOREIGN KEY (tag_id) REFERENCES tag (id)
);

CREATE TABLE script_tag (
  script_id INTEGER NOT NULL,
  tag_id VARCHAR NOT NULL,
  CONSTRAINT pk_script_tag PRIMARY KEY (script_id, tag_id),
  FOREIGN KEY (script_id) REFERENCES script (id),
  FOREIGN KEY (tag_id) REFERENCES tag (id)
);
