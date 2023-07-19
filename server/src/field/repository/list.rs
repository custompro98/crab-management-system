use tonic::Status;

use super::super::super::field::FieldRecord;
use super::super::super::pb::field::list_fields_request::Filter;

use super::super::super::pb::field::Field;

use super::Repository;

impl Repository {
    pub async fn list(&self, filter: Filter) -> Result<Vec<Field>, Status> {
        let records = sqlx::query_as!(
            FieldRecord,
            r#"
              SELECT *
              FROM fields
              WHERE account_id = $1
                AND deleted_at IS NULL
              LIMIT 1
            "#,
            filter.account_id
        ).fetch_all(&self.pool).await;

        match records {
            Ok(records) => {
                let mut collection: Vec<Field> = vec![];
                for record in records {
                    collection.push(record.to_proto());
                }

                Ok(collection)
            },
            Err(_) => Err(Status::internal("An internal error occurred")),
        }
    }
}
