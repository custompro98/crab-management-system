use tonic::Status;

use crate::account::AccountRecord;

use super::super::super::pb::account::Account;

use super::Repository;

impl Repository {
    pub async fn update(&self, account: Account) -> Result<Account, Status> {
        match AccountRecord::from_proto(account) {
            Err(_) => Err(Status::invalid_argument("Account is invalid")),
            Ok(input) => {
                let record = sqlx::query_as!(
                    AccountRecord,
                    r#"
                      UPDATE accounts
                      SET slug = $2, name = $3, updated_at = now()
                      WHERE id = $1
                        AND deleted_at IS NULL
                      RETURNING *
                    "#,
                    input.id, input.slug, input.name
                ).fetch_optional(&self.pool).await;

                match record {
                    Ok(record) => match record {
                        Some(record) => Ok(record.to_proto()),
                        None => Err(Status::not_found("Account not found")),
                    },
                    Err(_) => Err(Status::internal("An internal error occurred")),
                }
            }
        }

    }
}
