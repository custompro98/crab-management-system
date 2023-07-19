use tonic::Status;

use crate::user::UserRecord;

use super::super::super::pb::user::User;

use super::Repository;

impl Repository {
    pub async fn batch_get(&self, user_ids: &Vec<i32>) -> Result<Vec<User>, Status> {
        let records = sqlx::query_as!(
            UserRecord,
            r#"
              SELECT *
              FROM users
              WHERE id = ANY($1)
                AND deleted_at IS NULL
            "#,
            &user_ids[..]
        ).fetch_all(&self.pool).await;

        match records {
            Ok(records) => {
                let mut collection: Vec<User> = vec![];
                for record in records {
                    collection.push(record.to_proto());
                }

                Ok(collection)
            },
            Err(_) => Err(Status::internal("An internal error occurred")),
        }
    }
}
