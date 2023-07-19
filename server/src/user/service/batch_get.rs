use std::collections::HashMap;

use tonic::{Response, Status, Code};

use super::super::super::pb::user::batch_get_users_response::maybe_user::OptionalUser;
use super::super::super::pb::user::{batch_get_users_response::MaybeUser, User, BatchGetUsersResponse};

use super::Service;

impl Service {
    pub async fn batch_get(&self, user_ids: &Vec<i32>) -> Result<Response<BatchGetUsersResponse>, Status> {
        let users = self.repository.batch_get(user_ids).await;

        if let Err(status) = users {
            return match &status.code() {
                Code::NotFound => Err(status),
                Code::InvalidArgument => Err(status),
                Code::AlreadyExists => Err(status),
                Code::FailedPrecondition => Err(status),
                Code::PermissionDenied => Err(status),
                Code::Unauthenticated => Err(status),
                _ => Err(Status::internal("An internal error occurred"))
            };
        }

        let mut users_by_id: HashMap<i32, User> = HashMap::new();
        for user in users.unwrap() {
            users_by_id.insert(user.id, user);
        }

        let mut maybe_users: Vec<MaybeUser> = Vec::new();
        for user_id in user_ids {
            match users_by_id.get(&user_id) {
                Some(user) => maybe_users.push(MaybeUser {
                    optional_user: Some(OptionalUser::User(user.to_owned()))
                }),
                None => maybe_users.push(MaybeUser { optional_user: None }),
            }
        }

        Ok(Response::new(BatchGetUsersResponse {
            users: maybe_users,
        }))
    }
}
