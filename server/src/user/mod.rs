use tonic::{Request, Response, Status};

use self::pb::{CreateUserRequest, User, GetUserRequest, UpdateUserRequest, DeleteUserRequest};
use self::pb::user_service_server::UserService;

pub mod pb {
    tonic::include_proto!("user");
}

mod create;
mod get;
mod update;
mod delete;

#[derive(Default)]
pub struct Service {}

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

