use sqlx::PgPool;
use tonic::{Request, Response, Status};

use super::super::pb::field::{CreateFieldRequest, Field, GetFieldRequest, UpdateFieldRequest, DeleteFieldRequest};
use super::super::pb::field::field_service_server::FieldService;

mod create;
mod get;
mod update;
mod delete;

pub struct Service {
    repository: super::repository::Repository,
}

impl Service {
    pub fn new(pool: PgPool) -> Service {
        Service {
            repository: super::repository::Repository::new(pool)
        }
    }
}

#[tonic::async_trait]
impl FieldService for Service {
    async fn create_field(
        &self,
        request: Request<CreateFieldRequest>,
    ) -> Result<Response<Field>, Status> {
        self.on_create_field(request).await
    }

    async fn get_field(
        &self,
        request: Request<GetFieldRequest>,
    ) -> Result<Response<Field>, Status> {
        self.on_get_field(request).await
    }

    async fn update_field(
        &self,
        request: Request<UpdateFieldRequest>,
    ) -> Result<Response<Field>, Status> {
        self.on_update_field(request).await
    }

    async fn delete_field(
        &self,
        request: Request<DeleteFieldRequest>,
    ) -> Result<Response<()>, Status> {
        self.on_delete_field(request).await
    }
}
