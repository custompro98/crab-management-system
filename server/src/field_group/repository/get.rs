use tonic::Status;

use crate::field_group::FieldGroupRecord;

use super::super::super::pb::field_group::FieldGroup;

use super::Repository;

impl Repository {
    pub async fn get(&self, id: i32) -> Result<FieldGroup, Status> {
        let record = sqlx::query_as!(
            FieldGroupRecord,
            r#"
              SELECT *
              FROM field_groups
              WHERE id = $1
                AND deleted_at IS NULL
              LIMIT 1
            "#,
            id
        ).fetch_optional(&self.pool).await;

        match record {
            Ok(record) => match record {
                Some(record) => Ok(record.to_proto()),
                None => Err(Status::not_found("FieldGroup not found").into()),
            },
            Err(_) => Err(Status::internal("An internal error occurred")),
        }
    }
}
