use tonic::{Request, Response, Status, Code};

use super::super::pb::{CreateUserRequest, User};
use super::Service;

impl Service {
    pub async fn on_create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        if let None = &request.get_ref().user {
            return Err(Status::invalid_argument("User must be provided"));
        }

        let user = self.repository.create(request.get_ref().user.to_owned().unwrap()).await;

        match user {
            Ok(user) => Ok(Response::new(user)),
            Err(status) => match &status.code() {
                Code::NotFound => Err(status),
                Code::InvalidArgument => Err(status),
                Code::AlreadyExists => Err(status),
                Code::FailedPrecondition => Err(status),
                Code::PermissionDenied => Err(status),
                Code::Unauthenticated => Err(status),
                _ => Err(Status::internal("An internal error occurred")),
            },
        }
    }
}
