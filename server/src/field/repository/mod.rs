use sqlx::PgPool;

mod create;
mod list;
mod get;
mod batch_get;
mod update;
mod delete;

pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Repository {
        Repository { pool }
    }
}
