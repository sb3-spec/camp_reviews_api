use sqlx::PgPool;
use std::{env, path};

pub async fn connect_to_db() -> Result<PgPool, Error> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    match sqlx::migrate!().run(&pool).await {
        Ok(_) => println!("Migrations successfully applied"),
        Err(e) => println!("Error applying migrations: {:?}", e),
    };

    Ok(pool)
}

use thiserror::Error as ThisError;

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
