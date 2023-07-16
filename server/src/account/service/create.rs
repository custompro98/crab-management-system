use tonic::{Request, Response, Status, Code};

use super::super::pb::{CreateAccountRequest, Account};
use super::Service;

impl Service {
    pub async fn on_create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<Account>, Status> {
        if let None = &request.get_ref().account {
            return Err(Status::invalid_argument("Account must be provided"));
        }

        let account = self.repository.create(request.get_ref().account.to_owned().unwrap()).await;

        match account {
            Ok(account) => Ok(Response::new(account)),
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
