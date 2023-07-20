use sqlx::{QueryBuilder, Postgres};
use tonic::Status;

use crate::field_group::FieldGroupRecord;

use super::super::super::pb::field_group::FieldGroup;

use super::Repository;

impl Repository {
    pub async fn update(&self, field_group: FieldGroup, field_ids: &Vec<i32>) -> Result<FieldGroup, Status> {
        match FieldGroupRecord::from_proto(field_group) {
            Err(_) => Err(Status::invalid_argument("FieldGroup is invalid")),
            Ok(input) => {
                let trx = self.pool.begin().await;

                if let Err(e) = trx {
                    return Err(Status::internal(e.to_string()));
                }

                let mut trx = trx.unwrap();

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
                ).fetch_optional(&mut *trx).await;

                match &record {
                    Err(e) => {
                        match trx.rollback().await {
                            Ok(_) => return Err(Status::internal(e.to_string())),
                            Err(e) => return Err(Status::internal(e.to_string())),
                        };
                    }
                    Ok(_) => {
                        match record.as_ref().unwrap() {
                            None => {
                                match trx.rollback().await {
                                    Ok(_) => return Err(Status::internal("An internal error occurred")),
                                    Err(e) => return Err(Status::internal(e.to_string())),
                                };
                            },
                            _ => {}
                        }
                    },
                }

                let record = record.unwrap().unwrap();

                let removed_field_group_fields = sqlx::query!(
                    r#"
                      UPDATE field_group_fields
                      SET deleted_at = now()
                      WHERE field_group_id = $1
                        AND deleted_at IS NULL
                    "#,
                    input.id
                ).execute(&mut *trx).await;

                if let Err(e) = removed_field_group_fields {
                    match trx.rollback().await {
                        Ok(_) => return Err(Status::internal(e.to_string())),
                        Err(e) => return Err(Status::internal(e.to_string())),
                    };
                }

                if field_ids.len() > 0 {
                    // Note: using QueryBuilder because sqlx::query! does not support dynamic binds
                    // https://github.com/launchbadge/sqlx/issues/1560
                    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
                        "INSERT INTO field_group_fields (field_group_id, field_id) "
                    );

                    query_builder.push_values(field_ids, |mut bindings, field_id| {
                        bindings
                            .push_bind(record.id)
                            .push_bind(field_id);
                    });

                    let field_group_fields = query_builder.build().execute(&mut *trx).await;

                    if let Err(e) = field_group_fields {
                        match trx.rollback().await {
                            Ok(_) => return Err(Status::internal(e.to_string())),
                            Err(e) => return Err(Status::internal(e.to_string())),
                        };
                    }
                }

                if let Err(e) = trx.commit().await {
                    return Err(Status::internal(e.to_string()));
                }

                Ok(record.to_proto())
            }
        }

    }
}
