# sbox api

## Endpoints
### /owner
 - `POST` - Create a new owner-entity.

### /scripts
 - `POST` - Create a new script
 - `GET` - A list of own scripts

### /script/{id}
 - `GET` - Get a script record
 - `PUT` - Update a script record
 - `DELETE` - Delete a script record

### /data
 - `POST` - Create a data record

### /data/{id}
 - `GET` - Get one data record

### /tags
 - `GET` - Get list of public & own tags

## Resources
Resources are an abstraction on top of the db, it's not a 1 to 1 relationship!

### Script
```
{
  id: i32,
  source: String,
  tags: [String],
  output_tags: [String],
  outputs: [i32],
  owner: i32,
}
```
Where:
 - `id` - Unique identifier of this script
 - `source` - the source to execute.
 - `tags` - Tags this script "watches" new data for.
 - `output_tags` - Tags to tag the output `data` with (must be own tags).
 - `outputs` - Ids of all the `data` records the script has produced.
 - `owner` - Unique identifier of the owner.

### Data
```
{
  id: i32,
  input_id: i32,
  tags: [String],
  value: String
}
```
Where:
 - `id` - Unique i### Endpoints
dentifier of this data.
 - `input_id` - Id of `data` that produced this data (nullable).
 - `script_id` - Id of script that produced this data (nullable).
 - `tags` - Tags used to trigger associated functionality. (must be own tags for user created data).
 - `value` - json-serialized value of this record.

### Tag
```
{
  id: i32,
  value: String,
  public: Bool,
  owner: i32
}
```
Where
 - `id` - Unique identifier
 - `value` - A non-whitespaced string to remember or describe this tag.
 - `public` - True if this tag may be associated with non-owner functions and webhooks.
 - `owner` - Unique identifier of the owner.


### Owner
```
{
  id: i32
}
```
Where:
 - `id` - Unique identifier.
