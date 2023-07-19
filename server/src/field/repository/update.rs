use tonic::Status;

use crate::field::FieldRecord;

use super::super::super::pb::field::Field;

use super::Repository;

impl Repository {
    pub async fn update(&self, field: Field) -> Result<Field, Status> {
        match FieldRecord::from_proto(field) {
            Err(_) => Err(Status::invalid_argument("Field is invalid")),
            Ok(input) => {
                let record = sqlx::query_as!(
                    FieldRecord,
                    r#"
                      UPDATE fields
                      SET name = $2, field_type = $3, value = $4, updated_at = now()
                      WHERE id = $1
                        AND deleted_at IS NULL
                      RETURNING *
                    "#,
                    input.id, input.name, input.field_type.to_string(), input.value
                ).fetch_optional(&self.pool).await;

                match record {
                    Ok(record) => match record {
                        Some(record) => Ok(record.to_proto()),
                        None => Err(Status::not_found("Field not found")),
                    },
                    Err(_) => Err(Status::internal("An internal error occurred")),
                }
            }
        }

    }
}
