CREATE TABLE field_groups (
  id SERIAL PRIMARY KEY NOT NULL,
  account_id INT NOT NULL,
  name TEXT NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ,
  deleted_at TIMESTAMPTZ,

  FOREIGN KEY(account_id)
    REFERENCES accounts(id)
    ON DELETE CASCADE
);

CREATE INDEX field_groups_account_id ON field_groups (account_id);

CREATE UNIQUE INDEX field_groups_account_id_name_unique ON field_groups (
  account_id,
  lower(name)
) WHERE deleted_at IS NULL;
