CREATE TABLE document_fields (
  id SERIAL PRIMARY KEY NOT NULL,
  document_id INT NOT NULL,
  field_id INT NOT NULL,
  value TEXT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ,
  deleted_at TIMESTAMPTZ,

  FOREIGN KEY(document_id)
    REFERENCES documents(id)
    ON DELETE CASCADE,
  FOREIGN KEY(field_id)
    REFERENCES fields(id)
    ON DELETE CASCADE
);

CREATE INDEX document_fields_document_id ON document_fields (document_id);
CREATE INDEX document_fields_field_id ON field_group_fields (field_id);

CREATE UNIQUE INDEX document_fields_document_id_field_id_unique ON document_fields (
  document_id,
  field_id
) WHERE deleted_at IS NULL;
