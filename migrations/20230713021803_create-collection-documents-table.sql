CREATE TABLE collection_documents (
  id SERIAL PRIMARY KEY NOT NULL,
  collection_id INT NOT NULL,
  document_id INT NOT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP,

  FOREIGN KEY(collection_id)
    REFERENCES collections(id)
    ON DELETE CASCADE,
  FOREIGN KEY(document_id)
    REFERENCES documents(id)
    ON DELETE CASCADE
);

CREATE INDEX collection_documents_collection_id ON collection_documents (collection_id);
CREATE INDEX collection_documents_document_id ON collection_documents (document_id);


CREATE UNIQUE INDEX collection_documents_collection_id_document_id_unique ON collection_documents (
  collection_id,
  document_id
) WHERE deleted_at IS NULL;
