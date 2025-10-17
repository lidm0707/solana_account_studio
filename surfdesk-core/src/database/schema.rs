//! # Database Schema
//!
//! This module defines the database schema for the SurfDesk application.
//! It includes tables for projects, environments, accounts, and other
//! core entities using plain SQL instead of Diesel ORM.

use serde::{Deserialize, Serialize};

/// Projects table schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSchema {
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

/// Environments table schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentSchema {
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

/// Accounts table schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSchema {
    pub id: String,
    pub environment_id: String,
    pub pubkey: String,
    pub label: Option<String>,
    pub balance: Option<f64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub is_active: bool,
}

/// Transactions table schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSchema {
    pub id: String,
    pub environment_id: String,
    pub account_id: String,
    pub signature: String,
    pub slot: Option<i64>,
    pub block_time: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    pub error: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Mentions table schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentionSchema {
    pub id: String,
    pub environment_id: String,
    pub account_id: String,
    pub mention_type: String,
    pub content: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub confirmed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// SQL schema definitions for table creation
pub const PROJECTS_TABLE_SQL: &str = r#"
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
"#;

pub const ENVIRONMENTS_TABLE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS environments (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    name TEXT NOT NULL,
    type TEXT NOT NULL DEFAULT 'development',
    config TEXT NOT NULL DEFAULT '{}',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    rpc_url TEXT NOT NULL,
    ws_url TEXT,
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    priority INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE
)
"#;

pub const ACCOUNTS_TABLE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS accounts (
    id TEXT PRIMARY KEY,
    environment_id TEXT NOT NULL,
    pubkey TEXT NOT NULL UNIQUE,
    label TEXT,
    balance REAL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    FOREIGN KEY (environment_id) REFERENCES environments (id) ON DELETE CASCADE
)
"#;

pub const TRANSACTIONS_TABLE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS transactions (
    id TEXT PRIMARY KEY,
    environment_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    signature TEXT NOT NULL UNIQUE,
    slot INTEGER,
    block_time TIMESTAMP,
    status TEXT NOT NULL DEFAULT 'pending',
    error TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (environment_id) REFERENCES environments (id) ON DELETE CASCADE,
    FOREIGN KEY (account_id) REFERENCES accounts (id) ON DELETE CASCADE
)
"#;

pub const MENTIONS_TABLE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS mentions (
    id TEXT PRIMARY KEY,
    environment_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    mention_type TEXT NOT NULL DEFAULT 'general',
    content TEXT NOT NULL,
    metadata TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    confirmed_at TIMESTAMP,
    FOREIGN KEY (environment_id) REFERENCES environments (id) ON DELETE CASCADE,
    FOREIGN KEY (account_id) REFERENCES accounts (id) ON DELETE CASCADE
)
"#;

pub const INDEXES_SQL: &[&str] = &[
    "CREATE INDEX IF NOT EXISTS idx_projects_owner ON projects (owner)",
    "CREATE INDEX IF NOT EXISTS idx_environments_project_id ON environments (project_id)",
    "CREATE INDEX IF NOT EXISTS idx_accounts_environment_id ON accounts (environment_id)",
    "CREATE INDEX IF NOT EXISTS idx_transactions_environment_id ON transactions (environment_id)",
    "CREATE INDEX IF NOT EXISTS idx_transactions_account_id ON transactions (account_id)",
    "CREATE INDEX IF NOT EXISTS idx_mentions_environment_id ON mentions (environment_id)",
    "CREATE INDEX IF NOT EXISTS idx_mentions_account_id ON mentions (account_id)",
];
