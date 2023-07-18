use tonic::{Request, Response, Status, Code};

use super::super::super::pb::field::{CreateFieldRequest, Field};
use super::Service;

impl Service {
    pub async fn on_create_field(
        &self,
        request: Request<CreateFieldRequest>,
    ) -> Result<Response<Field>, Status> {
        if let None = &request.get_ref().field {
            return Err(Status::invalid_argument("Field must be provided"));
        }

        let field = self.repository.create(request.get_ref().field.to_owned().unwrap()).await;

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
