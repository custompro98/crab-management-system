use sqlx::PgPool;
use tonic::{Request, Response, Status};

use super::super::pb::account::{CreateAccountRequest, Account, GetAccountRequest, UpdateAccountRequest, DeleteAccountRequest};
use super::super::pb::account::account_service_server::AccountService;

mod create;
mod get;
mod update;
mod delete;

pub struct Service {
    repository: super::repository::Repository,
    users: super::super::user::repository::Repository,
}

impl Service {
    pub fn new(pool: PgPool) -> Service {
        Service {
            repository: super::repository::Repository::new(pool.clone()),
            users: super::super::user::repository::Repository::new(pool.clone()),
        }
    }
}

#[tonic::async_trait]
impl AccountService for Service {
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<Account>, Status> {
        let mut account = self.on_create_account(request).await?;
        let user = self.users.get(account.get_ref().owner_id).await?;

        account.get_mut().owner = Some(user);

        Ok(account)
    }

    async fn get_account(
        &self,
        request: Request<GetAccountRequest>,
    ) -> Result<Response<Account>, Status> {
        let mut account = self.on_get_account(request).await?;
        let user = self.users.get(account.get_ref().owner_id).await?;

        account.get_mut().owner = Some(user);

        Ok(account)
    }

    async fn update_account(
        &self,
        request: Request<UpdateAccountRequest>,
    ) -> Result<Response<Account>, Status> {
        let mut account = self.on_update_account(request).await?;
        let user = self.users.get(account.get_ref().owner_id).await?;

        account.get_mut().owner = Some(user);

        Ok(account)
    }

    async fn delete_account(
        &self,
        request: Request<DeleteAccountRequest>,
    ) -> Result<Response<()>, Status> {
        self.on_delete_account(request).await
    }
}
