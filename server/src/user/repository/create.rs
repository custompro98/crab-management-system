use crate::user::UserRecord;

use super::super::pb::User;

use super::Repository;

impl Repository {
    pub async fn on_create_user(&self, user: &User) -> Result<User, Box<dyn std::error::Error>> {
        let name = match user.optional_name {
            Some(_) => Some("mitch"),
            None => None,
        };

        let record = sqlx::query_as!(
            UserRecord,
            r#"
              INSERT INTO users (email, username, name)
              VALUES ($1, $2, $3)
              RETURNING *
            "#,
            user.email, user.username, name
        ).fetch_one(&self.pool).await?;

        Result::Ok(record.to_proto())
    }
}
