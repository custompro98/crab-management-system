use std::collections::HashMap;

use tonic::{Response, Status, Code};

use super::super::super::pb::account::{batch_get_accounts_response::maybe_account::OptionalAccount, Account};

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
        for field in fields.to_owned().unwrap() {
            fields_by_id.insert(field.id, field);
        }

        let accounts = self.account_service
            .batch_get(
                &fields
                    .unwrap()
                    .iter()
                    .map(|acc| acc.account_id )
                    .collect()
            ).await?;

        let mut accounts_by_id: HashMap<i32, Account> = HashMap::new();
        for account in &accounts.get_ref().accounts {
            if let Some(OptionalAccount::Account(account)) = &account.optional_account {
                accounts_by_id.insert(account.id, account.to_owned());
            }
        }

        let mut maybe_fields: Vec<MaybeField> = Vec::new();
        for field_id in field_ids {
            match fields_by_id.get_mut(&field_id) {
                Some(field) => {
                    let account = accounts_by_id.get(&field.account_id);

                    if let Some(account) = account {
                        field.account = Some(account.to_owned());
                    }

                    maybe_fields.push(MaybeField {
                        optional_field: Some(OptionalField::Field(field.to_owned()))
                    })
                },
                None => maybe_fields.push(MaybeField { optional_field: None }),
            }
        }

        Ok(Response::new(BatchGetFieldsResponse {
            fields: maybe_fields,
        }))
    }
}
