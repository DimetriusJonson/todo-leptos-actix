pub mod api_error;
pub mod errors;
pub mod security_context;
pub mod validate_helper;

#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};

#[cfg(feature = "ssr")]
pub type DbPool = Pool<Sqlite>;
