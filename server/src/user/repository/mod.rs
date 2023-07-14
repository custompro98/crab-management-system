use sqlx::PgPool;

use super::pb::User;

mod create;
mod get;
mod delete;

pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Repository {
        Repository { pool }
    }

    pub async fn create(&self, user: User) -> Result<User, Box<dyn std::error::Error>> {
        Ok(self.on_create_user(user).await?)
    }

    pub async fn get(&self, id: i32) -> Result<User, Box<dyn std::error::Error>> {
        Ok(self.on_get_user(id).await?)
    }

    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.on_delete_user(id).await?)
    }
}
