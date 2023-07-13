use std::time::SystemTime;

use chrono::{DateTime, Utc};
use tonic::{Request, Response, Status};

use crate::user::pb::user::OptionalName;

use super::super::pb::{GetUserRequest, User};
use super::Service;

impl Service {
    pub fn on_get_user(&self, request: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let now = now.to_rfc3339();

        let reply = User {
            id: 1,
            email: "mitchjoa@gmail.com".to_owned(),
            username: "custompro98".to_owned(),
            created_at: now,
            optional_name: Some(OptionalName::Name("Mitch".to_owned())),
            optional_updated_at: None,
            optional_deleted_at: None,
        };

        Ok(Response::new(reply))
    }
}
