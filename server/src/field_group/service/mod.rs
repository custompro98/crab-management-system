use sqlx::PgPool;

mod create;
mod delete;
mod get;
mod update;

pub struct Service {
    repository: super::repository::Repository,
    account_service: super::super::account::service::Service,
    field_service: super::super::field::service::Service,
}

impl Service {
    pub fn new(pool: PgPool) -> Service {
        Service {
            repository: super::repository::Repository::new(pool.clone()),
            account_service: super::super::account::service::Service::new(pool.clone()),
            field_service: super::super::field::service::Service::new(pool.clone()),
        }
    }
}
