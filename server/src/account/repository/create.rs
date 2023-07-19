use tonic::Status;

use crate::account::AccountRecord;

use super::super::super::pb::account::Account;

use super::Repository;

impl Repository {
    pub async fn create(&self, account: Account) -> Result<Account, Status> {
        match AccountRecord::from_proto(account) {
            Err(e) => Err(Status::invalid_argument(format!("{}", e))),
            Ok(input) => {
                let record = sqlx::query_as!(
                    AccountRecord,
                    r#"
                      INSERT INTO accounts (owner_id, slug, name)
                      VALUES ($1, $2, $3)
                      RETURNING *
                    "#,
                    input.owner_id, input.slug, input.name
                ).fetch_optional(&self.pool).await;

                match record {
                    Ok(record) => match record {
                        Some(record) => Ok(record.to_proto()),
                        None => Err(Status::failed_precondition("Account not created")),
                    },
                    Err(_) => Err(Status::internal("An internal error occurred")),

                }
            }
        }
    }
}
