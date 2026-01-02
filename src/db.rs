use std::time::Duration;
use sqlx::{Pool, Sqlite};

#[cfg(feature = "postgres")]
use sqlx::Postgres;

/// Universal Database Configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub acquire_timeout: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite::memory:".to_string(),
            max_connections: 5,
            acquire_timeout: Duration::from_secs(30),
        }
    }
}

/// A wrapper around various DB pools.
/// The Template uses this type so it doesn't care if it's Sqlite or Postgres.
#[derive(Clone, Debug)]
pub enum DatabasePool {
    Sqlite(Pool<Sqlite>),
    #[cfg(feature = "postgres")]
    Postgres(Pool<Postgres>),
}

/// The single entry point for connecting to any DB.
pub async fn connect(config: &DatabaseConfig) -> Result<DatabasePool, sqlx::Error> {
    // 1. Try Postgres if the feature is enabled AND the URL looks like postgres
    #[cfg(feature = "postgres")]
    if config.url.starts_with("postgres://") || config.url.starts_with("postgresql://") {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(config.max_connections)
            .acquire_timeout(config.acquire_timeout)
            .connect(&config.url)
            .await?;
        return Ok(DatabasePool::Postgres(pool));
    }

    // 2. Default to Sqlite
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(config.acquire_timeout)
        .connect(&config.url)
        .await?;
    
    Ok(DatabasePool::Sqlite(pool))
}

// Re-export specific connectors if you still need manual control
pub async fn connect_sqlite(url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
   sqlx::sqlite::SqlitePoolOptions::new().connect(url).await
}