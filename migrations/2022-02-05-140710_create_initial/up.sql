CREATE TABLE owner (
  id SERIAL PRIMARY KEY,
  name VARCHAR(200) NOT NULL
);

CREATE TABLE tag (
  id SERIAL PRIMARY KEY,
  value VARCHAR(32) NOT NULL,
  public BOOLEAN NOT NULL DEFAULT false,
  owner_id INTEGER NOT NULL,
  FOREIGN KEY (owner_id) REFERENCES owner (id)
);

CREATE TABLE script (
  id SERIAL PRIMARY KEY,
  source VARCHAR NOT NULL,
  owner_id INTEGER NOT NULL,
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
  CONSTRAINT pk_script_tag PRIMARY KEY (script_id, tag_id),
  FOREIGN KEY (script_id) REFERENCES script (id),
  FOREIGN KEY (tag_id) REFERENCES tag (id)
);
