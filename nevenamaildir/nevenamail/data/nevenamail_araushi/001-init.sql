CREATE TABLE IF NOT EXISTS nm_version (
  id_version SERIAL PRIMARY KEY,
  version TEXT NOT NULL,
  ctime TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS nm_item (
  id_item BIGSERIAL PRIMARY KEY,
  value INTEGER NOT NULL,
  ctime TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS nm_user (
  id_user BIGSERIAL PRIMARY KEY,
  email TEXT NOT NULL,
  ctime TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO nm_version (version)
VALUES ('0.1.0-araushi');

INSERT INTO nm_item (value)
VALUES (0);

INSERT INTO nm_user (email)
VALUES ('init@example.com');
