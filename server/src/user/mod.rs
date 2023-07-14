use std::time::SystemTime;

use chrono::{DateTime, Utc};
use validator::Validate;

use self::pb::user::{OptionalDeletedAt, OptionalName, OptionalUpdatedAt};
use self::pb::User;

use super::error::ValidationError;

pub mod pb {
    tonic::include_proto!("user");
}

mod repository;
pub mod service;

#[derive(sqlx::FromRow, validator::Validate)]
struct UserRecord {
    id: i32,
    #[validate(email)]
    email: String,
    username: String,
    name: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    deleted_at: Option<DateTime<Utc>>,
}

impl UserRecord {
    fn to_proto(self) -> User {
        User {
            id: self.id,
            email: self.email,
            username: self.username,
            created_at: self.created_at.to_string(),

            optional_name: match self.name {
                Some(name) => Some(OptionalName::Name(name)),
                None => None,
            },
            optional_updated_at: match self.updated_at {
                Some(timestamp) => Some(OptionalUpdatedAt::UpdatedAt(timestamp.to_string())),
                None => None,
            },
            optional_deleted_at: match self.deleted_at {
                Some(timestamp) => Some(OptionalDeletedAt::DeletedAt(timestamp.to_string())),
                None => None,
            },
        }
    }

    pub fn from_proto(proto: User) -> Result<UserRecord, ValidationError> {
        // If the id is 0, we are creating a user for the first time
        let created_at: DateTime<Utc> = match proto.id {
            0 => SystemTime::now().into(),
            _ => DateTime::parse_from_rfc3339(&proto.created_at)?.into(),
        };

        let record = UserRecord {
            id: proto.id,
            email: proto.email,
            username: proto.username,
            name: match proto.optional_name {
                Some(OptionalName::Name(name)) => Some(name),
                None => None,
            },
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
