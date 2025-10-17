#![allow(dead_code)]
//! # Database Service Module
//!
//! This module provides database integration for the SurfDesk application.
//! It handles data persistence, migrations, and query operations across
//! all platforms with platform-specific storage backends using Turso (libSQL).

use crate::error::{Result, SurfDeskError};
use libsql::{Connection, Database};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Result structure for count queries
#[derive(Debug)]
pub struct CountResult {
    pub count: i64,
}

/// Database service for data persistence
pub struct DatabaseService {
    /// Database connection
    db: Arc<Database>,
    /// Database configuration
    config: DatabaseConfig,
    /// Migration status
    migrations_run: Arc<RwLock<bool>>,
}

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Database file path or URL
    pub database_path: String,
    /// Connection pool size
    pub pool_size: u32,
    /// Connection timeout in seconds
    pub timeout: u64,
    /// Whether to enable foreign key constraints
    pub foreign_keys: bool,
    /// Whether to enable WAL mode
    pub wal_mode: bool,
    /// Database backend type
    pub backend: DatabaseBackend,
    /// Turso-specific configuration
    pub turso_config: Option<TursoConfig>,
}

/// Database backend types
#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseBackend {
    /// Local SQLite database
    SQLite,
    /// Turso (libSQL) cloud database
    Turso,
}

/// Turso database configuration
#[derive(Debug, Clone)]
pub struct TursoConfig {
    /// Database URL
    pub url: String,
    /// Authentication token
    pub auth_token: String,
    /// Remote database name
    pub database_name: String,
}

impl DatabaseService {
    /// Create a new database service
    pub async fn new(_config_service: &crate::services::config::ConfigService) -> Result<Self> {
        let _config = DatabaseConfig::from_env();
        Self::with_config(_config).await
    }

    /// Create a new database service with custom configuration
    pub async fn with_config(config: DatabaseConfig) -> Result<Self> {
        log::info!(
            "Initializing database service with backend: {:?}",
            config.backend
        );
        if config.is_turso() {
            if let Some(turso_config) = &config.turso_config {
                log::info!("Turso database: {}", turso_config.database_name);
            }
        } else {
            log::info!("SQLite database path: {}", config.database_path);
        }

        // Create database directory if it doesn't exist
        if let Some(parent) = std::path::Path::new(&config.database_path).parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                SurfDeskError::database(format!("Failed to create database directory: {}", e))
            })?;
        }

        log::info!(
            "Initializing database service with path: {}",
            config.database_path
        );

        // Create database directory if needed
        if let Some(parent) = std::path::Path::new(&config.database_path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Create database connection based on backend
        let db = match config.backend {
            DatabaseBackend::SQLite => {
                let db = libsql::Builder::new_local(&config.database_path)
                    .build()
                    .await
                    .map_err(|e| SurfDeskError::database(e.to_string()))?;
                db
            }
            DatabaseBackend::Turso => {
                if let Some(turso_config) = &config.turso_config {
                    log::info!(
                        "Connecting to Turso database: {}",
                        turso_config.database_name
                    );
                    let db = libsql::Builder::new_remote(
                        turso_config.url.clone(),
                        turso_config.auth_token.clone(),
                    )
                    .build()
                    .await
                    .map_err(|e| {
                        SurfDeskError::database(format!("Failed to connect to Turso: {}", e))
                    })?;
                    db
                } else {
                    return Err(SurfDeskError::database("Turso configuration missing"));
                }
            }
        };

        let service = Self {
            db: Arc::new(db),
            config,
            migrations_run: Arc::new(RwLock::new(false)),
        };

        // Run migrations
        service.run_migrations().await?;

        // Configure database
        let mut conn = service
            .db
            .connect()
            .map_err(|e| SurfDeskError::database(e.to_string()))?;
        service.configure_database(&mut conn).await?;

        log::info!("Database service initialized successfully");
        Ok(service)
    }

    /// Run database migrations
    async fn run_migrations(&self) -> Result<()> {
        let mut migrations_run = self.migrations_run.write().await;
        if *migrations_run {
            return Ok(());
        }

        let mut conn = self
            .db
            .connect()
            .map_err(|e| SurfDeskError::database(e.to_string()))?;

        // Run embedded migrations
        self.run_embedded_migrations(&mut conn).await?;

        // Configure database
        self.configure_database(&mut conn).await?;

        *migrations_run = true;
        log::info!("Database migrations completed");
        Ok(())
    }

    /// Run embedded migrations
    async fn run_embedded_migrations(&self, conn: &mut Connection) -> Result<()> {
        // Use raw SQL for table creation - simpler approach
        let migrations = vec![
            "CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                owner TEXT NOT NULL,
                settings TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            "CREATE TABLE IF NOT EXISTS environments (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                name TEXT NOT NULL,
                type TEXT NOT NULL,
                network TEXT NOT NULL,
                rpc_url TEXT,
                config TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            "CREATE TABLE IF NOT EXISTS accounts (
                id TEXT PRIMARY KEY,
                environment_id TEXT NOT NULL,
                pubkey TEXT NOT NULL,
                owner TEXT NOT NULL,
                balance TEXT,
                data TEXT,
                executable INTEGER NOT NULL DEFAULT 0,
                rent_epoch INTEGER,
                updated_at TEXT NOT NULL
            )",
            "CREATE TABLE IF NOT EXISTS transactions (
                id TEXT PRIMARY KEY,
                environment_id TEXT NOT NULL,
                signature TEXT NOT NULL,
                status TEXT NOT NULL,
                details TEXT,
                simulation TEXT,
                created_at TEXT NOT NULL,
                confirmed_at TEXT
            )",
            "CREATE INDEX IF NOT EXISTS idx_projects_owner ON projects (owner)",
            "CREATE INDEX IF NOT EXISTS idx_environments_project_id ON environments (project_id)",
            "CREATE INDEX IF NOT EXISTS idx_accounts_environment_id ON accounts (environment_id)",
            "CREATE INDEX IF NOT EXISTS idx_transactions_environment_id ON transactions (environment_id)",
        ];

        for migration in migrations {
            conn.execute(migration, ())
                .await
                .map_err(|e| SurfDeskError::database(e.to_string()))?;
        }

        Ok(())
    }

    /// Configure database settings
    async fn configure_database(&self, conn: &mut Connection) -> Result<()> {
        // Enable foreign key constraints
        if self.config.foreign_keys {
            conn.execute("PRAGMA foreign_keys = ON", ())
                .await
                .map_err(|e| SurfDeskError::database(e.to_string()))?;
        }

        // Enable WAL mode
        if self.config.wal_mode && matches!(self.config.backend, DatabaseBackend::SQLite) {
            conn.execute("PRAGMA journal_mode = WAL", ())
                .await
                .map_err(|e| SurfDeskError::database(e.to_string()))?;
        }

        // Set connection timeout
        let timeout_sql = format!("PRAGMA busy_timeout = {}", self.config.timeout * 1000);
        conn.execute(&timeout_sql, ())
            .await
            .map_err(|e| SurfDeskError::database(e.to_string()))?;

        // Optimize for performance
        conn.execute("PRAGMA synchronous = NORMAL", ())
            .await
            .map_err(|e| SurfDeskError::database(e.to_string()))?;

        conn.execute("PRAGMA cache_size = 10000", ())
            .await
            .map_err(|e| SurfDeskError::database(e.to_string()))?;

        Ok(())
    }

    /// Get a database connection
    pub async fn get_connection(&self) -> Result<Connection> {
        self.db
            .connect()
            .map_err(|e| SurfDeskError::database(e.to_string()))
    }

    /// Get database statistics
    pub async fn get_stats(&self) -> Result<DatabaseStats> {
        let conn = self.get_connection().await?;

        // Get table counts using libsql
        let mut rows = conn
            .query("SELECT COUNT(*) FROM projects", ())
            .await
            .map_err(|e| SurfDeskError::database(e.to_string()))?;
        let project_count = rows
            .next()
            .await?
            .map(|row| row.get(0).unwrap_or(0i64))
            .unwrap_or(0);

        let mut rows = conn
            .query("SELECT COUNT(*) FROM environments", ())
            .await
            .map_err(|e| SurfDeskError::database(e.to_string()))?;
        let environment_count = rows
            .next()
            .await?
            .map(|row| row.get(0).unwrap_or(0i64))
            .unwrap_or(0);

        let mut rows = conn
            .query("SELECT COUNT(*) FROM accounts", ())
            .await
            .map_err(|e| SurfDeskError::database(e.to_string()))?;
        let account_count = rows
            .next()
            .await?
            .map(|row| row.get(0).unwrap_or(0i64))
            .unwrap_or(0);

        let mut rows = conn
            .query("SELECT COUNT(*) FROM transactions", ())
            .await
            .map_err(|e| SurfDeskError::database(e.to_string()))?;
        let transaction_count = rows
            .next()
            .await?
            .map(|row| row.get(0).unwrap_or(0i64))
            .unwrap_or(0);

        // Get database file size (only for SQLite)
        let file_size = if matches!(self.config.backend, DatabaseBackend::SQLite) {
            std::fs::metadata(&self.config.database_path)
                .map(|m| m.len())
                .unwrap_or(0)
        } else {
            0 // Turso is remote, no local file size
        };

        Ok(DatabaseStats {
            projects: project_count as usize,
            environments: environment_count as usize,
            accounts: account_count as usize,
            transactions: transaction_count as usize,
            file_size_bytes: file_size,
            pool_connections: 1, // libsql uses single connection
            pool_idle_connections: 0,
        })
    }

    /// Backup database
    pub async fn backup(&self, backup_path: &str) -> Result<()> {
        if matches!(self.config.backend, DatabaseBackend::Turso) {
            return Err(SurfDeskError::database(
                "Backup not supported for Turso databases".to_string(),
            ));
        }

        let conn = self.get_connection().await?;

        // Use SQLite backup API
        let backup_path = std::path::Path::new(backup_path);
        if let Some(parent) = backup_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        conn.execute(&format!("VACUUM INTO '{}'", backup_path.display()), ())
            .await
            .map_err(|e| SurfDeskError::database(e.to_string()))?;

        log::info!("Database backed up to: {}", backup_path.display());
        Ok(())
    }

    /// Restore database from backup
    pub async fn restore(&self, backup_path: &str) -> Result<()> {
        if matches!(self.config.backend, DatabaseBackend::Turso) {
            return Err(SurfDeskError::database(
                "Restore not supported for Turso databases".to_string(),
            ));
        }

        let backup_path = std::path::Path::new(backup_path);

        if !backup_path.exists() {
            return Err(SurfDeskError::not_found(format!(
                "Backup file not found: {}",
                backup_path.display()
            )));
        }

        std::fs::copy(backup_path, &self.config.database_path)?;

        log::info!("Database restored from: {}", backup_path.display());
        Ok(())
    }

    /// Vacuum database to reclaim space
    pub async fn vacuum(&self) -> Result<()> {
        if matches!(self.config.backend, DatabaseBackend::Turso) {
            return Err(SurfDeskError::database(
                "Vacuum not supported for Turso databases".to_string(),
            ));
        }

        let conn = self.get_connection().await?;
        conn.execute("VACUUM", ())
            .await
            .map_err(|e| SurfDeskError::database(e.to_string()))?;

        log::info!("Database vacuumed");
        Ok(())
    }

    /// Analyze database for query optimization
    pub async fn analyze(&self) -> Result<()> {
        let conn = self.get_connection().await?;
        conn.execute("ANALYZE", ())
            .await
            .map_err(|e| SurfDeskError::database(e.to_string()))?;

        log::info!("Database analyzed");
        Ok(())
    }

    /// Shutdown the database service
    pub async fn shutdown(&self) -> Result<()> {
        log::info!("Shutting down database service");

        // Close all connections
        // Note: r2d2 Pool doesn't have a close() method in this version
        // The pool will be cleaned up when dropped
        log::info!("Database service shutdown");

        Ok(())
    }
}

/// Database statistics
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    /// Number of projects
    pub projects: usize,
    /// Number of environments
    pub environments: usize,
    /// Number of accounts
    pub accounts: usize,
    /// Number of transactions
    pub transactions: usize,
    /// Database file size in bytes
    pub file_size_bytes: u64,
    /// Number of connections in pool
    pub pool_connections: u32,
    /// Number of idle connections in pool
    pub pool_idle_connections: u32,
}

// CountResult is already defined above line 15

impl DatabaseConfig {
    /// Create database configuration for current platform
    pub fn from_platform() -> Self {
        let platform = crate::current_platform();

        let database_path = match platform {
            crate::platform::Platform::Desktop => {
                let mut path = PathBuf::from("./data/surfdesk");
                path.push("data");
                path.push("surfdesk.db");
                path.to_string_lossy().to_string()
            }
            crate::platform::Platform::Web => {
                // Web uses IndexedDB - return a placeholder
                "indexeddb://surfdesk".to_string()
            }
            crate::platform::Platform::Terminal => {
                let mut path = dirs::home_dir()
                    .unwrap_or_else(|| std::path::PathBuf::from("."))
                    .join(".surfdesk");
                path.push("data");
                path.push("surfdesk.db");
                path.to_string_lossy().to_string()
            }
        };

        Self {
            database_path,
            pool_size: 10,
            timeout: 30,
            foreign_keys: true,
            wal_mode: true,
            backend: DatabaseBackend::SQLite,
            turso_config: None,
        }
    }

    /// Create Turso database configuration
    pub fn from_turso(url: String, auth_token: String, database_name: String) -> Self {
        log::info!("Configuring Turso database: {}", database_name);
        Self {
            database_path: url.clone(),
            pool_size: 10,
            timeout: 30,
            foreign_keys: true,
            wal_mode: false, // WAL mode not available in Turso
            backend: DatabaseBackend::Turso,
            turso_config: Some(TursoConfig {
                url,
                auth_token,
                database_name,
            }),
        }
    }

    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        // Check for Turso environment variables
        if let (Ok(url), Ok(auth_token), Ok(database_name)) = (
            std::env::var("TURSO_URL"),
            std::env::var("TURSO_AUTH_TOKEN"),
            std::env::var("TURSO_DATABASE_NAME"),
        ) {
            Self::from_turso(url, auth_token, database_name)
        } else {
            log::info!("Using local SQLite database");
            Self::from_platform()
        }
    }

    /// Switch to Turso backend
    pub fn with_turso(mut self, url: String, auth_token: String, database_name: String) -> Self {
        self.backend = DatabaseBackend::Turso;
        self.database_path = url.clone();
        self.wal_mode = false;
        self.turso_config = Some(TursoConfig {
            url,
            auth_token,
            database_name,
        });
        self
    }

    /// Switch to SQLite backend
    pub fn with_sqlite(mut self, path: String) -> Self {
        self.backend = DatabaseBackend::SQLite;
        self.database_path = path;
        self.wal_mode = true;
        self.turso_config = None;
        self
    }

    /// Check if using Turso backend
    pub fn is_turso(&self) -> bool {
        matches!(self.backend, DatabaseBackend::Turso)
    }

    /// Check if using SQLite backend
    pub fn is_sqlite(&self) -> bool {
        matches!(self.backend, DatabaseBackend::SQLite)
    }
}

/// Turso migration helper
impl DatabaseService {
    /// Run migrations on Turso database
    pub async fn run_turso_migrations(&self) -> Result<()> {
        if !self.config.is_turso() {
            return Err(SurfDeskError::database("Not a Turso database backend"));
        }

        log::info!("Running migrations on Turso database");

        // For now, assume migrations are handled externally
        // TODO: Implement proper Turso migration system
        let mut migrations_run = self.migrations_run.write().await;
        *migrations_run = true;

        log::info!("Turso migrations marked as complete");
        Ok(())
    }

    /// Get backend information
    pub fn backend_info(&self) -> String {
        match self.config.backend {
            DatabaseBackend::SQLite => {
                format!("SQLite (path: {})", self.config.database_path)
            }
            DatabaseBackend::Turso => {
                if let Some(turso_config) = &self.config.turso_config {
                    format!(
                        "Turso (database: {}, url: {})",
                        turso_config.database_name, turso_config.url
                    )
                } else {
                    "Turso (configuration incomplete)".to_string()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_config() {
        let config = DatabaseConfig::from_platform();
        assert!(!config.database_path.is_empty());
        assert_eq!(config.pool_size, 10);
        assert!(config.foreign_keys);
        assert!(config.wal_mode);
    }

    #[tokio::test]
    async fn test_database_service_creation() {
        // Note: This test would need a config service
        // For now, we test the configuration logic
        let config = DatabaseConfig::from_platform();
        assert!(!config.database_path.is_empty());
    }

    #[test]
    fn test_database_stats() {
        let stats = DatabaseStats {
            projects: 5,
            environments: 10,
            accounts: 100,
            transactions: 50,
            file_size_bytes: 1024,
            pool_connections: 3,
            pool_idle_connections: 2,
        };

        assert_eq!(stats.projects, 5);
        assert_eq!(stats.environments, 10);
        assert_eq!(stats.accounts, 100);
        assert_eq!(stats.transactions, 50);
    }
}
