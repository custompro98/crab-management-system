use std::time::SystemTime;

use chrono::{DateTime, Utc};
use validator::Validate;

use super::pb::field::field::{
    OptionalDeletedAt, OptionalUpdatedAt, OptionalValue, Type as FieldTypePb,
};
use super::pb::field::Field;

use super::error::ValidationError;

pub mod handler;
mod repository;
pub mod service;

#[derive(serde::Serialize, Default)]
enum FieldType {
    #[default]
    Unspecified,
    Text,
}

#[derive(Default)]
enum FieldValue {
    #[default]
    Empty,
    Text(String),
}

#[derive(sqlx::FromRow, validator::Validate, Default)]
struct FieldRecord {
    id: i32,
    account_id: i32,
    name: String,
    value: Option<String>,
    #[validate(custom(
        function = "validate_field_type_specified",
        message = "Field type must be specified"
    ))]
    field_type: FieldType,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    deleted_at: Option<DateTime<Utc>>,
}

impl FieldRecord {
    fn to_proto(self) -> Field {
        Field {
            id: self.id,
            account_id: self.account_id,
            name: self.name,
            optional_value: match self.value {
                Some(value) => Some(OptionalValue::Value(value)),
                None => None,
            },
            r#type: self.field_type.to_proto().into(),
            account: None,

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

    pub fn from_proto(proto: Field) -> Result<FieldRecord, ValidationError> {
        // If there is no created_at, we're creating the record, otherwise it remains unchanged
        let created_at: DateTime<Utc> = match proto.created_at.as_str() {
            "" => SystemTime::now().into(),
            _ => DateTime::parse_from_rfc3339(&proto.created_at)?.into(),
        };

        // Only allow valid combinations of FieldType and Option<Value>
        let field_value = match (&proto.r#type(), &proto.optional_value) {
            (FieldTypePb::Unspecified, _) => FieldValue::Empty,
            (_, None) => FieldValue::Empty,
            (FieldTypePb::Text, Some(OptionalValue::Value(value))) => {
                FieldValue::Text(value.clone())
            }
        };

        let record = FieldRecord {
            id: proto.id,
            account_id: proto.account_id,
            name: proto.name.clone(),
            value: match field_value {
                FieldValue::Empty => None,
                FieldValue::Text(value) => Some(value),
            },
            field_type: FieldType::from_proto(proto.r#type()),
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

impl std::string::ToString for FieldType {
    fn to_string(&self) -> String {
        match self {
            FieldType::Unspecified => String::from("unspecified"),
            FieldType::Text => String::from("text"),
        }
    }
}

impl Into<String> for FieldType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl From<String> for FieldType {
    fn from(val: String) -> Self {
        match val.as_str() {
            "unspecified" => FieldType::Unspecified,
            "text" => FieldType::Text,
            &_ => todo!(),
        }
    }
}

impl FieldType {
    pub fn to_proto(&self) -> FieldTypePb {
        match self {
            FieldType::Text => FieldTypePb::Text,
            FieldType::Unspecified => FieldTypePb::Unspecified,
        }
    }

    pub fn from_proto(proto: FieldTypePb) -> FieldType {
        match proto {
            FieldTypePb::Unspecified => FieldType::Unspecified,
            FieldTypePb::Text => FieldType::Text,
        }
    }
}

fn validate_field_type_specified(value: &FieldType) -> Result<(), validator::ValidationError> {
    match value {
        FieldType::Unspecified => Err(validator::ValidationError::new("invalid_field_type")),
        _ => Ok(()),
    }
}
