CREATE TABLE owner (
  id SERIAL PRIMARY KEY
);

CREATE TABLE tag (
  id SERIAL PRIMARY KEY,
  value VARCHAR(32) NOT NULL,
  public BOOLEAN DEFAULT false,
  FOREIGN KEY owner_id INTEGER REFERENCES owner (id)
);

CREATE TABLE script (
  id SERIAL PRIMARY KEY,
  source VARCHAR NOT NULL,
  FOREIGN KEY owner_id INTEGER REFERENCES owner (id)
);

CREATE TABLE data (
  id SERIAL PRIMARY KEY,
  value VARCHAR NOT NULL,
  FOREIGN KEY input_id INTEGER REFERENCES data (id),
  FOREIGN KEY script_id INTEGER REFERENCES script (id)
);

-- many-to-many junction tables
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
