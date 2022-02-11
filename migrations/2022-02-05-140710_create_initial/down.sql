-- drop indexing
DROP INDEX idx_owner_external_id;

-- drop junction tables
DROP TABLE owner_tag;
DROP TABLE script_tag;
DROP TABLE data_tag;

-- drop 'regular' tables
DROP TABLE data;
DROP TABLE script;
DROP TABLE tag;
DROP TABLE owner;
