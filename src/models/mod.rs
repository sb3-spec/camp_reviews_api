use thiserror::Error as ThisError;

mod camp;
mod db;
mod review;
mod user;

pub use camp::{Camp, CampManager, CampPatch};
pub use db::connect_to_db;
pub use review::{Review, ReviewManager, ReviewPatch};
pub use user::{User, UserManager};

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
