use sqlx::PgPool;
use tonic::Status;

use super::super::pb::account::Account;

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

    pub async fn create(&self, account: Account) -> Result<Account, Status> {
        Ok(self.on_create_account(account).await?)
    }

    pub async fn get(&self, id: i32) -> Result<Account, Status> {
        Ok(self.on_get_account(id).await?)
    }

    pub async fn update(&self, account: Account) -> Result<Account, Status> {
        Ok(self.on_update_account(account).await?)
    }

    pub async fn delete(&self, id: i32) -> Result<(), Status> {
        Ok(self.on_delete_account(id).await?)
    }
}
