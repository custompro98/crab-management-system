use tonic::{Request, Response, Status};

use super::Service;
use super::pb::DeleteUserRequest;

impl Service {
    pub fn on_delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<()>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        Ok(Response::new(()))
    }
}
