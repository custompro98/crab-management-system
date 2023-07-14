use tonic::{Request, Response, Status};

use super::super::pb::DeleteUserRequest;
use super::Service;

impl Service {
    pub async fn on_delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<()>, Status> {
        let success = self.repository.delete(request.get_ref().id).await;

        match success {
            Ok(_) => Ok(Response::new(())),
            Err(_) => Err(Status::internal("An internal error occurred")),
        }
    }
}
