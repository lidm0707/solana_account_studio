//! # Database Migrations
//!
//! This module provides simple database schema creation for the SurfDesk application.
//! For Step 1.3, we're using a simplified approach without complex migration management.

use crate::error::{Result, SurfDeskError};
use libsql::Connection;

/// Simple migration runner for creating database tables
pub struct SimpleMigration;

impl SimpleMigration {
    /// Create all database tables
    pub async fn create_tables(conn: &mut Connection) -> Result<()> {
        // Projects table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                config TEXT,
                status TEXT NOT NULL DEFAULT 'active',
                owner TEXT,
                tags TEXT
            )
            "#,
            (),
        )
        .await
        .map_err(|e| SurfDeskError::database(format!("Failed to create projects table: {}", e)))?;

        // Environments table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS environments (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                name TEXT NOT NULL,
                type TEXT NOT NULL,
                config TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                rpc_url TEXT NOT NULL,
                ws_url TEXT,
                is_default BOOLEAN NOT NULL DEFAULT FALSE,
                priority INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            )
            "#,
            (),
        )
        .await
        .map_err(|e| {
            SurfDeskError::database(format!("Failed to create environments table: {}", e))
        })?;

        // Accounts table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS accounts (
                id TEXT PRIMARY KEY,
                environment_id TEXT NOT NULL,
                pubkey TEXT NOT NULL UNIQUE,
                account_data TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                label TEXT,
                account_type TEXT NOT NULL DEFAULT 'user',
                is_watched BOOLEAN NOT NULL DEFAULT FALSE,
                balance BIGINT,
                owner TEXT,
                executable BOOLEAN NOT NULL DEFAULT FALSE,
                rent_epoch BIGINT,
                FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE
            )
            "#,
            (),
        )
        .await
        .map_err(|e| SurfDeskError::database(format!("Failed to create accounts table: {}", e)))?;

        // Transactions table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS transactions (
                id TEXT PRIMARY KEY,
                environment_id TEXT NOT NULL,
                signature TEXT NOT NULL UNIQUE,
                transaction_data TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                confirmed_at TIMESTAMP,
                slot BIGINT,
                block_height BIGINT,
                fee BIGINT,
                error TEXT,
                transaction_type TEXT NOT NULL,
                accounts TEXT,
                memo TEXT,
                FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE
            )
            "#,
            (),
        )
        .await
        .map_err(|e| {
            SurfDeskError::database(format!("Failed to create transactions table: {}", e))
        })?;

        // Programs table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS programs (
                id TEXT PRIMARY KEY,
                environment_id TEXT NOT NULL,
                program_id TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                description TEXT,
                path TEXT,
                version TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                idl TEXT,
                upgrade_authority TEXT,
                is_deployed BOOLEAN NOT NULL DEFAULT FALSE,
                is_verified BOOLEAN NOT NULL DEFAULT FALSE,
                FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE
            )
            "#,
            (),
        )
        .await
        .map_err(|e| SurfDeskError::database(format!("Failed to create programs table: {}", e)))?;

        // Settings table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS settings (
                id TEXT PRIMARY KEY,
                key TEXT NOT NULL UNIQUE,
                value TEXT NOT NULL,
                category TEXT NOT NULL,
                description TEXT,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                is_user_configurable BOOLEAN NOT NULL DEFAULT TRUE,
                data_type TEXT NOT NULL DEFAULT 'string',
                validation TEXT
            )
            "#,
            (),
        )
        .await
        .map_err(|e| SurfDeskError::database(format!("Failed to create settings table: {}", e)))?;

        // Create indexes for performance
        let indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_projects_owner ON projects (owner)",
            "CREATE INDEX IF NOT EXISTS idx_environments_project_id ON environments (project_id)",
            "CREATE INDEX IF NOT EXISTS idx_accounts_environment_id ON accounts (environment_id)",
            "CREATE INDEX IF NOT EXISTS idx_transactions_environment_id ON transactions (environment_id)",
        ];

        for index_sql in indexes {
            conn.execute(index_sql, ())
                .await
                .map_err(|e| SurfDeskError::database(format!("Failed to create index: {}", e)))?;
        }

        Ok(())
    }

    /// Initialize database with basic setup
    pub async fn initialize_database(conn: &mut Connection) -> Result<()> {
        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", ())
            .await
            .map_err(|e| {
                SurfDeskError::database(format!("Failed to enable foreign keys: {}", e))
            })?;

        // Enable WAL mode for better concurrency
        conn.execute("PRAGMA journal_mode = WAL", ())
            .await
            .map_err(|e| SurfDeskError::database(format!("Failed to enable WAL mode: {}", e)))?;

        // Create tables
        Self::create_tables(conn).await?;

        Ok(())
    }
}

/// Database initialization utilities
pub struct DatabaseInitializer;

impl DatabaseInitializer {
    /// Initialize database with all tables and migrations
    pub async fn initialize(database_url: &str) -> Result<()> {
        let db = libsql::Builder::new_local(database_url)
            .build()
            .await
            .map_err(|e| SurfDeskError::database(format!("Failed to open database: {}", e)))?;

        let mut conn = db
            .connect()
            .map_err(|e| SurfDeskError::database(e.to_string()))?;

        SimpleMigration::initialize_database(&mut conn)
            .await
            .map_err(|e| {
                SurfDeskError::database(format!("Failed to initialize database: {}", e))
            })?;

        Ok(())
    }
}
