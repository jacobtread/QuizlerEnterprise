use anyhow::Context;
use sea_orm::{Database, DatabaseConnection, DbErr};
use tracing::debug;

pub mod entities;

pub type DbResult<T> = Result<T, DbErr>;

pub async fn connect() -> anyhow::Result<DatabaseConnection> {
    let database_url: String = std::env::var("DATABASE_URL").context("Missing database URL")?;

    debug!("Connecting to database: {database_url}");

    let db: DatabaseConnection = Database::connect(database_url).await?;

    Ok(db)
}
