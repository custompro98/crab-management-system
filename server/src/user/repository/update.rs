use crate::user::UserRecord;

use super::super::pb::User;

use super::Repository;

impl Repository {
    pub async fn on_update_user(&self, user: User) -> Result<User, Box<dyn std::error::Error>> {
        let input = UserRecord::from_proto(user)?;

        let record = sqlx::query_as!(
            UserRecord,
            r#"
              UPDATE users
              SET email = $2, username = $3, name = $4, updated_at = now()
              WHERE id = $1
              RETURNING *
            "#,
            input.id, input.email, input.username, input.name
        ).fetch_one(&self.pool).await?;

        Result::Ok(record.to_proto())
    }
}

