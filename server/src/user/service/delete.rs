use tonic::{Request, Response, Status, Code};

use super::super::pb::DeleteUserRequest;
use super::Service;

impl Service {
    pub async fn on_delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<()>, Status> {
        let success = self.repository.delete(request.get_ref().id).await;

        match success {
            Ok(_) => Ok(Response::new(())),
            Err(status) => match &status.code() {
                Code::NotFound => Err(status),
                Code::InvalidArgument => Err(status),
                Code::AlreadyExists => Err(status),
                Code::FailedPrecondition => Err(status),
                Code::PermissionDenied => Err(status),
                Code::Unauthenticated => Err(status),
                _ => Err(Status::internal("An internal error occurred")),
            }
        }
    }
}
