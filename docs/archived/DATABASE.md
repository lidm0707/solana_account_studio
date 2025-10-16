# Database Design & Architecture for SurfDesk (Dioxus 0.6+ Multi-Platform)

## Overview

SurfDesk implements a comprehensive, cross-platform database architecture using **SQLite** with **Diesel ORM** for persistent storage across desktop, web, and terminal platforms. The database system is designed to handle multi-platform data synchronization, offline-first operation, and efficient cross-platform data access patterns.

## Multi-Platform Database Strategy

### 🖥️ **Desktop Platform**
- **Native SQLite**: Direct file-based SQLite database
- **Local Storage**: Full file system access for database files
- **Performance**: Optimized for desktop hardware capabilities
- **Backup/Restore**: Native file system operations

### 🌐 **Web Platform** 
- **WASM-SQLite**: SQLite compiled to WebAssembly
- **Browser Storage**: IndexedDB for database persistence
- **Sync Layer**: Cloud synchronization for data consistency
- **Offline Mode**: Full offline capability with sync on reconnect

### 💻 **Terminal Platform**
- **Lightweight SQLite**: Optimized for server environments
- **File-based Storage**: Standard file system access
- **CLI Operations**: Database management via command line
- **Resource Efficiency**: Minimal memory and CPU footprint

## Database Schema Design

### Core Tables

#### Projects Table
```sql
CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    config JSON NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    platform_metadata JSON,
    sync_status TEXT DEFAULT 'local',
    last_synced_at DATETIME
);
```

#### Environments Table
```sql
CREATE TABLE environments (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    name TEXT NOT NULL,
    type TEXT NOT NULL, -- 'local', 'fork', 'custom'
    config JSON NOT NULL,
    status TEXT DEFAULT 'stopped', -- 'running', 'stopped', 'error'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    platform_specific JSON,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);
```

#### Programs Table
```sql
CREATE TABLE programs (
    id TEXT PRIMARY KEY,
    environment_id TEXT NOT NULL,
    name TEXT NOT NULL,
    pubkey TEXT NOT NULL,
    program_data JSON NOT NULL,
    idl JSON,
    deployment_slot INTEGER,
    deployment_hash TEXT,
    status TEXT DEFAULT 'pending', -- 'pending', 'deployed', 'failed'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE
);
```

#### Accounts Table
```sql
CREATE TABLE accounts (
    id TEXT PRIMARY KEY,
    environment_id TEXT NOT NULL,
    pubkey TEXT NOT NULL,
    owner TEXT NOT NULL,
    lamports BIGINT NOT NULL,
    account_data JSON NOT NULL,
    slot INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE
);
```

#### Transactions Table
```sql
CREATE TABLE transactions (
    id TEXT PRIMARY KEY,
    environment_id TEXT NOT NULL,
    signature TEXT NOT NULL,
    slot INTEGER NOT NULL,
    block_time INTEGER,
    fee BIGINT NOT NULL,
    status TEXT NOT NULL, -- 'pending', 'confirmed', 'failed'
    error_message TEXT,
    transaction_data JSON NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE
);
```

#### Snapshots Table
```sql
CREATE TABLE snapshots (
    id TEXT PRIMARY KEY,
    environment_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    slot INTEGER NOT NULL,
    accounts JSON NOT NULL,
    programs JSON NOT NULL,
    metadata JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    size_bytes INTEGER,
    FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE
);
```

#### Test Plans Table
```sql
CREATE TABLE test_plans (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    plan_data JSON NOT NULL,
    ai_generated BOOLEAN DEFAULT FALSE,
    status TEXT DEFAULT 'draft', -- 'draft', 'active', 'archived'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);
```

#### Test Results Table
```sql
CREATE TABLE test_results (
    id TEXT PRIMARY KEY,
    test_plan_id TEXT NOT NULL,
    environment_id TEXT NOT NULL,
    execution_time DATETIME DEFAULT CURRENT_TIMESTAMP,
    status TEXT NOT NULL, -- 'passed', 'failed', 'error'
    results JSON NOT NULL,
    metrics JSON,
    error_message TEXT,
    FOREIGN KEY (test_plan_id) REFERENCES test_plans(id) ON DELETE CASCADE,
    FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE
);
```

### Cross-Platform Synchronization Tables

#### Sync Queue Table
```sql
CREATE TABLE sync_queue (
    id TEXT PRIMARY KEY,
    table_name TEXT NOT NULL,
    record_id TEXT NOT NULL,
    operation TEXT NOT NULL, -- 'create', 'update', 'delete'
    data JSON NOT NULL,
    platform TEXT NOT NULL,
    status TEXT DEFAULT 'pending', -- 'pending', 'syncing', 'completed', 'failed'
    retry_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    processed_at DATETIME
);
```

#### Platform Metadata Table
```sql
CREATE TABLE platform_metadata (
    id TEXT PRIMARY KEY,
    platform TEXT NOT NULL,
    key TEXT NOT NULL,
    value JSON NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(platform, key)
);
```

## Database Implementation

### Core Database Service

```rust
// surfdesk-core/src/database/mod.rs
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<SqliteConnection>>;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Connection error: {0}")]
    ConnectionError(#[from] diesel::result::ConnectionError),
    #[error("Query error: {0}")]
    QueryError(#[from] diesel::result::Error),
    #[error("Pool error: {0}")]
    PoolError(#[from] diesel::r2d2::PoolError),
    #[error("Migration error: {0}")]
    MigrationError(String),
    #[error("Platform error: {0}")]
    PlatformError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub platform: PlatformType,
    pub database_url: String,
    pub max_connections: u32,
    pub connection_timeout: u64,
    pub enable_wal: bool,
    pub enable_foreign_keys: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            platform: PlatformType::Desktop,
            database_url: "surfdesk.db".to_string(),
            max_connections: 10,
            connection_timeout: 30,
            enable_wal: true,
            enable_foreign_keys: true,
        }
    }
}

pub struct Database {
    pool: Arc<DbPool>,
    config: DatabaseConfig,
    sync_manager: Option<Arc<SyncManager>>,
}

impl Database {
    pub async fn new(config: DatabaseConfig) -> Result<Self, DatabaseError> {
        let manager = ConnectionManager::<SqliteConnection>::new(&config.database_url);
        let pool = Pool::builder()
            .max_size(config.max_connections)
            .connection_timeout(std::time::Duration::from_secs(config.connection_timeout))
            .build(manager)
            .map_err(DatabaseError::PoolError)?;

        let db = Self {
            pool: Arc::new(pool),
            config: config.clone(),
            sync_manager: None,
        };

        // Initialize database
        db.initialize().await?;

        Ok(db)
    }

    async fn initialize(&self) -> Result<(), DatabaseError> {
        let conn = &mut self.get_connection()?;
        
        // Enable WAL mode for better concurrency
        if self.config.enable_wal {
            diesel::sql_query("PRAGMA journal_mode = WAL")
                .execute(conn)?;
        }

        // Enable foreign key constraints
        if self.config.enable_foreign_keys {
            diesel::sql_query("PRAGMA foreign_keys = ON")
                .execute(conn)?;
        }

        // Run migrations
        self.run_migrations(conn)?;

        // Set up platform-specific optimizations
        self.setup_platform_optimizations(conn)?;

        Ok(())
    }

    fn get_connection(&self) -> Result<DbConn, DatabaseError> {
        self.pool.get().map_err(DatabaseError::ConnectionError)
    }

    pub async fn create_project(&self, project: NewProject) -> Result<Project, DatabaseError> {
        let conn = &mut self.get_connection()?;
        
        let project = diesel::insert_into(projects::table)
            .values(&project)
            .returning(projects::all_columns)
            .get_result::<Project>(conn)?;

        // Queue for sync if not local
        if self.sync_manager.is_some() {
            self.queue_sync_operation("projects", &project.id, "create", &project).await?;
        }

        Ok(project)
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>, DatabaseError> {
        let conn = &mut self.get_connection()?;
        
        let projects = projects::table
            .load::<Project>(conn)?;

        Ok(projects)
    }

    pub async fn update_project(&self, project_id: &str, project: UpdateProject) -> Result<Project, DatabaseError> {
        let conn = &mut self.get_connection()?;
        
        let project = diesel::update(projects::table.find(project_id))
            .set(&project)
            .returning(projects::all_columns)
            .get_result::<Project>(conn)?;

        // Queue for sync
        if self.sync_manager.is_some() {
            self.queue_sync_operation("projects", project_id, "update", &project).await?;
        }

        Ok(project)
    }

    pub async fn delete_project(&self, project_id: &str) -> Result<(), DatabaseError> {
        let conn = &mut self.get_connection()?;
        
        diesel::delete(projects::table.find(project_id))
            .execute(conn)?;

        // Queue for sync
        if self.sync_manager.is_some() {
            self.queue_sync_operation("projects", project_id, "delete", &serde_json::json!({})).await?;
        }

        Ok(())
    }

    async fn queue_sync_operation(&self, table: &str, record_id: &str, operation: &str, data: &serde_json::Value) -> Result<(), DatabaseError> {
        if let Some(sync_manager) = &self.sync_manager {
            sync_manager.queue_operation(table, record_id, operation, data).await?;
        }
        Ok(())
    }
}
```

### Platform-Specific Implementations

#### Desktop Database Implementation

```rust
// surfdesk-desktop/src/database/desktop.rs
use surfdesk_core::database::*;
use std::path::PathBuf;

pub struct DesktopDatabase {
    database: Database,
    backup_manager: BackupManager,
}

impl DesktopDatabase {
    pub async fn new() -> Result<Self, DatabaseError> {
        let data_dir = dirs::data_dir()
            .ok_or_else(|| DatabaseError::PlatformError("No data directory found".to_string()))?
            .join("surfdesk");

        std::fs::create_dir_all(&data_dir)
            .map_err(|e| DatabaseError::PlatformError(e.to_string()))?;

        let config = DatabaseConfig {
            platform: PlatformType::Desktop,
            database_url: data_dir.join("surfdesk.db").to_string_lossy().to_string(),
            max_connections: 15,
            connection_timeout: 30,
            enable_wal: true,
            enable_foreign_keys: true,
        };

        let database = Database::new(config).await?;
        let backup_manager = BackupManager::new(&data_dir);

        Ok(Self {
            database,
            backup_manager,
        })
    }

    pub async fn create_backup(&self, name: &str) -> Result<PathBuf, DatabaseError> {
        self.backup_manager.create_backup(&self.database, name).await
    }

    pub async fn restore_backup(&self, backup_path: &PathBuf) -> Result<(), DatabaseError> {
        self.backup_manager.restore_backup(&self.database, backup_path).await
    }

    pub async fn get_database_info(&self) -> Result<DatabaseInfo, DatabaseError> {
        let conn = &mut self.database.get_connection()?;
        
        let size: i64 = diesel::sql_query("SELECT page_count * page_size as size FROM pragma_page_count(), pragma_page_size()")
            .get_result(conn)?;

        Ok(DatabaseInfo {
            size_bytes: size as u64,
            table_count: self.get_table_count(conn)?,
            platform: PlatformType::Desktop,
        })
    }

    fn get_table_count(&self, conn: &mut SqliteConnection) -> Result<u32, DatabaseError> {
        let count: i64 = diesel::sql_query("SELECT COUNT(*) FROM sqlite_master WHERE type='table'")
            .get_result(conn)?;
        Ok(count as u32)
    }
}

pub struct BackupManager {
    backup_dir: PathBuf,
}

impl BackupManager {
    pub fn new(data_dir: &PathBuf) -> Self {
        let backup_dir = data_dir.join("backups");
        std::fs::create_dir_all(&backup_dir).ok();
        
        Self { backup_dir }
    }

    pub async fn create_backup(&self, database: &Database, name: &str) -> Result<PathBuf, DatabaseError> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_filename = format!("{}_{}.db", name, timestamp);
        let backup_path = self.backup_dir.join(backup_filename);

        // Use SQLite backup API
        let conn = &mut database.get_connection()?;
        let backup_conn = SqliteConnection::establish(&backup_path.to_string_lossy())?;

        conn.backup(&backup_conn, None, Some(|progress| {
            log::debug!("Backup progress: {}%", progress.page_count * 100 / progress.remaining);
        }))?;

        Ok(backup_path)
    }

    pub async fn restore_backup(&self, database: &Database, backup_path: &PathBuf) -> Result<(), DatabaseError> {
        // Close existing connections
        // Restore from backup
        // Reinitialize database
        
        let backup_conn = SqliteConnection::establish(&backup_path.to_string_lossy())?;
        let conn = &mut database.get_connection()?;

        backup_conn.backup(conn, None, Some(|progress| {
            log::debug!("Restore progress: {}%", progress.page_count * 100 / progress.remaining);
        }))?;

        Ok(())
    }
}
```

#### Web Database Implementation

```rust
// surfdesk-web/src/database/web.rs
use surfdesk_core::database::*;
use gloo_net::http::Request;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::IdbDatabase;

pub struct WebDatabase {
    database: Database,
    sync_manager: Arc<WebSyncManager>,
    idb_instance: Option<IdbDatabase>,
}

impl WebDatabase {
    pub async fn new() -> Result<Self, DatabaseError> {
        // Initialize IndexedDB for fallback storage
        let idb_instance = Self::init_indexed_db().await.ok();
        
        // Initialize in-memory SQLite for performance
        let config = DatabaseConfig {
            platform: PlatformType::Web,
            database_url: ":memory:".to_string(),
            max_connections: 5,
            connection_timeout: 10,
            enable_wal: false, // Disabled for WASM
            enable_foreign_keys: true,
        };

        let database = Database::new(config).await?;
        let sync_manager = Arc::new(WebSyncManager::new());

        // Load data from IndexedDB
        if let Some(ref idb) = idb_instance {
            Self::load_from_indexed_db(&database, idb).await?;
        }

        Ok(Self {
            database,
            sync_manager,
            idb_instance,
        })
    }

    async fn init_indexed_db() -> Result<IdbDatabase, DatabaseError> {
        let promise = Promise::new(&mut |resolve, reject| {
            // IndexedDB initialization logic
            wasm_bindgen_futures::spawn_local(async move {
                match Self::create_idb().await {
                    Ok(db) => resolve(&db.into()),
                    Err(e) => reject(&JsValue::from_str(&e.to_string())),
                }
            });
        });

        let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
        Ok(IdbDatabase::from(result))
    }

    async fn create_idb() -> Result<IdbDatabase, DatabaseError> {
        // IndexedDB creation logic
        todo!("Implement IndexedDB creation")
    }

    async fn load_from_indexed_db(database: &Database, idb: &IdbDatabase) -> Result<(), DatabaseError> {
        // Load data from IndexedDB into SQLite
        todo!("Implement IndexedDB loading")
    }

    async fn save_to_indexed_db(&self) -> Result<(), DatabaseError> {
        if let Some(ref idb) = self.idb_instance {
            // Save current SQLite data to IndexedDB
            todo!("Implement IndexedDB saving")
        }
        Ok(())
    }

    pub async fn sync_with_cloud(&self) -> Result<(), DatabaseError> {
        self.sync_manager.sync_all().await
    }

    pub async fn enable_offline_mode(&self) -> Result<(), DatabaseError> {
        // Ensure all data is persisted to IndexedDB
        self.save_to_indexed_db().await?;
        Ok(())
    }
}

#[wasm_bindgen]
pub struct WebSyncManager {
    sync_url: String,
    api_key: Option<String>,
}

impl WebSyncManager {
    pub fn new() -> Self {
        Self {
            sync_url: "https://api.surfdesk.dev/sync".to_string(),
            api_key: None,
        }
    }

    pub async fn sync_all(&self) -> Result<(), DatabaseError> {
        // Get pending sync operations
        let pending_ops = self.get_pending_operations().await?;
        
        for op in pending_ops {
            match self.sync_operation(&op).await {
                Ok(_) => self.mark_operation_synced(&op.id).await?,
                Err(e) => log::error!("Failed to sync operation {}: {}", op.id, e),
            }
        }

        Ok(())
    }

    async fn sync_operation(&self, operation: &SyncOperation) -> Result<(), DatabaseError> {
        let payload = serde_json::to_string(operation)?;
        
        let response = Request::post(&self.sync_url)
            .header("Content-Type", "application/json")
            .body(payload)?
            .send()
            .await
            .map_err(|e| DatabaseError::PlatformError(e.to_string()))?;

        if response.ok() {
            Ok(())
        } else {
            Err(DatabaseError::PlatformError("Sync failed".to_string()))
        }
    }
}
```

#### Terminal Database Implementation

```rust
// surfdesk-tui/src/database/terminal.rs
use surfdesk_core::database::*;
use std::io::{self, Write};

pub struct TerminalDatabase {
    database: Database,
    config_dir: PathBuf,
}

impl TerminalDatabase {
    pub async fn new() -> Result<Self, DatabaseError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| DatabaseError::PlatformError("No config directory found".to_string()))?
            .join("surfdesk");

        std::fs::create_dir_all(&config_dir)
            .map_err(|e| DatabaseError::PlatformError(e.to_string()))?;

        let config = DatabaseConfig {
            platform: PlatformType::Terminal,
            database_url: config_dir.join("surfdesk.db").to_string_lossy().to_string(),
            max_connections: 3, // Lower for terminal
            connection_timeout: 15, // Faster timeout
            enable_wal: true,
            enable_foreign_keys: true,
        };

        let database = Database::new(config).await?;

        Ok(Self {
            database,
            config_dir,
        })
    }

    pub async fn export_data(&self, format: ExportFormat) -> Result<String, DatabaseError> {
        match format {
            ExportFormat::Json => self.export_json().await,
            ExportFormat::Csv => self.export_csv().await,
            ExportFormat::Sql => self.export_sql().await,
        }
    }

    async fn export_json(&self) -> Result<String, DatabaseError> {
        let conn = &mut self.database.get_connection()?;
        
        // Export all tables as JSON
        let projects: Vec<Project> = projects::table.load(conn)?;
        let environments: Vec<Environment> = environments::table.load(conn)?;
        let programs: Vec<Program> = programs::table.load(conn)?;
        
        let export = ExportData {
            projects,
            environments,
            programs,
            exported_at: chrono::Utc::now(),
        };

        Ok(serde_json::to_string_pretty(&export)?)
    }

    async fn export_csv(&self) -> Result<String, DatabaseError> {
        let mut csv_output = String::new();
        
        // Export projects as CSV
        let conn = &mut self.database.get_connection()?;
        let projects: Vec<Project> = projects::table.load(conn)?;
        
        csv_output.push_str("id,name,description,created_at,updated_at\n");
        for project in projects {
            csv_output.push_str(&format!("{},{},{},{},{}\n",
                project.id,
                project.name,
                project.description.unwrap_or_default(),
                project.created_at,
                project.updated_at
            ));
        }

        Ok(csv_output)
    }

    async fn export_sql(&self) -> Result<String, DatabaseError> {
        // Generate SQL dump
        let conn = &mut self.database.get_connection()?;
        
        let sql_dump = diesel::sql_query("
            SELECT sql FROM sqlite_master 
            WHERE type='table' AND name NOT LIKE 'sqlite_%'
            ORDER BY name
        ").load::<SqlDumpRow>(conn)?;

        let mut dump = String::new();
        for row in sql_dump {
            dump.push_str(&row.sql);
            dump.push('\n');
        }

        Ok(dump)
    }

    pub async fn import_data(&self, data: &str, format: ImportFormat) -> Result<(), DatabaseError> {
        match format {
            ImportFormat::Json => self.import_json(data).await,
            ImportFormat::Sql => self.import_sql(data).await,
        }
    }

    async fn import_json(&self, data: &str) -> Result<(), DatabaseError> {
        let import_data: ExportData = serde_json::from_str(data)?;
        let conn = &mut self.database.get_connection()?;
        
        // Import in transaction
        conn.transaction::<_, DatabaseError, _>(|conn| {
            for project in import_data.projects {
                diesel::insert_into(projects::table)
                    .values(&project)
                    .execute(conn)?;
            }
            
            for environment in import_data.environments {
                diesel::insert_into(environments::table)
                    .values(&environment)
                    .execute(conn)?;
            }
            
            for program in import_data.programs {
                diesel::insert_into(programs::table)
                    .values(&program)
                    .execute(conn)?;
            }
            
            Ok(())
        })?;

        Ok(())
    }

    pub async fn vacuum_database(&self) -> Result<(), DatabaseError> {
        let conn = &mut self.database.get_connection()?;
        diesel::sql_query("VACUUM").execute(conn)?;
        Ok(())
    }

    pub async fn analyze_database(&self) -> Result<DatabaseStats, DatabaseError> {
        let conn = &mut self.database.get_connection()?;
        
        let size: i64 = diesel::sql_query("SELECT page_count * page_size as size FROM pragma_page_count(), pragma_page_size()")
            .get_result(conn)?;

        let table_stats: Vec<TableStats> = diesel::sql_query("
            SELECT name, 
                   (SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=main.name) as exists,
                   (SELECT COUNT(*) FROM main.name) as row_count
            FROM sqlite_master 
            WHERE type='table' AND name NOT LIKE 'sqlite_%'
        ").load(conn)?;

        Ok(DatabaseStats {
            size_bytes: size as u64,
            table_stats,
            platform: PlatformType::Terminal,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportData {
    pub projects: Vec<Project>,
    pub environments: Vec<Environment>,
    pub programs: Vec<Program>,
    pub exported_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub enum ExportFormat {
    Json,
    Csv,
    Sql,
}

#[derive(Debug)]
pub enum ImportFormat {
    Json,
    Sql,
}

#[derive(Debug, QueryableByName)]
struct SqlDumpRow {
    #[diesel(sql_type = "Text")]
    sql: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub size_bytes: u64,
    pub table_stats: Vec<TableStats>,
    pub platform: PlatformType,
}

#[derive(Debug, Serialize, Deserialize, QueryableByName)]
pub struct TableStats {
    #[diesel(sql_type = "Text")]
    name: String,
    #[diesel(sql_type = "Integer")]
    exists: i32,
    #[diesel(sql_type = "Integer")]
    row_count: i64,
}
```

## Database Migrations

### Migration Management

```rust
// surfdesk-core/src/database/migrations/mod.rs
use diesel::migration::{Migration, MigrationSource};
use diesel::sqlite::SqliteConnection;
use std::path::Path;

pub struct MigrationManager;

impl MigrationManager {
    pub fn run_migrations(conn: &mut SqliteConnection) -> Result<(), DatabaseError> {
        // Embedded migrations
        let migrations = Self::embedded_migrations();
        
        for migration in migrations {
            migration.run(conn)?;
        }
        
        Ok(())
    }

    fn embedded_migrations() -> Vec<Box<dyn Migration<SqliteConnection>>> {
        vec![
            Box::new(m2024_01_01_001_initial_schema::Migration),
            Box::new(m2024_01_02_002_add_sync_tables::Migration),
            Box::new(m2024_01_03_003_add_platform_metadata::Migration),
            Box::new(m2024_01_04_004_add_test_tables::Migration),
            Box::new(m2024_01_05_005_optimize_indexes::Migration),
        ]
    }
}

// Example migration
mod m2024_01_01_001_initial_schema {
    use diesel::migration::{Migration, MigrationSource};
    use diesel::sqlite::SqliteConnection;
    use diesel::sql_types::*;
    use diesel::prelude::*;

    pub struct Migration;

    impl Migration for Migration {
        fn version(&self) -> i32 {
            20240101001
        }

        fn run(&self, conn: &mut SqliteConnection) -> Result<(), diesel::result::Error> {
            // Create projects table
            diesel::sql_query(r#"
                CREATE TABLE projects (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    description TEXT,
                    config JSON NOT NULL,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            "#).execute(conn)?;

            // Create environments table
            diesel::sql_query(r#"
                CREATE TABLE environments (
                    id TEXT PRIMARY KEY,
                    project_id TEXT NOT NULL,
                    name TEXT NOT NULL,
                    type TEXT NOT NULL,
                    config JSON NOT NULL,
                    status TEXT DEFAULT 'stopped',
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
                )
            "#).execute(conn)?;

            // Create indexes
            diesel::sql_query("CREATE INDEX idx_environments_project_id ON environments(project_id)")
                .execute(conn)?;

            Ok(())
        }

        fn revert(&self, conn: &mut SqliteConnection) -> Result<(), diesel::result::Error> {
            diesel::sql_query("DROP TABLE IF EXISTS environments").execute(conn)?;
            diesel::sql_query("DROP TABLE IF EXISTS projects").execute(conn)?;
            Ok(())
        }
    }
}
```

## Performance Optimization

### Database Performance Strategies

#### Connection Pool Management

```rust
// surfdesk-core/src/database/pool.rs
use diesel::r2d2::{Pool, ConnectionManager, PooledConnection};
use std::time::Duration;

pub struct DatabasePoolManager {
    pool: Pool<ConnectionManager<SqliteConnection>>,
    metrics: PoolMetrics,
}

impl DatabasePoolManager {
    pub fn new(database_url: &str, max_size: u32) -> Result<Self, DatabaseError> {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        
        let pool = Pool::builder()
            .max_size(max_size)
            .connection_timeout(Duration::from_secs(30))
            .idle_timeout(Some(Duration::from_secs(600))) // 10 minutes
            .max_lifetime(Some(Duration::from_secs(1800))) // 30 minutes
            .test_on_check_out(true)
            .build(manager)
            .map_err(DatabaseError::PoolError)?;

        Ok(Self {
            pool,
            metrics: PoolMetrics::default(),
        })
    }

    pub async fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, DatabaseError> {
        let start = std::time::Instant::now();
        
        let conn = self.pool.get()
            .map_err(DatabaseError::PoolError)?;
        
        let duration = start.elapsed();
        self.metrics.record_connection_wait(duration);
        
        Ok(conn)
    }

    pub fn get_pool_stats(&self) -> PoolStats {
        PoolStats {
            state: self.pool.state(),
            connections: self.pool.state().connections,
            idle_connections: self.pool.state().idle_connections,
            metrics: self.metrics.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PoolMetrics {
    pub total_connections: u64,
    pub average_wait_time: Duration,
    pub max_wait_time: Duration,
}

impl Default for PoolMetrics {
    fn default() -> Self {
        Self {
            total_connections: 0,
            average_wait_time: Duration::from_millis(0),
            max_wait_time: Duration::from_millis(0),
        }
    }
}

impl PoolMetrics {
    pub fn record_connection_wait(&mut self, duration: Duration) {
        self.total_connections += 1;
        self.max_wait_time = self.max_wait_time.max(duration);
        
        // Update running average
        let total_ms = self.average_wait_time.as_millis() as u64 * (self.total_connections - 1);
        let new_ms = total_ms + duration.as_millis() as u64;
        self.average_wait_time = Duration::from_millis(new_ms / self.total_connections);
    }
}

#[derive(Debug, Clone)]
pub struct PoolStats {
    pub state: diesel::r2d2::State,
    pub connections: u32,
    pub idle_connections: u32,
    pub metrics: PoolMetrics,
}
```

#### Query Optimization

```rust
// surfdesk-core/src/database/queries.rs
use diesel::prelude::*;
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};

pub struct QueryOptimizer;

impl QueryOptimizer {
    // Optimized project queries
    pub fn get_projects_with_stats(conn: &mut SqliteConnection) -> Result<Vec<ProjectWithStats>, DatabaseError> {
        let projects = diesel::sql_query(r#"
            SELECT 
                p.*,
                COUNT(e.id) as environment_count,
                COUNT(pr.id) as program_count,
                MAX(e.updated_at) as last_activity
            FROM projects p
            LEFT JOIN environments e ON p.id = e.project_id
            LEFT JOIN programs pr ON e.id = pr.environment_id
            GROUP BY p.id
            ORDER BY p.updated_at DESC
        "#).load::<ProjectWithStats>(conn)?;

        Ok(projects)
    }

    // Optimized account queries with pagination
    pub fn get_accounts_paginated(
        conn: &mut SqliteConnection,
        environment_id: &str,
        page: u32,
        page_size: u32,
    ) -> Result<PaginatedAccounts, DatabaseError> {
        let offset = (page - 1) * page_size;

        let accounts: Vec<Account> = accounts::table
            .filter(accounts::environment_id.eq(environment_id))
            .order(accounts::updated_at.desc())
            .limit(page_size as i64)
            .offset(offset as i64)
            .load(conn)?;

        let total_count: i64 = accounts::table
            .filter(accounts::environment_id.eq(environment_id))
            .count()
            .get_result(conn)?;

        Ok(PaginatedAccounts {
            accounts,
            total_count: total_count as u64,
            page,
            page_size,
            total_pages: (total_count as f64 / page_size as f64).ceil() as u32,
        })
    }

    // Batch operations for better performance
    pub fn batch_insert_accounts(
        conn: &mut SqliteConnection,
        accounts: Vec<NewAccount>,
    ) -> Result<Vec<Account>, DatabaseError> {
        conn.transaction::<_, DatabaseError, _>(|conn| {
            let inserted_accounts = diesel::insert_into(accounts::table)
                .values(&accounts)
                .returning(accounts::all_columns)
                .get_results(conn)?;

            Ok(inserted_accounts)
        })
    }

    // Optimized search with full-text search
    pub fn search_accounts(
        conn: &mut SqliteConnection,
        environment_id: &str,
        query: &str,
    ) -> Result<Vec<Account>, DatabaseError> {
        let search_pattern = format!("%{}%", query);

        let accounts = accounts::table
            .filter(accounts::environment_id.eq(environment_id))
            .filter(
                accounts::pubkey.like(&search_pattern)
                    .or(accounts::owner.like(&search_pattern))
            )
            .limit(100) // Limit search results
            .load(conn)?;

        Ok(accounts)
    }
}

#[derive(Debug, QueryableByName, Serialize, Deserialize)]
pub struct ProjectWithStats {
    #[diesel(sql_type = "Text")]
    pub id: String,
    #[diesel(sql_type = "Text")]
    pub name: String,
    #[diesel(sql_type = "Nullable<Text>")]
    pub description: Option<String>,
    #[diesel(sql_type = "Json")]
    pub config: serde_json::Value,
    #[diesel(sql_type = "Timestamp")]
    pub created_at: chrono::NaiveDateTime,
    #[diesel(sql_type = "Timestamp")]
    pub updated_at: chrono::NaiveDateTime,
    #[diesel(sql_type = "BigInt")]
    pub environment_count: i64,
    #[diesel(sql_type = "BigInt")]
    pub program_count: i64,
    #[diesel(sql_type = "Nullable<Timestamp>")]
    pub last_activity: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedAccounts {
    pub accounts: Vec<Account>,
    pub total_count: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}
```

## Testing Strategy

### Cross-Platform Database Testing

```rust
// surfdesk-core/src/database/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::path::PathBuf;

    fn create_test_database() -> (Database, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        
        let config = DatabaseConfig {
            platform: PlatformType::Desktop,
            database_url: db_path.to_string_lossy().to_string(),
            max_connections: 5,
            connection_timeout: 10,
            enable_wal: true,
            enable_foreign_keys: true,
        };

        let db = futures::executor::block_on(Database::new(config)).unwrap();
        (db, temp_dir)
    }

    #[tokio::test]
    async fn test_create_project() {
        let (db, _temp_dir) = create_test_database();

        let new_project = NewProject {
            id: "test-project-1".to_string(),
            name: "Test Project".to_string(),
            description: Some("A test project".to_string()),
            config: serde_json::json!({
                "theme": "dark",
                "default_environment": "local"
            }),
        };

        let created_project = db.create_project(new_project).await.unwrap();
        assert_eq!(created_project.name, "Test Project");
        assert_eq!(created_project.description, Some("A test project".to_string()));
    }

    #[tokio::test]
    async fn test_project_crud_operations() {
        let (db, _temp_dir) = create_test_database();

        // Create
        let project = db.create_project(NewProject {
            id: "test-project-2".to_string(),
            name: "Test Project 2".to_string(),
            description: None,
            config: serde_json::json!({}),
        }).await.unwrap();

        // Read
        let projects = db.get_projects().await.unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].id, project.id);

        // Update
        let updated_project = db.update_project(&project.id, UpdateProject {
            name: Some("Updated Test Project".to_string()),
            description: Some("Updated description".to_string()),
            config: None,
        }).await.unwrap();
        assert_eq!(updated_project.name, "Updated Test Project");

        // Delete
        db.delete_project(&project.id).await.unwrap();
        let projects = db.get_projects().await.unwrap();
        assert_eq!(projects.len(), 0);
    }

    #[tokio::test]
    async fn test_foreign_key_constraints() {
        let (db, _temp_dir) = create_test_database();

        // Create project and environment
        let project = db.create_project(NewProject {
            id: "test-project-3".to_string(),
            name: "Test Project".to_string(),
            description: None,
            config: serde_json::json!({}),
        }).await.unwrap();

        let environment = db.create_environment(NewEnvironment {
            id: "test-env-1".to_string(),
            project_id: project.id.clone(),
            name: "Test Environment".to_string(),
            type_: "local".to_string(),
            config: serde_json::json!({}),
            status: "stopped".to_string(),
        }).await.unwrap();

        // Try to delete project with environment (should fail with foreign key constraint)
        let result = db.delete_project(&project.id).await;
        assert!(result.is_err());

        // Delete environment first, then project
        db.delete_environment(&environment.id).await.unwrap();
        db.delete_project(&project.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_transaction_rollback() {
        let (db, _temp_dir) = create_test_database();

        // Create initial project
        let project = db.create_project(NewProject {
            id: "test-project-4".to_string(),
            name: "Test Project".to_string(),
            description: None,
            config: serde_json::json!({}),
        }).await.unwrap();

        // Test transaction rollback on error
        let result = db.transaction::<_, DatabaseError, _>(|conn| {
            // Update project
            diesel::update(projects::table.find(&project.id))
                .set(projects::name.eq("Updated Name"))
                .execute(conn)?;

            // Create environment
            let env = NewEnvironment {
                id: "test-env-rollback".to_string(),
                project_id: project.id.clone(),
                name: "Test Env".to_string(),
                type_: "local".to_string(),
                config: serde_json::json!({}),
                status: "stopped".to_string(),
            };
            diesel::insert_into(environments::table)
                .values(&env)
                .execute(conn)?;

            // Simulate error
            Err(DatabaseError::QueryError(diesel::result::Error::NotFound))

        });

        assert!(result.is_err());

        // Verify rollback - project should not be updated
        let projects = db.get_projects().await.unwrap();
        assert_eq!(projects[0].name, "Test Project");
    }
}
```

## Security & Data Protection

### Database Security Measures

```rust
// surfdesk-core/src/database/security.rs
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};

pub struct DatabaseSecurity {
    encryption_key: Key<Aes256Gcm>,
}

impl DatabaseSecurity {
    pub fn new(password: &str) -> Result<Self, DatabaseError> {
        // Derive encryption key from password using Argon2
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| DatabaseError::SecurityError(e.to_string()))?;

        let key_bytes = password_hash.hash.unwrap().as_bytes();
        let key = Key::from_slice(&key_bytes[..32]); // Use first 32 bytes
        
        Ok(Self {
            encryption_key: *key,
        })
    }

    pub fn encrypt_sensitive_data(&self, data: &[u8]) -> Result<Vec<u8>, DatabaseError> {
        let cipher = Aes256Gcm::new(&self.encryption_key);
        let nonce = Nonce::from_slice(b"unique nonce"); // In production, use random nonce
        
        cipher.encrypt(nonce, data)
            .map_err(|e| DatabaseError::SecurityError(e.to_string()))
    }

    pub fn decrypt_sensitive_data(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, DatabaseError> {
        let cipher = Aes256Gcm::new(&self.encryption_key);
        let nonce = Nonce::from_slice(b"unique nonce");
        
        cipher.decrypt(nonce, encrypted_data)
            .map_err(|e| DatabaseError::SecurityError(e.to_string()))
    }

    pub fn hash_api_key(&self, api_key: &str) -> Result<String, DatabaseError> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        
        let password_hash = argon2.hash_password(api_key.as_bytes(), &salt)
            .map_err(|e| DatabaseError::SecurityError(e.to_string()))?;

        Ok(password_hash.to_string())
    }

    pub fn verify_api_key(&self, api_key: &str, hash: &str) -> Result<bool, DatabaseError> {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| DatabaseError::SecurityError(e.to_string()))?;

        Ok(argon2.verify_password(api_key.as_bytes(), &parsed_hash).is_ok())
    }
}

// Secure database configuration
pub struct SecureDatabaseConfig {
    pub base_config: DatabaseConfig,
    pub encryption_enabled: bool,
    pub access_control_enabled: bool,
    pub audit_log_enabled: bool,
}

impl SecureDatabaseConfig {
    pub fn new_desktop() -> Self {
        Self {
            base_config: DatabaseConfig::default(),
            encryption_enabled: true,
            access_control_enabled: true,
            audit_log_enabled: true,
        }
    }

    pub fn new_web() -> Self {
        Self {
            base_config: DatabaseConfig {
                platform: PlatformType::Web,
                ..Default::default()
            },
            encryption_enabled: true,
            access_control_enabled: false, // Web has different access model
            audit_log_enabled: true,
        }
    }

    pub fn new_terminal() -> Self {
        Self {
            base_config: DatabaseConfig {
                platform: PlatformType::Terminal,
                max_connections: 3,
                ..Default::default()
            },
            encryption_enabled: false, // Terminal often runs in trusted environments
            access_control_enabled: false,
            audit_log_enabled: true,
        }
    }
}
```

## Best Practices & Guidelines

### Database Design Principles

1. **Normalization**: Follow proper database normalization to reduce redundancy
2. **Indexing**: Create appropriate indexes for frequently queried columns
3. **Constraints**: Use foreign key constraints to maintain data integrity
4. **Transactions**: Group related operations in transactions for consistency
5. **Migrations**: Use version-controlled migrations for schema changes

### Cross-Platform Considerations

1. **Performance**: Optimize for each platform's constraints
2. **Storage**: Use platform-appropriate storage mechanisms
3. **Sync**: Implement robust synchronization for web platform
4. **Backup**: Provide backup/restore capabilities for desktop
5. **CLI**: Support command-line operations for terminal

### Security Best Practices

1. **Encryption**: Encrypt sensitive data at rest
2. **Access Control**: Implement proper access controls
3. **Audit Logging**: Log all database operations
4. **Input Validation**: Validate all inputs to prevent SQL injection
5. **Password Security**: Use strong password hashing algorithms

---

**This database architecture provides a robust, cross-platform foundation for SurfDesk's data persistence needs, ensuring consistency, performance, and security across desktop, web, and terminal platforms.**