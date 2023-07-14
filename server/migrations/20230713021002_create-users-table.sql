CREATE TABLE users (
  id SERIAL PRIMARY KEY NOT NULL,
  email TEXT NOT NULL,
  username TEXT NOT NULL,
  name TEXT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ,
  deleted_at TIMESTAMPTZ
);

CREATE INDEX users_email ON users (email);
CREATE INDEX users_username ON users (username);

CREATE UNIQUE INDEX users_email_unique ON users (
  lower(email)
) WHERE deleted_at IS NULL;

CREATE UNIQUE INDEX users_username_unique ON users (
  lower(username)
) WHERE deleted_at IS NULL;
