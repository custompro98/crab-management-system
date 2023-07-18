use std::time::SystemTime;

use chrono::{DateTime, Utc};

use super::pb::account::account::{OptionalDeletedAt, OptionalUpdatedAt};
use super::pb::account::Account;
use super::pb::user::User;

use super::error::ValidationError;

pub mod handler;
mod repository;
pub mod service;

#[derive(sqlx::FromRow, validator::Validate)]
struct AccountRecord {
    id: i32,
    owner_id: i32,
    slug: String,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    deleted_at: Option<DateTime<Utc>>,
}

impl AccountRecord {
    fn to_proto(self) -> Account {
        Account {
            id: self.id,
            owner_id: self.owner_id,
            slug: self.slug,
            name: self.name,

            owner: None::<User>,
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

    pub fn from_proto(proto: Account) -> Result<AccountRecord, ValidationError> {
        // If there is no created_at, we're creating the record, otherwise it remains unchanged
        let created_at: DateTime<Utc> = match proto.created_at.as_str() {
            "" => SystemTime::now().into(),
            _ => DateTime::parse_from_rfc3339(&proto.created_at)?.into(),
        };

        let record = AccountRecord {
            id: proto.id,
            owner_id: proto.owner_id,
            slug: proto.slug,
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

        Ok(record)
    }
}
