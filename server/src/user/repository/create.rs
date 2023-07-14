use crate::user::UserRecord;

use super::super::pb::User;

use super::Repository;

impl Repository {
    pub async fn on_create_user(&self, user: User) -> Result<User, Box<dyn std::error::Error>> {
        let input = UserRecord::from_proto(user)?;

        let record = sqlx::query_as!(
            UserRecord,
            r#"
              INSERT INTO users (email, username, name)
              VALUES ($1, $2, $3)
              RETURNING *
            "#,
            input.email, input.username, input.name
        ).fetch_one(&self.pool).await?;

        Result::Ok(record.to_proto())
    }
}
