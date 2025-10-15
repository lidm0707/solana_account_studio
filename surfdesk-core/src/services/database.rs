//! # Database Service Module
//!
//! This module provides database integration for the SurfDesk application.
//! It handles data persistence, migrations, and query operations across
//! all platforms with platform-specific storage backends.

use crate::database::migrations::SimpleMigration;
use crate::error::{Result, SurfDeskError};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Result structure for count queries
#[derive(Debug, QueryableByName)]
pub struct CountResult {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub count: i64,
}

/// Database service for data persistence
pub struct DatabaseService {
    /// Database connection pool
    pool: Arc<Pool<ConnectionManager<SqliteConnection>>>,
    /// Database configuration
    config: DatabaseConfig,
    /// Migration status
    migrations_run: Arc<RwLock<bool>>,
}

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Database file path
    pub database_path: String,
    /// Connection pool size
    pub pool_size: u32,
    /// Connection timeout in seconds
    pub timeout: u64,
    /// Whether to enable foreign key constraints
    pub foreign_keys: bool,
    /// Whether to enable WAL mode
    pub wal_mode: bool,
}

impl DatabaseService {
    /// Create a new database service
    pub async fn new(_config_service: &crate::services::config::ConfigService) -> Result<Self> {
        let config = DatabaseConfig::from_platform();

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

        // Create connection pool
        let manager = ConnectionManager::<SqliteConnection>::new(&config.database_path);
        let pool = Pool::builder()
            .max_size(config.pool_size)
            .build(manager)
            .map_err(|e| SurfDeskError::database(e.to_string()))?;

        let service = Self {
            pool: Arc::new(pool),
            config,
            migrations_run: Arc::new(RwLock::new(false)),
        };

        // Run migrations
        service.run_migrations().await?;

        // Configure database
        let mut conn = service.get_connection().await?;
        service.configure_database(&mut conn)?;

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
            .pool
            .get()
            .map_err(|e| SurfDeskError::database(e.to_string()))?;

        // Run embedded migrations
        self.run_embedded_migrations(&mut conn)?;

        // Configure database settings
        self.configure_database(&mut conn)?;

        *migrations_run = true;
        log::info!("Database migrations completed");
        Ok(())
    }

    /// Run embedded migrations
    fn run_embedded_migrations(&self, conn: &mut SqliteConnection) -> Result<()> {
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
            diesel::sql_query(migration)
                .execute(conn)
                .map_err(|e| SurfDeskError::database(e.to_string()))?;
        }

        Ok(())
    }

    /// Configure database settings
    fn configure_database(&self, conn: &mut SqliteConnection) -> Result<()> {
        // Enable foreign key constraints
        if self.config.foreign_keys {
            diesel::sql_query("PRAGMA foreign_keys = ON")
                .execute(conn)
                .map_err(|e| SurfDeskError::database(e.to_string()))?;
        }

        // Enable WAL mode
        if self.config.wal_mode {
            diesel::sql_query("PRAGMA journal_mode = WAL")
                .execute(conn)
                .map_err(|e| SurfDeskError::database(e.to_string()))?;
        }

        // Set connection timeout
        let timeout_sql = format!("PRAGMA busy_timeout = {}", self.config.timeout * 1000);
        diesel::sql_query(&timeout_sql)
            .execute(conn)
            .map_err(|e| SurfDeskError::database(e.to_string()))?;

        // Optimize for performance
        diesel::sql_query("PRAGMA synchronous = NORMAL")
            .execute(conn)
            .map_err(|e| SurfDeskError::database(e.to_string()))?;

        diesel::sql_query("PRAGMA cache_size = 10000")
            .execute(conn)
            .map_err(|e| SurfDeskError::database(e.to_string()))?;

        Ok(())
    }

    /// Get a database connection from the pool
    pub async fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>> {
        self.pool
            .get()
            .map_err(|e| SurfDeskError::database(e.to_string()))
    }

    /// Get database statistics
    pub async fn get_stats(&self) -> Result<DatabaseStats> {
        let mut conn = self.get_connection().await?;

        // Get table counts
        let project_count: i64 = diesel::sql_query("SELECT COUNT(*) FROM projects")
            .load::<CountResult>(&mut conn)?
            .first()
            .map(|r| r.count)
            .unwrap_or(0);

        let environment_count: i64 = diesel::sql_query("SELECT COUNT(*) FROM environments")
            .load::<CountResult>(&mut conn)?
            .first()
            .map(|r| r.count)
            .unwrap_or(0);

        let account_count: i64 = diesel::sql_query("SELECT COUNT(*) FROM accounts")
            .load::<CountResult>(&mut conn)?
            .first()
            .map(|r| r.count)
            .unwrap_or(0);

        let transaction_count: i64 = diesel::sql_query("SELECT COUNT(*) FROM transactions")
            .load::<CountResult>(&mut conn)?
            .first()
            .map(|r| r.count)
            .unwrap_or(0);

        // Get database file size
        let metadata = std::fs::metadata(&self.config.database_path)?;
        let file_size = metadata.len();

        // Get pool statistics
        let pool_state = self.pool.state();

        Ok(DatabaseStats {
            projects: project_count as usize,
            environments: environment_count as usize,
            accounts: account_count as usize,
            transactions: transaction_count as usize,
            file_size_bytes: file_size,
            pool_connections: pool_state.connections,
            pool_idle_connections: pool_state.idle_connections,
        })
    }

    /// Backup database
    pub async fn backup(&self, backup_path: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;

        // Use SQLite backup API
        let backup_path = std::path::Path::new(backup_path);
        if let Some(parent) = backup_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        diesel::sql_query(&format!("VACUUM INTO '{}'", backup_path.display()))
            .execute(&mut conn)
            .map_err(|e| SurfDeskError::database(e.to_string()))?;

        log::info!("Database backed up to: {}", backup_path.display());
        Ok(())
    }

    /// Restore database from backup
    pub async fn restore(&self, backup_path: &str) -> Result<()> {
        let backup_path = std::path::Path::new(backup_path);

        if !backup_path.exists() {
            return Err(SurfDeskError::not_found(format!(
                "Backup file not found: {}",
                backup_path.display()
            )));
        }

        // Close all connections and replace database file
        drop(self.pool.clone());

        std::fs::copy(backup_path, &self.config.database_path)?;

        log::info!("Database restored from: {}", backup_path.display());
        Ok(())
    }

    /// Vacuum database to reclaim space
    pub async fn vacuum(&self) -> Result<()> {
        let mut conn = self.get_connection().await?;
        diesel::sql_query("VACUUM")
            .execute(&mut conn)
            .map_err(|e| SurfDeskError::database(e.to_string()))?;

        log::info!("Database vacuumed");
        Ok(())
    }

    /// Analyze database for query optimization
    pub async fn analyze(&self) -> Result<()> {
        let mut conn = self.get_connection().await?;
        diesel::sql_query("ANALYZE")
            .execute(&mut conn)
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
