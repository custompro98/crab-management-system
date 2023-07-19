use std::collections::HashMap;

use tonic::{Response, Status, Code};

use super::super::super::pb::user::User;

use super::super::super::pb::user::batch_get_users_response::maybe_user::OptionalUser;

use super::super::super::pb::account::batch_get_accounts_response::maybe_account::OptionalAccount;
use super::super::super::pb::account::{batch_get_accounts_response::MaybeAccount, Account, BatchGetAccountsResponse};

use super::Service;

impl Service {
    pub async fn batch_get(&self, account_ids: &Vec<i32>) -> Result<Response<BatchGetAccountsResponse>, Status> {
        let accounts = self.repository.batch_get(account_ids).await;

        if let Err(status) = accounts {
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

        let mut accounts_by_id: HashMap<i32, Account> = HashMap::new();
        for account in accounts.to_owned().unwrap() {
            accounts_by_id.insert(account.id, account);
        }

        let users = self.user_service
            .batch_get(
                &accounts
                    .unwrap()
                    .iter()
                    .map(|acc| acc.owner_id )
                    .collect()
            ).await?;

        let mut users_by_id: HashMap<i32, User> = HashMap::new();
        for user in &users.get_ref().users {
            if let Some(OptionalUser::User(user)) = &user.optional_user {
                users_by_id.insert(user.id, user.to_owned());
            }
        }

        let mut maybe_accounts: Vec<MaybeAccount> = Vec::new();
        for account_id in account_ids {
            match accounts_by_id.get_mut(&account_id) {
                Some(account) => {
                    let user = users_by_id.get(&account.owner_id);

                    if let Some(user) = user {
                        account.owner = Some(user.to_owned());
                    }

                    maybe_accounts.push(MaybeAccount {
                        optional_account: Some(OptionalAccount::Account(account.to_owned()))
                    })
                },
                None => maybe_accounts.push(MaybeAccount { optional_account: None }),
            }
        }

        Ok(Response::new(BatchGetAccountsResponse {
            accounts: maybe_accounts,
        }))
    }
}
