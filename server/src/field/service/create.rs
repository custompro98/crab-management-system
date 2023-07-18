use tonic::{Response, Status, Code};

use super::super::super::pb::field::Field;
use super::Service;

impl Service {
    pub async fn create(
        &self,
        field: Field,
    ) -> Result<Response<Field>, Status> {
        let field = self.repository.create(field).await;

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
        let account = self.account_service.get(field.account_id).await?;
        field.account = Some(account.get_ref().to_owned());

        Ok(Response::new(field))
    }
}
