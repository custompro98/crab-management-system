use tonic::Status;

use crate::field_group::FieldGroupRecord;

use super::super::super::pb::field_group::FieldGroup;

use super::Repository;

impl Repository {
    pub async fn update(&self, field_group: FieldGroup) -> Result<FieldGroup, Status> {
        match FieldGroupRecord::from_proto(field_group) {
            Err(_) => Err(Status::invalid_argument("FieldGroup is invalid")),
            Ok(input) => {
                let record = sqlx::query_as!(
                    FieldGroupRecord,
                    r#"
                      UPDATE field_groups
                      SET name = $2, updated_at = now()
                      WHERE id = $1
                        AND deleted_at IS NULL
                      RETURNING *
                    "#,
                    input.id, input.name
                ).fetch_optional(&self.pool).await;

                match record {
                    Ok(record) => match record {
                        Some(record) => Ok(record.to_proto()),
                        None => Err(Status::not_found("FieldGroup not found")),
                    },
                    Err(_) => Err(Status::internal("An internal error occurred")),
                }
            }
        }

    }
}
