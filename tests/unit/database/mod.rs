//! Database unit tests module
//!
//! This module contains unit tests for all database-related functionality in the SurfDesk application.
//! Tests focus on database schema, migrations, queries, transactions, data validation,
//! and connection management across different supported database backends.

// Import all database test modules
mod schema_tests;
mod migration_tests;
mod query_tests;

// Re-export common test utilities
pub use crate::common::*;

/// Test configuration for database tests
pub mod database_test_config {
    /// Default database test timeout
    pub const DEFAULT_TIMEOUT: u64 = 15000;

    /// Test database types
    pub const SUPPORTED_DATABASES: &[&str] = &["sqlite", "postgresql", "mysql"];

    /// Test table names
    pub const TEST_TABLES: &[&str] = &[
        "accounts",
        "transactions",
        "environments",
        "projects",
        "surfpool_configs",
    ];

    /// Test database URLs for different backends
    pub const TEST_DATABASE_URLS: &[(&str, &str)] = &[
        ("sqlite", "sqlite::memory:"),
        ("sqlite_file", "sqlite:test_surfdesk.db"),
        ("postgresql", "postgresql://test:test@localhost:5432/test_surfdesk"),
        ("mysql", "mysql://test:test@localhost:3306/test_surfdesk"),
    ];
}

/// Setup function for database tests
pub fn setup_database_tests() {
    // Initialize test environment for database testing
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();
}

/// Clean up function for database tests
pub fn cleanup_database_tests() {
    // Clean up test databases, temporary files, etc.
    // This is called after database tests complete
}

/// Mock database factory for testing
pub struct MockDatabaseFactory;

impl MockDatabaseFactory {
    /// Create an in-memory SQLite database for testing
    pub fn create_memory_sqlite() -> String {
        "sqlite::memory:".to_string()
    }

    /// Create a temporary file-based SQLite database for testing
    pub fn create_temp_sqlite() -> String {
        format!("sqlite:test_{}.db", uuid::Uuid::new_v4())
    }

    /// Create test database connection pool
    pub async fn create_test_pool(database_url: &str) -> Result<sqlx::Pool<sqlx::Sqlite>, sqlx::Error> {
        match database_url {
            url if url.starts_with("sqlite:") => {
                let pool = sqlx::SqlitePool::connect(url).await?;
                Ok(pool)
            }
            _ => Err(sqlx::Error::Configuration(
                "Unsupported database type for testing".into(),
            )),
        }
    }

    /// Initialize test database schema
    pub async fn init_test_schema(pool: &sqlx::Pool<sqlx::Sqlite>) -> Result<(), sqlx::Error> {
        // Create test tables
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS test_accounts (
                id TEXT PRIMARY KEY,
                pubkey TEXT NOT NULL UNIQUE,
                lamports BIGINT NOT NULL,
                data BLOB,
                owner TEXT NOT NULL,
                executable BOOLEAN NOT NULL DEFAULT FALSE,
                rent_epoch BIGINT NOT NULL DEFAULT 0,
                label TEXT,
                account_type TEXT NOT NULL DEFAULT 'unknown',
                is_watched BOOLEAN NOT NULL DEFAULT FALSE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS test_transactions (
                id TEXT PRIMARY KEY,
                signature TEXT NOT NULL UNIQUE,
                slot BIGINT NOT NULL,
                block_time INTEGER,
                status TEXT NOT NULL DEFAULT 'pending',
                error TEXT,
                meta_json TEXT,
                message_json TEXT,
                fee BIGINT NOT NULL DEFAULT 0,
                compute_units_consumed INTEGER,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

/// Database test utilities
pub struct DatabaseTestUtils;

impl DatabaseTestUtils {
    /// Verify database connection is healthy
    pub async fn verify_connection(pool: &sqlx::Pool<sqlx::Sqlite>) -> bool {
        sqlx::query("SELECT 1")
            .fetch_one(pool)
            .await
            .is_ok()
    }

    /// Generate test account data
    pub fn generate_test_account() -> crate::common::mock_data::Account {
        crate::common::mock_data::MockAccount::default()
    }

    /// Generate test transaction data
    pub fn generate_test_transaction() -> crate::common::mock_data::Transaction {
        crate::common::mock_data::MockTransaction::confirmed()
    }

    /// Generate test environment data
    pub fn generate_test_environment() -> crate::common::mock_data::Environment {
        crate::common::mock_data::MockEnvironment::development()
    }

    /// Generate test project data
    pub fn generate_test_project() -> crate::common::mock_data::Project {
        crate::common::mock_data::MockProject::surfpool()
    }

    /// Insert test data into database
    pub async fn insert_test_data(
        pool: &sqlx::Pool<sqlx::Sqlite>,
        table: &str,
        data: serde_json::Value,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        match table {
            "test_accounts" => {
                sqlx::query(
                    r#"
                    INSERT INTO test_accounts (id, pubkey, lamports, owner, executable, rent_epoch, label, account_type, is_watched)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                    "#,
                )
                .bind(data["id"].as_str().unwrap_or(""))
                .bind(data["pubkey"].as_str().unwrap_or(""))
                .bind(data["lamports"].as_i64().unwrap_or(0))
                .bind(data["owner"].as_str().unwrap_or(""))
                .bind(data["executable"].as_bool().unwrap_or(false))
                .bind(data["rent_epoch"].as_i64().unwrap_or(0))
                .bind(data["label"].as_str())
                .bind(data["account_type"].as_str().unwrap_or("unknown"))
                .bind(data["is_watched"].as_bool().unwrap_or(false))
                .execute(pool)
                .await
            }
            "test_transactions" => {
                sqlx::query(
                    r#"
                    INSERT INTO test_transactions (id, signature, slot, block_time, status, error, fee)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                    "#,
                )
                .bind(data["id"].as_str().unwrap_or(""))
                .bind(data["signature"].as_str().unwrap_or(""))
                .bind(data["slot"].as_i64().unwrap_or(0))
                .bind(data["block_time"].as_i64())
                .bind(data["status"].as_str().unwrap_or("pending"))
                .bind(data["error"].as_str())
                .bind(data["fee"].as_i64().unwrap_or(0))
                .execute(pool)
                .await
            }
            _ => Err(sqlx::Error::Configuration(
                "Unsupported test table".into(),
            )),
        }
    }

    /// Count test records in a table
    pub async fn count_test_records(
        pool: &sqlx::Pool<sqlx::Sqlite>,
        table: &str,
    ) -> Result<i64, sqlx::Error> {
        let query = format!("SELECT COUNT(*) FROM {}", table);
        let result: (i64,) = sqlx::query_as(&query).fetch_one(pool).await?;
        Ok(result.0)
    }

    /// Clean test data from a table
    pub async fn clean_test_table(
        pool: &sqlx::Pool<sqlx::Sqlite>,
        table: &str,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!("DELETE FROM {}", table);
        sqlx::query(&query).execute(pool).await
    }
}

#[cfg(test)]
mod test_helpers {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_database_test_configuration() {
        assert!(!database_test_config::SUPPORTED_DATABASES.is_empty());
        assert!(!database_test_config::TEST_TABLES.is_empty());
        assert!(!database_test_config::TEST_DATABASE_URLS.is_empty());
        assert!(database_test_config::DEFAULT_TIMEOUT > 0);
    }

    #[tokio::test]
    async fn test_setup_database_tests() {
        // Should not panic
        setup_database_tests();
        cleanup_database_tests();
    }

    #[tokio::test]
    async fn test_mock_database_factory() {
        let memory_url = MockDatabaseFactory::create_memory_sqlite();
        assert_eq!(memory_url, "sqlite::memory:");

        let temp_url = MockDatabaseFactory::create_temp_sqlite();
        assert!(temp_url.starts_with("sqlite:test_"));
        assert!(temp_url.ends_with(".db"));
    }

    #[tokio::test]
    async fn test_create_test_pool() {
        let pool = MockDatabaseFactory::create_test_pool("sqlite::memory:").await;
        assert!(pool.is_ok());

        let pool_result = pool.unwrap();
        assert!(DatabaseTestUtils::verify_connection(&pool_result).await);

        // Clean up
        pool_result.close().await;
    }

    #[tokio::test]
    async fn test_init_test_schema() {
        let pool = MockDatabaseFactory::create_test_pool("sqlite::memory:").await.unwrap();

        let schema_result = MockDatabaseFactory::init_test_schema(&pool).await;
        assert!(schema_result.is_ok());

        // Verify tables were created
        let account_count = DatabaseTestUtils::count_test_records(&pool, "test_accounts").await.unwrap();
        let transaction_count = DatabaseTestUtils::count_test_records(&pool, "test_transactions").await.unwrap();

        assert_eq!(account_count, 0);
        assert_eq!(transaction_count, 0);

        // Clean up
        pool.close().await;
    }

    #[tokio::test]
    async fn test_database_test_utils() {
        let pool = MockDatabaseFactory::create_test_pool("sqlite::memory:").await.unwrap();
        MockDatabaseFactory::init_test_schema(&pool).await.unwrap();

        // Test data generation
        let account = DatabaseTestUtils::generate_test_account();
        let transaction = DatabaseTestUtils::generate_test_transaction();
        let environment = DatabaseTestUtils::generate_test_environment();
        let project = DatabaseTestUtils::generate_test_project();

        assert!(!account.pubkey.is_empty());
        assert!(!transaction.signature.is_empty());
        assert!(!environment.id.is_empty());
        assert!(!project.id.is_empty());

        // Test data insertion
        let account_data = serde_json::json!({
            "id": "test_account_1",
            "pubkey": account.pubkey,
            "lamports": account.lamports,
            "owner": account.owner,
            "executable": account.executable,
            "rent_epoch": account.rent_epoch,
            "label": account.label,
            "account_type": account.account_type,
            "is_watched": account.is_watched
        });

        let insert_result = DatabaseTestUtils::insert_test_data(
            &pool,
            "test_accounts",
            account_data,
        ).await;
        assert!(insert_result.is_ok());

        // Test record counting
        let count = DatabaseTestUtils::count_test_records(&pool, "test_accounts").await.unwrap();
        assert_eq!(count, 1);

        // Test data cleanup
        let cleanup_result = DatabaseTestUtils::clean_test_table(&pool, "test_accounts").await.unwrap();
        assert!(cleanup_result.rows_affected() > 0);

        let count_after_cleanup = DatabaseTestUtils::count_test_records(&pool, "test_accounts").await.unwrap();
        assert_eq!(count_after_cleanup, 0);

        // Clean up
        pool.close().await;
    }

    #[test]
    fn test_database_url_patterns() {
        for (db_type, url) in database_test_config::TEST_DATABASE_URLS {
            match *db_type {
                "sqlite" | "sqlite_file" => {
                    assert!(url.starts_with("sqlite:"));
                }
                "postgresql" => {
                    assert!(url.starts_with("postgresql://"));
                    assert!(url.contains("test_surfdesk"));
                }
                "mysql" => {
                    assert!(url.starts_with("mysql://"));
                    assert!(url.contains("test_surfdesk"));
                }
                _ => panic!("Unsupported database type: {}", db_type),
            }
        }
    }

    #[test]
    fn test_table_name_patterns() {
        for table in database_test_config::TEST_TABLES {
            assert!(
                table.chars().all(|c| c.is_ascii_lowercase() || c == '_'),
                "Table name should be lowercase with underscores: {}",
                table
            );
            assert!(!table.trim().is_empty(), "Table name should not be empty");
        }
    }

    #[test]
    fn test_database_types_integrity() {
        for db_type in database_test_config::SUPPORTED_DATABASES {
            match *db_type {
                "sqlite" | "postgresql" | "mysql" => {
                    // Valid database types
                    assert!(true);
                }
                _ => panic!("Invalid database type: {}", db_type),
            }
        }
    }

    #[test]
    fn test_database_data_integrity() {
        let account = DatabaseTestUtils::generate_test_account();
        let transaction = DatabaseTestUtils::generate_test_transaction();
        let environment = DatabaseTestUtils::generate_test_environment();
        let project = DatabaseTestUtils::generate_test_project();

        // Verify data integrity
        assert_ne!(account.pubkey, "");
        assert_ne!(transaction.signature, "");
        assert_ne!(environment.id, "");
        assert_ne!(project.id, "");

        // Verify data uniqueness
        assert_ne!(account.pubkey, transaction.signature);
        assert_ne!(environment.id, project.id);

        // Verify data format consistency
        assert_eq!(account.pubkey.len(), 44); // Standard Solana pubkey length
        assert!(transaction.signature.len() >= 80); // Transaction signature length
    }
}
