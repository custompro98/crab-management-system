use std::time::SystemTime;

use chrono::{DateTime, Utc};
use validator::Validate;

use super::pb::field_group::field_group::{OptionalDeletedAt, OptionalUpdatedAt};
use super::pb::field_group::FieldGroup;

use super::error::ValidationError;

pub mod handler;
mod repository;
pub mod service;

#[derive(sqlx::FromRow, validator::Validate, Default)]
struct FieldGroupRecord {
    id: i32,
    account_id: i32,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    deleted_at: Option<DateTime<Utc>>,
}

impl FieldGroupRecord {
    fn to_proto(self) -> FieldGroup {
        FieldGroup {
            id: self.id,
            account_id: self.account_id,
            name: self.name,
            account: None,
            fields: Vec::new(),

            created_at: self.created_at.to_rfc3339(),
            optional_updated_at: match self.updated_at {
                Some(timestamp) => Some(OptionalUpdatedAt::UpdatedAt(timestamp.to_rfc3339())),
                None => None,
            },
            optional_deleted_at: match self.deleted_at {
                Some(timestamp) => Some(OptionalDeletedAt::DeletedAt(timestamp.to_rfc3339())),
                None => None,
            },
        }
    }

    pub fn from_proto(proto: FieldGroup) -> Result<FieldGroupRecord, ValidationError> {
        // If there is no created_at, we're creating the record, otherwise it remains unchanged
        let created_at: DateTime<Utc> = match proto.created_at.as_str() {
            "" => SystemTime::now().into(),
            _ => DateTime::parse_from_rfc3339(&proto.created_at)?.into(),
        };

        let record = FieldGroupRecord {
            id: proto.id,
            account_id: proto.account_id,
            name: proto.name,
            created_at,
            updated_at: match proto.optional_updated_at {
                Some(OptionalUpdatedAt::UpdatedAt(timestamp)) => {
                    Some(DateTime::parse_from_rfc3339(&timestamp)?.into())
                }
                None => None,
            },
            deleted_at: match proto.optional_deleted_at {
                Some(OptionalDeletedAt::DeletedAt(timestamp)) => {
                    Some(DateTime::parse_from_rfc3339(&timestamp)?.into())
                }
                None => None,
            },
        };

        match record.validate() {
            Ok(_) => Ok(record),
            Err(e) => Err(e.into()),
        }
    }
}
