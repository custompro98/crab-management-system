use tonic::{Response, Status, Code};

use super::super::super::pb::account::Account;
use super::Service;

impl Service {
    pub async fn get(&self, id: i32) -> Result<Response<Account>, Status> {
        let account = self.repository.get(id).await;

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
        let user = self.user_service.get(account.owner_id).await?;
        account.owner = Some(user.get_ref().to_owned());

        Ok(Response::new(account))
    }
}
