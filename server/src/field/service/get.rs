use tonic::{Request, Response, Status, Code};

use super::super::super::pb::field::{GetFieldRequest, Field};
use super::Service;

impl Service {
    pub async fn on_get_field(&self, request: Request<GetFieldRequest>) -> Result<Response<Field>, Status> {
        let field = self.repository.get(request.get_ref().id).await;

        match field {
            Ok(field) => Ok(Response::new(field)),
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
