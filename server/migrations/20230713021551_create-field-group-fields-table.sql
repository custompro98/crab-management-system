CREATE TABLE field_group_fields (
  id SERIAL PRIMARY KEY NOT NULL,
  field_group_id INT NOT NULL,
  field_id INT NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ,
  deleted_at TIMESTAMPTZ,

  FOREIGN KEY(field_group_id)
    REFERENCES field_groups(id)
    ON DELETE CASCADE,
  FOREIGN KEY(field_id)
    REFERENCES fields(id)
    ON DELETE CASCADE
);

CREATE INDEX field_group_fields_field_group_id ON field_group_fields (field_group_id);
CREATE INDEX field_group_fields_field_id ON field_group_fields (field_id);

CREATE UNIQUE INDEX field_group_fields_field_group_id_field_id_unique ON field_group_fields (
  field_group_id,
  field_id
) WHERE deleted_at IS NULL;
