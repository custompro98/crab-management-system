use super::Repository;

impl Repository {
    pub async fn on_delete_user(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
              UPDATE users
              SET deleted_at = now()
              WHERE id = $1
            "#,
            id
        ).fetch_one(&self.pool).await?;

        Result::Ok(())
    }
}
