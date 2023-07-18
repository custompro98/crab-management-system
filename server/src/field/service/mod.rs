use sqlx::PgPool;

mod create;
mod get;
mod update;
mod delete;

pub struct Service {
    repository: super::repository::Repository,
    account_service: super::super::account::service::Service,
}

impl Service {
    pub fn new(pool: PgPool) -> Service {
        Service {
            repository: super::repository::Repository::new(pool.clone()),
            account_service: super::super::account::service::Service::new(pool.clone())
        }
    }
}
