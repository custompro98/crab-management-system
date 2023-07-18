use tonic::{Response, Status, Code};

use super::Service;

impl Service {
    pub async fn delete(
        &self,
        id: i32,
    ) -> Result<Response<()>, Status> {
        let success = self.repository.delete(id).await;

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
