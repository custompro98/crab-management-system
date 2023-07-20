use tonic::{Response, Status, Code};

use super::super::super::pb::field::batch_get_fields_response::maybe_field::OptionalField::Field;
use super::super::super::pb::field_group::FieldGroup;
use super::Service;

impl Service {
    pub async fn create(
        &self,
        field_group: FieldGroup,
    ) -> Result<Response<FieldGroup>, Status> {
        let field_ids: Vec<i32> = field_group.fields.iter().map(|field| field.id).collect();
        let field_group = self.repository.create(field_group, &field_ids).await;

        if let Err(status) = field_group {
            return match &status.code() {
                Code::NotFound => Err(status),
                Code::InvalidArgument => Err(status),
                Code::AlreadyExists => Err(status),
                Code::FailedPrecondition => Err(status),
                Code::PermissionDenied => Err(status),
                Code::Unauthenticated => Err(status),
                _ => Err(Status::internal(status.message()))
            };
        }

        let mut field_group = field_group.unwrap();
        let account = self.account_service.get(field_group.account_id).await?;
        field_group.account = Some(account.get_ref().to_owned());

        let fields = self.field_service.batch_get(&field_ids).await?;
        field_group.fields = fields
            .get_ref()
            .fields
            .iter()
            .map(|maybe| {
                match &maybe.optional_field {
                    Some(Field(field)) => Some(field.to_owned()),
                    None => None,
                }
            })
            .filter(|maybe| maybe.is_some())
            .map(|field| field.unwrap())
            .collect();

        Ok(Response::new(field_group))
    }
}
