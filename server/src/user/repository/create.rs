use tonic::Status;

use crate::user::UserRecord;
use crate::user::pb::user::OptionalName;

use super::super::pb::User;

use super::Repository;

impl Repository {
    pub async fn on_create_user(&self, input: User) -> Result<User, Status> {
        let name = match input.optional_name {
            Some(OptionalName::Name(name)) => Some(name),
            None => None,
        };

        let record = sqlx::query_as!(
            UserRecord,
            r#"
              INSERT INTO users (email, username, name)
              VALUES ($1, $2, $3)
              RETURNING *
            "#,
            input.email, input.username, name
        ).fetch_optional(&self.pool).await;

        match record {
            Ok(record) => match record {
                Some(record) => Ok(record.to_proto()),
                None => Err(Status::failed_precondition("User not created")),
            },
            Err(_) => Err(Status::internal("An internal error occurred")),

        }
    }
}
