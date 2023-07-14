CREATE TABLE fields (
  id SERIAL PRIMARY KEY NOT NULL,
  account_id INT NOT NULL,
  name TEXT NOT NULL,
  field_type TEXT NOT NULL,
  value TEXT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ,
  deleted_at TIMESTAMPTZ,

  FOREIGN KEY(account_id)
    REFERENCES accounts(id)
    ON DELETE CASCADE
);

CREATE INDEX fields_account_id ON fields (account_id);

CREATE UNIQUE INDEX fields_account_id_name_unique_partial_unique ON fields (
  account_id,
  lower(name)
) WHERE deleted_at IS NULL;
