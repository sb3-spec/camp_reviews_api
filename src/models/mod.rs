use thiserror::Error as ThisError;

mod db;
mod review;

pub use db::connect_to_db;
pub use review::{Review, ReviewManager, ReviewPatch};

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Failed to connect to database")]
    DatabaseConnectionFailed(#[from] sqlx::Error),

    #[error("Failed to read environment variable")]
    EnvironmentVariableReadFailure(#[from] std::env::VarError),

    #[error("Failed to read sql files")]
    SqlFileReadFailure(#[from] std::io::Error),

    #[error("Failed to apply migrations")]
    MigrationFailed(#[from] sqlx::migrate::MigrateError),
}
