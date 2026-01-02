#[cfg(feature = "db-sqlx")]
use sqlx::Pool;
use std::time::Duration;

#[cfg(feature = "sqlite")]
use sqlx::Sqlite;

#[cfg(feature = "postgres")]
use sqlx::Postgres;

/// Universal Database Configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        #[cfg(feature = "sqlite")]
        {
            Self {
                url: "sqlite::memory:".to_string(),
                max_connections: 5,
                min_connections: 1,
                acquire_timeout: Duration::from_secs(30),
            }
        }

        #[cfg(all(not(feature = "sqlite"), feature = "postgres"))]
        {
            Self {
                url: "postgres://localhost:5432/mydb".to_string(),
                max_connections: 5,
                min_connections: 1,
                acquire_timeout: Duration::from_secs(30),
            }
        }

        #[cfg(not(any(feature = "sqlite", feature = "postgres")))]
        Self {
            url: "".to_string(),
            max_connections: 5,
            min_connections: 1,
            acquire_timeout: Duration::from_secs(30),
        }
    }
}

impl DatabaseConfig {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            ..Default::default()
        }
    }

    #[cfg(feature = "sqlite")]
    pub fn in_memory() -> Self {
        Self {
            url: "sqlite::memory:".to_string(),
            max_connections: 1, // SQLite in-memory is single-connection
            min_connections: 1,
            ..Default::default()
        }
    }
}

/// A wrapper around various DB pools.
/// The Template uses this type so it doesn't care if it's Sqlite or Postgres.
#[derive(Clone, Debug)]
pub enum DatabasePool {
    #[cfg(feature = "sqlite")]
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

    // 2. Fallback to Sqlite if the feature is enabled
    #[cfg(feature = "sqlite")]
    {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(config.max_connections)
            .acquire_timeout(config.acquire_timeout)
            .connect(&config.url)
            .await?;

        Ok(DatabasePool::Sqlite(pool))
    }

    #[cfg(not(feature = "sqlite"))]
    {
        Err(sqlx::Error::Configuration(
            "No supported database features enabled".into(),
        ))
    }
}

impl DatabasePool {
    #[cfg(feature = "sqlite")]
    pub fn sqlite_pool(&self) -> Option<&Pool<Sqlite>> {
        if let DatabasePool::Sqlite(pool) = self {
            Some(pool)
        } else {
            None
        }
    }

    #[cfg(feature = "postgres")]
    pub fn postgres_pool(&self) -> Option<&Pool<Postgres>> {
        if let DatabasePool::Postgres(pool) = self {
            Some(pool)
        } else {
            None
        }
    }
}
#[cfg(feature = "sqlite")]
pub async fn connect_sqlite(url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    sqlx::sqlite::SqlitePoolOptions::new().connect(url).await
}
