use sqlx::PgPool;
use tonic::{Request, Response, Status};

use super::pb::{CreateUserRequest, User, GetUserRequest, UpdateUserRequest, DeleteUserRequest};
use super::pb::user_service_server::UserService;

mod create;
mod get;
mod update;
mod delete;

pub struct Service {
    repository: super::repository::Repository,
}

impl Service {
    pub fn new(pool: PgPool) -> Service {
        Service {
            repository: super::repository::Repository::new(pool)
        }
    }
}

#[tonic::async_trait]
impl UserService for Service {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        self.on_create_user(request).await
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<User>, Status> {
        self.on_get_user(request).await
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<User>, Status> {
        self.on_update_user(request).await
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<()>, Status> {
        self.on_delete_user(request).await
    }
}
