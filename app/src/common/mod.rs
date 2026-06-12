pub mod api_error;
pub mod app_state;
pub mod validate_helper;

#[cfg(feature = "ssr")]
//pub type DbPool = sqlx::Pool<sqlx::Sqlite>;
pub type DbPool = sqlx::Pool<sqlx::Postgres>;
