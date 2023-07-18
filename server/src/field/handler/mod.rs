use sqlx::PgPool;
use tonic::{Request, Response, Status};

use super::super::pb::field::{CreateFieldRequest, Field, GetFieldRequest, UpdateFieldRequest, DeleteFieldRequest};
use super::super::pb::field::field_service_server::FieldService;

pub struct Handler {
    service: super::service::Service,
}

impl Handler {
    pub fn new(pool: PgPool) -> Handler {
        Handler {
            service: super::service::Service::new(pool.clone()),
        }
    }
}

#[tonic::async_trait]
impl FieldService for Handler {
    async fn create_field(
        &self,
        request: Request<CreateFieldRequest>,
    ) -> Result<Response<Field>, Status> {
        self.service.create(request.get_ref().field.to_owned().expect("Field must be defined")).await
    }

    async fn get_field(
        &self,
        request: Request<GetFieldRequest>,
    ) -> Result<Response<Field>, Status> {
        self.service.get(request.get_ref().id).await
    }

    async fn update_field(
        &self,
        request: Request<UpdateFieldRequest>,
    ) -> Result<Response<Field>, Status> {
        self.service.update(request.get_ref().field.to_owned().expect("Field must be defined")).await
    }

    async fn delete_field(
        &self,
        request: Request<DeleteFieldRequest>,
    ) -> Result<Response<()>, Status> {
        self.service.delete(request.get_ref().id).await
    }
}
