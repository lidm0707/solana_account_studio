//! # Database Migrations
//!
//! This module provides simple database schema creation for the SurfDesk application.
//! For Step 1.3, we're using a simplified approach without complex migration management.

use crate::error::{Result, SurfDeskError};
use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

/// Simple migration runner for creating database tables
pub struct SimpleMigration;

impl SimpleMigration {
    /// Create all database tables
    pub fn create_tables(conn: &mut SqliteConnection) -> Result<()> {
        conn.batch_execute(
            r#"
            -- Projects table
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
            );

            -- Environments table
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
            );

            -- Accounts table
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
            );

            -- Transactions table
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
            );

            -- Programs table
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
            );

            -- Settings table
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
            );
            "#,
        )
        .map_err(|e| SurfDeskError::database(format!("Failed to create database tables: {}", e)))?;

        // Create indexes for performance
        conn.batch_execute(
            r#"
            CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status);
            CREATE INDEX IF NOT EXISTS idx_environments_project_id ON environments(project_id);
            CREATE INDEX IF NOT EXISTS idx_accounts_environment_id ON accounts(environment_id);
            CREATE INDEX IF NOT EXISTS idx_accounts_pubkey ON accounts(pubkey);
            CREATE INDEX IF NOT EXISTS idx_transactions_environment_id ON transactions(environment_id);
            CREATE INDEX IF NOT EXISTS idx_transactions_signature ON transactions(signature);
            CREATE INDEX IF NOT EXISTS idx_programs_environment_id ON programs(environment_id);
            CREATE INDEX IF NOT EXISTS idx_programs_program_id ON programs(program_id);
            CREATE INDEX IF NOT EXISTS idx_settings_key ON settings(key);
            "#,
        ).map_err(|e| {
            SurfDeskError::database(format!("Failed to create database indexes: {}", e))
        })?;

        log::info!("Database tables created successfully");
        Ok(())
    }

    /// Initialize database with basic setup
    pub fn initialize_database(conn: &mut SqliteConnection) -> Result<()> {
        // Enable foreign keys
        diesel::sql_query("PRAGMA foreign_keys = ON")
            .execute(&mut *conn)
            .map_err(|e| {
                SurfDeskError::database(format!("Failed to enable foreign keys: {}", e))
            })?;

        // Enable WAL mode for better concurrency
        diesel::sql_query("PRAGMA journal_mode = WAL")
            .execute(&mut *conn)
            .map_err(|e| SurfDeskError::database(format!("Failed to enable WAL mode: {}", e)))?;

        // Create tables
        Self::create_tables(conn)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_simple_migration() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_url = db_path.to_string_lossy();

        let mut conn = SqliteConnection::establish(&db_url).unwrap();
        let result = SimpleMigration::initialize_database(&mut conn);
        assert!(result.is_ok());
    }
}
