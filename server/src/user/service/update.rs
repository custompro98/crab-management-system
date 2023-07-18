use tonic::{Response, Status, Code};

use super::super::super::pb::user::User;
use super::Service;

impl Service {
    pub async fn update(
        &self,
        user: User,
    ) -> Result<Response<User>, Status> {
        let user = self.repository.update(user).await;

        match user {
            Ok(user) => Ok(Response::new(user)),
            Err(status) => match &status.code() {
                Code::NotFound => Err(status),
                Code::InvalidArgument => Err(status),
                Code::AlreadyExists => Err(status),
                Code::FailedPrecondition => Err(status),
                Code::PermissionDenied => Err(status),
                Code::Unauthenticated => Err(status),
                _ => Err(Status::internal("An internal error occurred")),
            },
        }
    }
}
