use tonic::Status;

use crate::field::FieldRecord;

use super::super::super::pb::field::Field;

use super::Repository;

impl Repository {
    pub async fn batch_get(&self, field_ids: &Vec<i32>) -> Result<Vec<Field>, Status> {
        let records = sqlx::query_as!(
            FieldRecord,
            r#"
              SELECT *
              FROM fields
              WHERE id = ANY($1)
                AND deleted_at IS NULL
            "#,
            &field_ids[..]
        ).fetch_all(&self.pool).await;

        match records {
            Ok(records) => {
                let mut collection: Vec<Field> = vec![];
                for record in records {
                    collection.push(record.to_proto());
                }

                Ok(collection)
            },
            Err(_) => Err(Status::internal("An internal error occurred")),
        }
    }
}
