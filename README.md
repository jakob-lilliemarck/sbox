# Sbox - Sandbox untrusted code anywhere

## Todo

**DONE Celery app-instance request-guard and fairing:**
Create a request guard to make it simple to send celery tasks. Any route should
be able to create tasks on the queue.

**Extend database schema and apis:**
Existing:
- `source` table:
  1. `lang` - language used, (consider remove in favour of !#shebang).
  2. `src` - the script source as a string.

Extend with:
- `inputs` table:
  1. `data` - json data string.
  2. `tags` - many-to-many relationship. One `input` may be tagged with multiple `tags`, and a `tag` may be used for multiple `inputs`.
- `tags` table:
  1. `id` - user defined string, unique.
  2. `inputs` - many-to-many relationship, (as described above).
  3. `outputs` - many-to-many relationship, (as described above).
- `outputs` table:
  1. `data` - json data string.
  2. `input` - many-to-one relationship, one `input` may relate to many `outputs`
  3. `source` - the source that resulted in this `output`, many-to-one (as described above)
- `watcher` table:
  1. `tags` - the `tags` to watch new inputs for
  2. `source` - the `source` this `watcher` runs on new input, one-to-many relationship.
  A `watcher` relates to one `source`, but a `source` may be used by multiple watchers.

IDEA
  Should "tags" be used over "watcher"? If an `input` and `source` shares a `tag`, and the `tag` table is a junction table between the two - then all script relating to a particular input-update can be readily retrieved!

  Watchers then have no meaning.

  `Outputs` could be another junction? Outputs are the juction between a `Source` and an `Input`.

  DRAW THIS!


**Duplex communication:**
Search options and implement a protocol and frameowkr that supports full-duplex
communication, with the goal of beeing able to push data to a client on updates
to any database entity.

More specifically, this app should push data on changes to a specific column of
a set of entries in a "watcher" table. i.e:
"When the column 'a' of watcher with id "1" updates with a value *not equal* to
the current value, push the updates watcher to the client"

**GitHub actions build & deploy:**
Set up a VM and a GitHub actions build & deploy workflow - The app must go online!
