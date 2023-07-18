use tonic::Status;

use crate::user::UserRecord;

use super::super::super::pb::user::User;

use super::Repository;

impl Repository {
    pub async fn get(&self, id: i32) -> Result<User, Status> {
        let record = sqlx::query_as!(
            UserRecord,
            r#"
              SELECT *
              FROM users
              WHERE id = $1
                AND deleted_at IS NULL
              LIMIT 1
            "#,
            id
        ).fetch_optional(&self.pool).await;

        match record {
            Ok(record) => match record {
                Some(record) => Ok(record.to_proto()),
                None => Err(Status::not_found("User not found").into()),
            },
            Err(_) => Err(Status::internal("An internal error occurred")),
        }
    }
}
