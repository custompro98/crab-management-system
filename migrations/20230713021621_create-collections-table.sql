CREATE TABLE collections (
  id SERIAL PRIMARY KEY NOT NULL,
  account_id INT NOT NULL,
  name TEXT NOT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP,

  FOREIGN KEY(account_id)
    REFERENCES accounts(id)
    ON DELETE CASCADE
);

CREATE INDEX collections_account_id ON collections (account_id);

CREATE UNIQUE INDEX collections_account_id_name_unique ON collections (
  account_id,
  lower(name)
) WHERE deleted_at IS NULL;
