use tonic::Status;

use crate::field::FieldRecord;

use super::super::super::pb::field::Field;

use super::Repository;

impl Repository {
    pub async fn on_create_field(&self, field: Field) -> Result<Field, Status> {
        match FieldRecord::from_proto(field) {
            Err(e) => Err(Status::invalid_argument(format!("{}", e))),
            Ok(input) => {
                let record = sqlx::query_as!(
                    FieldRecord,
                    r#"
                      INSERT INTO fields (account_id, name, field_type, value)
                      VALUES ($1, $2, $3, $4)
                      RETURNING *
                    "#,
                    input.account_id, input.name, input.field_type.to_string(), input.value
                ).fetch_optional(&self.pool).await;

                match record {
                    Ok(record) => match record {
                        Some(record) => Ok(record.to_proto()),
                        None => Err(Status::failed_precondition("Field not created")),
                    },
                    Err(_) => Err(Status::internal("An internal error occurred")),

                }
            }
        }
    }
}
