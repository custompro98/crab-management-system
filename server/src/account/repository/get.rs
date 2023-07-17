use tonic::Status;

use crate::account::AccountRecord;

use super::super::super::pb::account::Account;

use super::Repository;

impl Repository {
    pub async fn on_get_account(&self, id: i32) -> Result<Account, Status> {
        let record = sqlx::query_as!(
            AccountRecord,
            r#"
              SELECT *
              FROM accounts
              WHERE id = $1
                AND deleted_at IS NULL
              LIMIT 1
            "#,
            id
        ).fetch_optional(&self.pool).await;

        match record {
            Ok(record) => match record {
                Some(record) => Ok(record.to_proto()),
                None => Err(Status::not_found("Account not found").into()),
            },
            Err(_) => Err(Status::internal("An internal error occurred")),
        }
    }
}
