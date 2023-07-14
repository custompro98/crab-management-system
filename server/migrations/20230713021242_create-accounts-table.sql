CREATE TABLE accounts (
  id SERIAL PRIMARY KEY NOT NULL,
  owner_id INT NOT NULL,
  slug TEXT NOT NULL,
  name TEXT NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ,
  deleted_at TIMESTAMPTZ,

  FOREIGN KEY(owner_id) REFERENCES users(id)
);

CREATE INDEX accounts_owner_id ON accounts (owner_id);
CREATE INDEX accounts_slug ON accounts (lower(slug));

CREATE UNIQUE INDEX accounts_slug_unique ON accounts (
  lower(slug)
) WHERE deleted_at IS NULL;
