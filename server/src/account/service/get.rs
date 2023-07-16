use tonic::{Request, Response, Status, Code};

use super::super::pb::{GetAccountRequest, Account};
use super::Service;

impl Service {
    pub async fn on_get_account(&self, request: Request<GetAccountRequest>) -> Result<Response<Account>, Status> {
        let account = self.repository.get(request.get_ref().id).await;

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
