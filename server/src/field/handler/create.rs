use tonic::{Request, Response, Status, Code};

use super::super::super::pb::field::{CreateFieldRequest, Field};
use super::Handler;

impl Handler {
    pub async fn on_create_field(
        &self,
        request: Request<CreateFieldRequest>,
    ) -> Result<Response<Field>, Status> {
        if let None = &request.get_ref().field {
            return Err(Status::invalid_argument("Field must be provided"));
        }

        let field = self.repository.create(request.get_ref().field.to_owned().unwrap()).await;

        if let Err(status) = field {
            return match &status.code() {
                Code::NotFound => Err(status),
                Code::InvalidArgument => Err(status),
                Code::AlreadyExists => Err(status),
                Code::FailedPrecondition => Err(status),
                Code::PermissionDenied => Err(status),
                Code::Unauthenticated => Err(status),
                _ => Err(Status::internal("An internal error occurred"))
            };
        }

        let mut field = field.unwrap();
        let account = self.accounts.get(field.account_id).await?;
        field.account = Some(account);

        Ok(Response::new(field))
    }
}
