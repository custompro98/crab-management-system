use sqlx::PgPool;
use tonic::{Request, Response, Status};

use super::super::pb::user::{CreateUserRequest, User, GetUserRequest, UpdateUserRequest, DeleteUserRequest};
use super::super::pb::user::user_service_server::UserService;

pub struct Handler {
    service: super::service::Service,
}

impl Handler {
    pub fn new(pool: PgPool) -> Handler {
        Handler {
            service: super::service::Service::new(pool)
        }
    }
}

#[tonic::async_trait]
impl UserService for Handler {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        self.service.create(request.get_ref().user.to_owned().expect("User must be defined")).await }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<User>, Status> {
        self.service.get(request.get_ref().id).await
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<User>, Status> {
        self.service.update(request.get_ref().user.to_owned().expect("User must be defined")).await
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<()>, Status> {
        self.service.delete(request.get_ref().id).await
    }
}
