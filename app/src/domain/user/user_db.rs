#[cfg(feature = "ssr")]
pub mod db {
    use sqlx::query_as;

    use crate::{domain::user::model::user::User, common::DbPool};

    pub async fn get_user_from_db(pool: &DbPool, id: i64) -> Result<Option<User>, sqlx::Error> {
        query_as!(
            User,
            r#"
                SELECT
                    id,
                    username,
                    password,
                    token
                FROM users
                    WHERE id = $1 and deleted_at is null
            "#,
            id,
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn get_user_by_name_from_db(
        pool: &DbPool,
        name: Option<String>,
    ) -> Result<Option<User>, sqlx::Error> {
        query_as!(
            User,
            r#"
                SELECT
                    id,
                    username,
                    password,
                    token
                FROM users
                    WHERE username = $1 and deleted_at is null
            "#,
            name,
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create_user_in_db(pool: &DbPool, user: &User) -> Result<User, sqlx::Error> {
        query_as!(
            User,
            r#"
                INSERT INTO users (username, password)
                VALUES ($1, $2)
                RETURNING id, username, password, token
            "#,
            user.username,
            user.password
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update_user_in_db(pool: &DbPool, user: &User) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                UPDATE users 
                SET
                    token = $1
                WHERE id = $2 and deleted_at is null
            "#,
            user.token,
            user.id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
