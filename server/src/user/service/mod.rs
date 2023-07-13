use tonic::{Request, Response, Status};

use super::pb::{CreateUserRequest, User, GetUserRequest, UpdateUserRequest, DeleteUserRequest};
use super::pb::user_service_server::UserService;

mod create;
mod delete;
mod get;
mod update;

#[derive(Default)]
pub struct Service {}

impl Service {
    pub fn new() -> Service {
        Service::default()
    }
}

#[tonic::async_trait]
impl UserService for Service {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        self.on_create_user(request)
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<User>, Status> {
        self.on_get_user(request)
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<User>, Status> {
        self.on_update_user(request)
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<()>, Status> {
        self.on_delete_user(request)
    }
}

