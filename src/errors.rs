use thiserror::Error;

// #[allow(dead_code)]
#[derive(Debug, Error)]
pub enum LibError {
    #[error("actix_web error: {0}")]
    ActixError(String),
    #[error("database error: {0}")]
    DbError(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("chrono error: {0}")]
    Chrono(String),
    #[error("diesel error: {0}")]
    Diesel(#[from] diesel::result::Error),
}
