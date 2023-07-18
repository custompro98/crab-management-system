use sqlx::PgPool;

mod create;
mod get;
mod update;
mod delete;

pub struct Service {
    repository: super::repository::Repository,
    user_service: super::super::user::service::Service,
}

impl Service {
    pub fn new(pool: PgPool) -> Service {
        Service {
            repository: super::repository::Repository::new(pool.clone()),
            user_service: super::super::user::service::Service::new(pool.clone())
        }
    }
}
