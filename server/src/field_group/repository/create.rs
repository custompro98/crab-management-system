use tonic::Status;

use crate::field_group::FieldGroupRecord;

use super::super::super::pb::field_group::FieldGroup;

use super::Repository;

impl Repository {
    pub async fn create(&self, field_group: FieldGroup) -> Result<FieldGroup, Status> {
        match FieldGroupRecord::from_proto(field_group) {
            Err(e) => Err(Status::invalid_argument(format!("{}", e))),
            Ok(input) => {
                let record = sqlx::query_as!(
                    FieldGroupRecord,
                    r#"
                      INSERT INTO field_groups (account_id, name)
                      VALUES ($1, $2)
                      RETURNING *
                    "#,
                    input.account_id, input.name
                ).fetch_optional(&self.pool).await;

                match record {
                    Ok(record) => match record {
                        Some(record) => Ok(record.to_proto()),
                        None => Err(Status::failed_precondition("FieldGroup not created")),
                    },
                    Err(_) => Err(Status::internal("An internal error occurred")),

                }
            }
        }
    }
}
