use tonic::{Request, Response, Status, Code};

use super::super::super::pb::account::{UpdateAccountRequest, Account};
use super::Service;

impl Service {
    pub async fn on_update_account(
        &self,
        request: Request<UpdateAccountRequest>,
    ) -> Result<Response<Account>, Status> {
        if let None = &request.get_ref().account {
            return Err(Status::invalid_argument("Account must be provided"));
        }

        let account = self.repository.update(request.get_ref().account.to_owned().unwrap()).await;

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
