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

CREATE TABLE IF NOT EXISTS nm_message (
  id_message BIGSERIAL PRIMARY KEY,
  sender_user_id BIGINT NOT NULL REFERENCES nm_user(id_user),
  subject TEXT NOT NULL,
  body TEXT NOT NULL,
  ctime TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS nm_recipient_kind (
  id_recipient_kind INT PRIMARY KEY,
  kind TEXT NOT NULL,
  ctime TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS nm_message_recipient (
  id_message_recipient BIGSERIAL PRIMARY KEY,
  message_id BIGINT NOT NULL REFERENCES nm_message(id_message),
  recipient_user_id BIGINT NOT NULL REFERENCES nm_user(id_user),
  kind_id INT NOT NULL REFERENCES nm_recipient_kind(id_recipient_kind),
  ctime TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO nm_version (version)
VALUES ('0.1.0-araushi');

INSERT INTO nm_item (value)
VALUES (0);

INSERT INTO nm_user (email)
VALUES ('a100@example.com');
INSERT INTO nm_user (email)
VALUES ('b200@example.com');
INSERT INTO nm_user (email)
VALUES ('c300@example.com');
INSERT INTO nm_user (email)
VALUES ('d400@example.com');

INSERT INTO nm_message (sender_user_id, subject, body)
VALUES (1, 'Init Subject', 'Init Body');

INSERT INTO nm_recipient_kind (id_recipient_kind, kind)
VALUES (1, 'to');
INSERT INTO nm_recipient_kind (id_recipient_kind, kind)
VALUES (2, 'cc');
INSERT INTO nm_recipient_kind (id_recipient_kind, kind)
VALUES (3, 'bcc');

INSERT INTO nm_message_recipient(message_id, recipient_user_id, kind_id)
VALUES (1, 2, 1);