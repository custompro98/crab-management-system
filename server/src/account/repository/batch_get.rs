use tonic::Status;

use crate::account::AccountRecord;

use super::super::super::pb::account::Account;

use super::Repository;

impl Repository {
    pub async fn batch_get(&self, account_ids: &Vec<i32>) -> Result<Vec<Account>, Status> {
        let records = sqlx::query_as!(
            AccountRecord,
            r#"
              SELECT *
              FROM accounts
              WHERE id = ANY($1)
                AND deleted_at IS NULL
            "#,
            &account_ids[..]
        ).fetch_all(&self.pool).await;

        match records {
            Ok(records) => {
                let mut collection: Vec<Account> = vec![];
                for record in records {
                    collection.push(record.to_proto());
                }

                Ok(collection)
            },
            Err(_) => Err(Status::internal("An internal error occurred")),
        }
    }
}
