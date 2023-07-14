use tonic::{Request, Response, Status};

use super::super::pb::{GetUserRequest, User};
use super::Service;

impl Service {
    pub async fn on_get_user(&self, request: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        let user = self.repository.get(request.get_ref().id).await;

        match user {
            Ok(user) => Ok(Response::new(user)),
            Err(_) => Err(Status::internal("An internal error occurred")),
        }
    }
}
