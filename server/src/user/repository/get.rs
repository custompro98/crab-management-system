use crate::user::UserRecord;

use super::super::pb::User;

use super::Repository;

impl Repository {
    pub async fn on_get_user(&self, id: i32) -> Result<User, Box<dyn std::error::Error>> {
        let record = sqlx::query_as!(
            UserRecord,
            r#"
              SELECT *
              FROM users
              WHERE id = $1
              LIMIT 1
            "#,
            id
        ).fetch_one(&self.pool).await?;

        Result::Ok(record.to_proto())
    }
}
