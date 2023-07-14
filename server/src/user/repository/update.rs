use tonic::Status;

use crate::user::UserRecord;

use super::super::pb::User;

use super::Repository;

impl Repository {
    pub async fn on_update_user(&self, user: User) -> Result<User, Status> {
        match UserRecord::from_proto(user) {
            Err(_) => Err(Status::invalid_argument("User is invalid")),
            Ok(input) => {
                let record = sqlx::query_as!(
                    UserRecord,
                    r#"
                      UPDATE users
                      SET email = $2, username = $3, name = $4, updated_at = now()
                      WHERE id = $1
                        AND deleted_at IS NULL
                      RETURNING *
                    "#,
                    input.id, input.email, input.username, input.name
                ).fetch_optional(&self.pool).await;

                match record {
                    Ok(record) => match record {
                        Some(record) => Ok(record.to_proto()),
                        None => Err(Status::not_found("User not found")),
                    },
                    Err(_) => Err(Status::internal("An internal error occurred")),
                }
            }
        }

    }
}
