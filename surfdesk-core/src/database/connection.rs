// Re-export key database types for convenience
// pub use schema::*; // Removed - no longer using diesel

use crate::{
    database::migrations::SimpleMigration,
    error::{Result, SurfDeskError},
};
use libsql::Connection;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Database connection type (libsql doesn't use pooling in the same way)
pub type DbConnection = Connection;

/// Database models for common operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub config: Option<serde_json::Value>,
    pub status: String,
    pub owner: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub type_: String,
    pub config: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub rpc_url: String,
    pub ws_url: Option<String>,
    pub is_default: bool,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub environment_id: String,
    pub pubkey: String,
    pub account_data: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub label: Option<String>,
    pub account_type: String,
    pub is_watched: bool,
    pub balance: Option<i64>,
    pub owner: Option<String>,
    pub executable: bool,
    pub rent_epoch: Option<i64>,
}

/// Database utility functions for libsql
pub async fn establish_connection(database_url: &str) -> Result<Connection> {
    let db = libsql::Builder::new_local(database_url)
        .build()
        .await
        .map_err(|e| {
            SurfDeskError::database(format!("Failed to establish database connection: {}", e))
        })?;
    db.connect()
        .map_err(|e| SurfDeskError::database(e.to_string()))
}

/// Run database migrations
pub async fn run_migrations(conn: &mut Connection) -> Result<()> {
    SimpleMigration::initialize_database(conn)
        .await
        .map_err(|e| SurfDeskError::database(format!("Failed to run migrations: {}", e)))
}

/// Database initialization utilities
pub struct DatabaseInitializer;

impl DatabaseInitializer {
    /// Initialize database with all tables and migrations
    pub async fn initialize(database_url: &str) -> Result<()> {
        let mut conn = establish_connection(database_url).await?;

        conn.execute("PRAGMA foreign_keys = ON", ())
            .await
            .map_err(|e| {
                SurfDeskError::database(format!("Failed to enable foreign keys: {}", e))
            })?;

        conn.execute("PRAGMA journal_mode = WAL", ())
            .await
            .map_err(|e| SurfDeskError::database(format!("Failed to enable WAL mode: {}", e)))?;

        run_migrations(&mut conn).await?;

        Ok(())
    }

    /// Get default database path for the current platform
    pub fn default_database_path() -> PathBuf {
        let path = dirs::data_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .join("surfdesk");

        std::fs::create_dir_all(&path).ok();
        path.join("surfdesk.db")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_database_initialization() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_url = db_path.to_string_lossy();

        let result = DatabaseInitializer::initialize(&db_url).await;
        assert!(result.is_ok());

        // Verify connection works
        let conn = establish_connection(&db_url).await;
        assert!(conn.is_ok());
    }

    #[tokio::test]
    async fn test_connection_pool() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test_pool.db");
        let db_url = db_path.to_string_lossy();

        DatabaseInitializer::initialize(&db_url).await.unwrap();

        // For libsql, we just test establishing a connection
        let conn = establish_connection(&db_url).await;
        assert!(conn.is_ok());
    }
}
