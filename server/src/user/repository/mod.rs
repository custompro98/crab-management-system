use sqlx::PgPool;
use tonic::Status;

use super::super::pb::user::User;

mod create;
mod get;
mod update;
mod delete;

pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Repository {
        Repository { pool }
    }

    pub async fn create(&self, user: User) -> Result<User, Status> {
        Ok(self.on_create_user(user).await?)
    }

    pub async fn get(&self, id: i32) -> Result<User, Status> {
        Ok(self.on_get_user(id).await?)
    }

    pub async fn update(&self, user: User) -> Result<User, Status> {
        Ok(self.on_update_user(user).await?)
    }

    pub async fn delete(&self, id: i32) -> Result<(), Status> {
        Ok(self.on_delete_user(id).await?)
    }
}
