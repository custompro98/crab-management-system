use sqlx::PgPool;
use tonic::{Request, Response, Status};

use super::super::pb::field_group::{CreateFieldGroupRequest, FieldGroup, GetFieldGroupRequest, UpdateFieldGroupRequest, DeleteFieldGroupRequest};
use super::super::pb::field_group::field_group_service_server::FieldGroupService;

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
impl FieldGroupService for Handler {
    async fn create_field_group(
        &self,
        request: Request<CreateFieldGroupRequest>,
    ) -> Result<Response<FieldGroup>, Status> {
        self.service.create(request.get_ref().field_group.to_owned().expect("FieldGroup must be defined")).await
    }

    async fn get_field_group(
        &self,
        request: Request<GetFieldGroupRequest>,
    ) -> Result<Response<FieldGroup>, Status> {
        self.service.get(request.get_ref().id).await
    }

    async fn update_field_group(
        &self,
        request: Request<UpdateFieldGroupRequest>,
    ) -> Result<Response<FieldGroup>, Status> {
        self.service.update(request.get_ref().field_group.to_owned().expect("FieldGroup must be defined")).await
    }

    async fn delete_field_group(
        &self,
        request: Request<DeleteFieldGroupRequest>,
    ) -> Result<Response<()>, Status> {
        self.service.delete(request.get_ref().id).await
    }
}
