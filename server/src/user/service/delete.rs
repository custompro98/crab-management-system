use tonic::{Request, Response, Status};

use super::super::pb::DeleteUserRequest;
use super::Service;

impl Service {
    pub fn on_delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<()>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        Ok(Response::new(()))
    }
}
