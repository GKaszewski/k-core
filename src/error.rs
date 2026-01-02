use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[cfg(feature = "db-sqlx")]
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found")]
    NotFound,

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal server error")]
    Internal,
}
