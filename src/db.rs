use sqlx::{Pool, Sqlite};

#[cfg(feature = "db-sqlx")]
pub async fn connect_sqlite(url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
}

// Future expansion for Postgres
#[cfg(feature = "postgres")]
use sqlx::Postgres;

#[cfg(feature = "postgres")]
pub async fn connect_postgres(url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
}
