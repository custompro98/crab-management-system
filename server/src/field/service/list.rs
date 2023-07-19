use tonic::{Response, Status, Code};

use super::super::super::pb::field::{ListFieldsResponse, list_fields_request::Filter};
use super::Service;

impl Service {
    pub async fn list(&self, filter: Filter) -> Result<Response<ListFieldsResponse>, Status> {
        let fields = self.repository.list(filter).await;

        if let Err(status) = fields {
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

        let fields = fields.unwrap();
        /* let account = self.account_service.get(field.account_id).await?;
        field.account = Some(account.get_ref().to_owned()); */

        Ok(Response::new(ListFieldsResponse { fields }))
    }
}
