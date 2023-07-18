use sqlx::PgPool;

mod create;
mod get;
mod update;
mod delete;

pub struct Service {
    repository: super::repository::Repository,
}

impl Service {
    pub fn new(pool: PgPool) -> Service {
        Service {
            repository: super::repository::Repository::new(pool),
        }
    }
}
