use std::collections::HashMap;

use tonic::{Response, Status, Code};

use super::super::super::pb::field::batch_get_fields_response::maybe_field::OptionalField;
use super::super::super::pb::field::{batch_get_fields_response::MaybeField, Field, BatchGetFieldsResponse};

use super::Service;

impl Service {
    pub async fn batch_get(&self, field_ids: &Vec<i32>) -> Result<Response<BatchGetFieldsResponse>, Status> {
        let fields = self.repository.batch_get(field_ids).await;

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

        let mut fields_by_id: HashMap<i32, Field> = HashMap::new();
        for field in fields.unwrap() {
            fields_by_id.insert(field.id, field);
        }

        let mut maybe_fields: Vec<MaybeField> = Vec::new();
        for field_id in field_ids {
            match fields_by_id.get(&field_id) {
                Some(field) => maybe_fields.push(MaybeField {
                    optional_field: Some(OptionalField::Field(field.to_owned()))
                }),
                None => maybe_fields.push(MaybeField { optional_field: None }),
            }
        }

        /* let account = self.account_service.get(field.account_id).await?;
        field.account = Some(account.get_ref().to_owned()); */

        Ok(Response::new(BatchGetFieldsResponse {
            fields: maybe_fields,
        }))
    }
}
