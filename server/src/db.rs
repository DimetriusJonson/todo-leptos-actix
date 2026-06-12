use app::common::DbPool;

#[cfg(feature = "sqlx-postgres")]
pub async fn create_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("no database url specify");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .min_connections(1)
        .max_connections(3)
        .connect(database_url.as_str())
        .await
        .expect("could not connect to database_url");

    sqlx::migrate!("migrations/postgres")
        .run(&pool)
        .await
        .expect("migrations failed");

    Ok(pool)
}
 
#[cfg(feature = "sqlx-sqlite")]
pub async fn create_pool() -> Result<DbPool, sqlx::Error> {
    use log::info;
    use sqlx::migrate::MigrateDatabase;

    let database_url = std::env::var("DATABASE_URL").expect("no database url specify");
    info!("database_url={}", database_url);
    if !sqlx::Sqlite::database_exists(&database_url).await.unwrap_or(false) {
        info!("Creating database {}", database_url);
        match sqlx::Sqlite::create_database(&database_url).await {
            Ok(_) => info!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        info!("Database already exists");
    }

    let db = sqlx::SqlitePool::connect(&database_url).await?;

    info!("db successfully initialized!");

    sqlx::migrate!("migrations/sqlite").run(&db).await.expect("migrations failed");

    Ok(db)
}
