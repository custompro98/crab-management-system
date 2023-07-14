use tonic::{Request, Response, Status};

use super::super::pb::{CreateUserRequest, User};
use super::Service;

impl Service {
    pub async fn on_create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        match &request.get_ref().user {
            None => Err(Status::invalid_argument("User must be provided")),
            Some(user) => {
                let user = self.repository.create(user).await;

                match user {
                    Ok(user) => Ok(Response::new(user)),
                    Err(_) => Err(Status::internal("An internal error occurred")),
                }
            }
        }
    }
}
