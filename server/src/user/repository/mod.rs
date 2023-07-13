use sqlx::PgPool;

use super::pb::User;

mod get;

pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Repository {
        Repository { pool }
    }

    pub async fn get(&self, id: i32) -> Result<User, Box<dyn std::error::Error>> {
        Ok(self.on_get_user(id).await?)
    }
}
