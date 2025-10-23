BEGIN;
CREATE TABLE IF NOT EXISTS ctid(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    value TEXT NOT NULL UNIQUE,
    added INTEGER NOT NULL DEFAULT (unixepoch('now')),
    expired INTEGER NOT NULL DEFAULT 0 CHECK (expired IN (0, 1))
) STRICT;

CREATE INDEX IF NOT EXISTS ctid_name ON ctid(name);
CREATE INDEX IF NOT EXISTS ctid_name_version ON ctid(name, version);
CREATE INDEX IF NOT EXISTS ctid_value ON ctid(value);
END;