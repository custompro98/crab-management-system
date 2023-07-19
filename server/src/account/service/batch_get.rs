use std::collections::HashMap;

use tonic::{Response, Status, Code};

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
        for account in accounts.unwrap() {
            accounts_by_id.insert(account.id, account);
        }

        let mut maybe_accounts: Vec<MaybeAccount> = Vec::new();
        for account_id in account_ids {
            match accounts_by_id.get(&account_id) {
                Some(account) => maybe_accounts.push(MaybeAccount {
                    optional_account: Some(OptionalAccount::Account(account.to_owned()))
                }),
                None => maybe_accounts.push(MaybeAccount { optional_account: None }),
            }
        }

        /* let user = self.user_service.get(user.user_id).await?;
        account.user = Some(user.get_ref().to_owned()); */

        Ok(Response::new(BatchGetAccountsResponse {
            accounts: maybe_accounts,
        }))
    }
}
