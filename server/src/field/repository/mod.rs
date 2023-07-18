use sqlx::PgPool;
use tonic::Status;

use super::super::pb::field::Field;

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

    pub async fn create(&self, field: Field) -> Result<Field, Status> {
        Ok(self.on_create_field(field).await?)
    }

    pub async fn get(&self, id: i32) -> Result<Field, Status> {
        Ok(self.on_get_field(id).await?)
    }

    pub async fn update(&self, field: Field) -> Result<Field, Status> {
        Ok(self.on_update_field(field).await?)
    }

    pub async fn delete(&self, id: i32) -> Result<(), Status> {
        Ok(self.on_delete_field(id).await?)
    }
}
