use crate::user::pb::user::{OptionalName, OptionalUpdatedAt, OptionalDeletedAt};

use super::super::pb::User;

use super::Repository;

impl Repository {
    pub async fn on_get_user(&self, id: i32) -> Result<User, Box<dyn std::error::Error>> {
        let record = sqlx::query!(
            r#"
              SELECT *
              FROM users
              WHERE id = $1
              LIMIT 1
            "#,
            id,
        ).fetch_one(&self.pool).await?;

        Result::Ok(User{
            id: record.id,
            email: record.email,
            username: record.username,
            created_at: record.created_at.to_string(),

            optional_name: match record.name {
                Some(name) => Some(OptionalName::Name(name)),
                None => None,
            },
            optional_updated_at: match record.updated_at {
                Some(timestamp) => Some(OptionalUpdatedAt::UpdatedAt(timestamp.to_string())),
                None => None,
            },
            optional_deleted_at: match record.deleted_at {
                Some(timestamp) => Some(OptionalDeletedAt::DeletedAt(timestamp.to_string())),
                None => None,
            }
        })
    }
}
