//! # Database Module
//!
//! This module provides database integration for the SurfDesk application.
//! It includes schema definitions, migrations, and database service
//! implementations using Diesel ORM with SQLite backend.

pub mod migrations;
pub mod schema;

// Re-export key database types for convenience
pub use migrations::SimpleMigration;
pub use schema::*;

use crate::error::{Result, SurfDeskError};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Database connection pool type
pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

/// Database connection type
pub type DbConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

/// Database models for common operations
#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
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

/// Database utility functions
pub fn establish_connection(database_url: &str) -> Result<SqliteConnection> {
    SqliteConnection::establish(database_url).map_err(|e| {
        SurfDeskError::database(format!("Failed to establish database connection: {}", e))
    })
}

/// Create a connection pool
pub fn create_pool(database_url: &str, max_size: u32) -> Result<DbPool> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .max_size(max_size)
        .build(manager)
        .map_err(|e| SurfDeskError::database(format!("Failed to create connection pool: {}", e)))
}

/// Run database migrations
pub fn run_migrations(conn: &mut SqliteConnection) -> Result<()> {
    SimpleMigration::initialize_database(conn)
        .map_err(|e| SurfDeskError::database(format!("Failed to run migrations: {}", e)))
}

/// Database initialization utilities
pub struct DatabaseInitializer;

impl DatabaseInitializer {
    /// Initialize database with all tables and migrations
    pub fn initialize(database_url: &str) -> Result<()> {
        let mut conn = establish_connection(database_url)?;

        // Enable foreign keys
        diesel::sql_query("PRAGMA foreign_keys = ON")
            .execute(&mut conn)
            .map_err(|e| {
                SurfDeskError::database(format!("Failed to enable foreign keys: {}", e))
            })?;

        // Enable WAL mode for better concurrency
        diesel::sql_query("PRAGMA journal_mode = WAL")
            .execute(&mut conn)
            .map_err(|e| SurfDeskError::database(format!("Failed to enable WAL mode: {}", e)))?;

        // Run migrations
        run_migrations(&mut conn)?;

        log::info!("Database initialized successfully");
        Ok(())
    }

    /// Get default database path for the current platform
    pub fn default_database_path() -> PathBuf {
        let mut path = dirs::data_dir()
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

    #[test]
    fn test_database_initialization() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_url = db_path.to_string_lossy();

        let result = DatabaseInitializer::initialize(&db_url);
        assert!(result.is_ok());

        // Verify connection works
        let conn = establish_connection(&db_url);
        assert!(conn.is_ok());
    }

    #[test]
    fn test_connection_pool() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test_pool.db");
        let db_url = db_path.to_string_lossy();

        DatabaseInitializer::initialize(&db_url).unwrap();

        let pool = create_pool(&db_url, 5);
        assert!(pool.is_ok());
    }
}
