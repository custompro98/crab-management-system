use sqlx::PgPool;

mod create;
mod get;
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
