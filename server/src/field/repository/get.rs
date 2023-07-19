use tonic::Status;

use crate::field::FieldRecord;

use super::super::super::pb::field::Field;

use super::Repository;

impl Repository {
    pub async fn get(&self, id: i32) -> Result<Field, Status> {
        let record = sqlx::query_as!(
            FieldRecord,
            r#"
              SELECT *
              FROM fields
              WHERE id = $1
                AND deleted_at IS NULL
              LIMIT 1
            "#,
            id
        ).fetch_optional(&self.pool).await;

        match record {
            Ok(record) => match record {
                Some(record) => Ok(record.to_proto()),
                None => Err(Status::not_found("Field not found").into()),
            },
            Err(_) => Err(Status::internal("An internal error occurred")),
        }
    }
}
