pub mod api_error;
pub mod app_state;
pub mod validate_helper;

#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};

#[cfg(feature = "ssr")]
pub type DbPool = Pool<Sqlite>;
