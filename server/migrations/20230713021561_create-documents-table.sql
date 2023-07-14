CREATE TABLE documents (
  id SERIAL PRIMARY KEY NOT NULL,
  account_id INT NOT NULL,
  field_group_id INT NOT NULL,
  name TEXT NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ,
  deleted_at TIMESTAMPTZ,

  FOREIGN KEY(account_id)
    REFERENCES accounts(id)
    ON DELETE CASCADE,

  FOREIGN KEY(field_group_id)
    REFERENCES field_groups(id)
    ON DELETE CASCADE
);

CREATE INDEX documents_account_id ON documents (account_id);
CREATE INDEX documents_field_group_id ON documents (field_group_id);

CREATE UNIQUE INDEX documents_account_id_name_unique_partial_unique ON documents (
  account_id,
  lower(name)
) WHERE deleted_at IS NULL;
