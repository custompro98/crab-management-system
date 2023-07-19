use tonic::{Response, Status, Code};

use super::super::super::pb::field_group::FieldGroup;
use super::Service;

impl Service {
    pub async fn get(&self, id: i32) -> Result<Response<FieldGroup>, Status> {
        let field_group = self.repository.get(id).await;

        if let Err(status) = field_group {
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

        let mut field_group = field_group.unwrap();
        let account = self.account_service.get(field_group.account_id).await?;
        field_group.account = Some(account.get_ref().to_owned());

        Ok(Response::new(field_group))
    }
}
