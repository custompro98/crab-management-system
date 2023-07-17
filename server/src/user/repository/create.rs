use tonic::Status;

use crate::user::UserRecord;

use super::super::super::pb::user::User;

use super::Repository;

impl Repository {
    pub async fn on_create_user(&self, user: User) -> Result<User, Status> {
        match UserRecord::from_proto(user) {
            Err(e) => Err(Status::invalid_argument(format!("{}", e))),
            Ok(input) => {
                let record = sqlx::query_as!(
                    UserRecord,
                    r#"
                      INSERT INTO users (email, username, name)
                      VALUES ($1, $2, $3)
                      RETURNING *
                    "#,
                    input.email, input.username, input.name
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
    }
}
