use tonic::{Request, Response, Status, Code};

use super::super::super::pb::account::{GetAccountRequest, Account};
use super::Service;

impl Service {
    pub async fn on_get_account(&self, request: Request<GetAccountRequest>) -> Result<Response<Account>, Status> {
        let account = self.repository.get(request.get_ref().id).await;

        if let Err(status) = account {
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

        let mut account = account.unwrap();
        let user = self.users.get(account.owner_id).await?;
        account.owner = Some(user);

        Ok(Response::new(account))
    }
}
