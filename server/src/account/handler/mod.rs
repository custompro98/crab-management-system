use sqlx::PgPool;
use tonic::{Request, Response, Status};

use super::super::pb::account::{CreateAccountRequest, Account, GetAccountRequest, UpdateAccountRequest, DeleteAccountRequest};
use super::super::pb::account::account_service_server::AccountService;

pub struct Handler {
    service: super::service::Service,
}

impl Handler {
    pub fn new(pool: PgPool) -> Handler {
        Handler {
            service: super::service::Service::new(pool.clone()),
        }
    }
}

#[tonic::async_trait]
impl AccountService for Handler {
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<Account>, Status> {
        self.service.create(request.get_ref().account.to_owned().expect("Account must be defined")).await
    }

    async fn get_account(
        &self,
        request: Request<GetAccountRequest>,
    ) -> Result<Response<Account>, Status> {
        self.service.get(request.get_ref().id).await
    }

    async fn update_account(
        &self,
        request: Request<UpdateAccountRequest>,
    ) -> Result<Response<Account>, Status> {
        self.service.update(request.get_ref().account.to_owned().expect("Account must be defined")).await
    }

    async fn delete_account(
        &self,
        request: Request<DeleteAccountRequest>,
    ) -> Result<Response<()>, Status> {
        self.service.delete(request.get_ref().id).await
    }
}
