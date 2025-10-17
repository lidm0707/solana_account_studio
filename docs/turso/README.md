# 🌊 Turso Database Integration Guide

## 📋 OVERVIEW

Turso is a serverless SQLite-compatible database built on libSQL. SurfDesk has transitioned from Diesel to Turso to provide better performance, scalability, and developer experience across all platforms.

## 🚀 ADVANTAGES OF TURSO

### **Performance & Scalability**
```
✅ Serverless Architecture: Auto-scaling without infrastructure management
✅ Edge Computing: Global distribution for low-latency access
✅ SQLite Compatibility: Leverage existing SQLite knowledge and tools
✅ Real-time Replication: Automatic data synchronization
✅ Connection Pooling: Efficient resource management
```

### **Developer Experience**
```
✅ Simple API: Clean, intuitive Rust integration
✅ Migration Support: Built-in schema management
✅ Local Development: SQLite for local, Turso for production
✅ Type Safety: Full Rust type system integration
✅ Async Support: Non-blocking operations throughout
```

## 🏗️ ARCHITECTURE

### **Multi-Platform Strategy**
```
🖥️ Desktop: Local SQLite with optional Turso sync
🌐 Web: IndexedDB with Turso cloud backend
💻 Terminal: Direct Turso connection or local SQLite
📱 Mobile: Future support with embedded libSQL
```

### **Database Backend Selection**
```rust
pub enum DatabaseBackend {
    /// Local SQLite database
    SQLite,
    /// Turso (libSQL) cloud database
    Turso,
}
```

## 🛠️ SETUP & CONFIGURATION

### **Prerequisites**
```bash
# Install Turso CLI
curl -sSfL https://get.tur.so/install.sh | bash

# Authenticate with Turso
turso auth login

# Create database
turso db create surfdesk

# Get database URL and auth token
turso db show surfdesk --url
turso db tokens create surfdesk
```

### **Environment Configuration**
```bash
# .env file
TURSO_URL="libsql://your-db-url.turso.io"
TURSO_AUTH_TOKEN="your-auth-token"
TURSO_DATABASE_NAME="surfdesk"

# Or use Rust environment variables
export TURSO_URL="libsql://your-db-url.turso.io"
export TURSO_AUTH_TOKEN="your-auth-token"
export TURSO_DATABASE_NAME="surfdesk"
```

### **Cargo Dependencies**
```toml
[dependencies]
libsql = "0.5"
libsql-sys = { version = "0.5", features = ["bindgen"] }
tokio = { version = "1.0", features = ["full"] }

[features]
default = ["database"]
database = ["libsql"]
turso = ["libsql"]
```

## 📊 DATABASE SCHEMA

### **Core Tables**
```sql
-- Projects table
CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    owner TEXT NOT NULL,
    settings TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Environments table
CREATE TABLE IF NOT EXISTS environments (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    network TEXT NOT NULL,
    rpc_url TEXT,
    config TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Accounts table
CREATE TABLE IF NOT EXISTS accounts (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    environment_id TEXT NOT NULL,
    pubkey TEXT NOT NULL,
    label TEXT,
    balance REAL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE
);

-- Transactions table
CREATE TABLE IF NOT EXISTS transactions (
    id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL,
    signature TEXT NOT NULL,
    slot BIGINT,
    block_time BIGINT,
    status TEXT NOT NULL,
    amount REAL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
);
```

## 🔧 RUST INTEGRATION

### **Database Configuration**
```rust
use libsql::{Database, Connection};
use std::sync::Arc;

pub struct DatabaseConfig {
    pub database_path: String,
    pub backend: DatabaseBackend,
    pub turso_config: Option<TursoConfig>,
}

pub struct TursoConfig {
    pub url: String,
    pub auth_token: String,
    pub database_name: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        if let (Ok(url), Ok(auth_token), Ok(database_name)) = (
            std::env::var("TURSO_URL"),
            std::env::var("TURSO_AUTH_TOKEN"),
            std::env::var("TURSO_DATABASE_NAME"),
        ) {
            Self::from_turso(url, auth_token, database_name)
        } else {
            Self::from_platform() // Fallback to local SQLite
        }
    }

    pub fn from_turso(url: String, auth_token: String, database_name: String) -> Self {
        Self {
            database_path: url.clone(),
            backend: DatabaseBackend::Turso,
            turso_config: Some(TursoConfig {
                url,
                auth_token,
                database_name,
            }),
            ..Default::default()
        }
    }
}
```

### **Database Service**
```rust
pub struct DatabaseService {
    db: Arc<Database>,
    config: DatabaseConfig,
}

impl DatabaseService {
    pub async fn new(config: DatabaseConfig) -> Result<Self> {
        let db = match config.backend {
            DatabaseBackend::SQLite => {
                Database::builder()
                    .path(&config.database_path)
                    .build()?
            }
            DatabaseBackend::Turso => {
                if let Some(turso_config) = &config.turso_config {
                    Database::builder()
                        .url(&turso_config.url)
                        .auth_token(&turso_config.auth_token)
                        .build()?
                } else {
                    return Err(SurfDeskError::database("Turso configuration missing"));
                }
            }
        };

        let service = Self {
            db: Arc::new(db),
            config,
        };

        service.run_migrations().await?;
        Ok(service)
    }

    pub async fn get_connection(&self) -> Result<Connection> {
        self.db.connect()
            .map_err(|e| SurfDeskError::database(e.to_string()))
    }
}
```

### **CRUD Operations**
```rust
impl DatabaseService {
    // Create project
    pub async fn create_project(&self, project: &Project) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        conn.execute(
            "INSERT INTO projects (id, name, description, owner, settings, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            (
                &project.id,
                &project.name,
                &project.description,
                &project.owner,
                &project.settings,
                &project.created_at,
                &project.updated_at,
            ),
        ).await?;
        
        Ok(())
    }

    // Get projects
    pub async fn get_projects(&self) -> Result<Vec<Project>> {
        let mut conn = self.get_connection().await?;
        let mut rows = conn.query("SELECT * FROM projects ORDER BY created_at DESC", ()).await?;
        
        let mut projects = Vec::new();
        while let Some(row) = rows.next().await? {
            projects.push(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                owner: row.get(3)?,
                settings: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            });
        }
        
        Ok(projects)
    }

    // Update project
    pub async fn update_project(&self, project: &Project) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        conn.execute(
            "UPDATE projects SET name = ?, description = ?, settings = ?, updated_at = ?
             WHERE id = ?",
            (
                &project.name,
                &project.description,
                &project.settings,
                &project.updated_at,
                &project.id,
            ),
        ).await?;
        
        Ok(())
    }

    // Delete project
    pub async fn delete_project(&self, id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        conn.execute("DELETE FROM projects WHERE id = ?", (id,)).await?;
        Ok(())
    }
}
```

## 🔄 MIGRATIONS

### **Migration System**
```rust
pub struct MigrationManager;

impl MigrationManager {
    pub async fn run_migrations(db: &Database) -> Result<()> {
        let mut conn = db.connect()?;
        
        // Create migrations table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS migrations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                version TEXT NOT NULL UNIQUE,
                applied_at TEXT NOT NULL
            )",
            (),
        ).await?;
        
        // Run migrations
        let migrations = vec![
            ("2024_01_01_001_initial_schema", include_str!("migrations/2024_01_01_001_initial_schema.sql")),
            ("2024_01_02_002_add_indexes", include_str!("migrations/2024_01_02_002_add_indexes.sql")),
        ];
        
        for (version, sql) in migrations {
            if !Self::is_migration_applied(&mut conn, version).await? {
                conn.execute(sql, ()).await?;
                conn.execute(
                    "INSERT INTO migrations (version, applied_at) VALUES (?, ?)",
                    (version, &chrono::Utc::now().to_rfc3339()),
                ).await?;
                
                log::info!("Applied migration: {}", version);
            }
        }
        
        Ok(())
    }
    
    async fn is_migration_applied(conn: &mut Connection, version: &str) -> Result<bool> {
        let mut rows = conn.query(
            "SELECT 1 FROM migrations WHERE version = ?",
            (version,),
        ).await?;
        
        Ok(rows.next().await?.is_some())
    }
}
```

## 🌐 SYNC & REPLICATION

### **Local-Cloud Sync**
```rust
pub struct SyncManager {
    local_db: Arc<Database>,
    remote_db: Arc<Database>,
}

impl SyncManager {
    pub async fn sync_to_cloud(&self) -> Result<()> {
        let local_conn = self.local_db.connect()?;
        let remote_conn = self.remote_db.connect()?;
        
        // Get last sync timestamp
        let last_sync = self.get_last_sync_timestamp().await?;
        
        // Sync projects
        self.sync_table(&local_conn, &remote_conn, "projects", last_sync).await?;
        self.sync_table(&local_conn, &remote_conn, "environments", last_sync).await?;
        self.sync_table(&local_conn, &remote_conn, "accounts", last_sync).await?;
        
        // Update sync timestamp
        self.update_sync_timestamp().await?;
        
        Ok(())
    }
    
    async fn sync_table(
        &self,
        local: &Connection,
        remote: &Connection,
        table: &str,
        since: &str,
    ) -> Result<()> {
        // Get local changes
        let local_changes = local.query(
            &format!("SELECT * FROM {} WHERE updated_at > ?", table),
            (since,),
        ).await?;
        
        // Apply to remote
        while let Some(row) = local_changes.next().await? {
            self.apply_row_to_remote(remote, table, row).await?;
        }
        
        // Get remote changes
        let remote_changes = remote.query(
            &format!("SELECT * FROM {} WHERE updated_at > ?", table),
            (since,),
        ).await?;
        
        // Apply to local
        while let Some(row) = remote_changes.next().await? {
            self.apply_row_to_local(local, table, row).await?;
        }
        
        Ok(())
    }
}
```

## 📈 PERFORMANCE OPTIMIZATION

### **Connection Management**
```rust
pub struct ConnectionPool {
    db: Arc<Database>,
    max_connections: usize,
    connections: Arc<Mutex<Vec<Connection>>>,
}

impl ConnectionPool {
    pub async fn get_connection(&self) -> Result<Connection> {
        let mut connections = self.connections.lock().await;
        
        if let Some(conn) = connections.pop() {
            Ok(conn)
        } else {
            self.db.connect()
                .map_err(|e| SurfDeskError::database(e.to_string()))
        }
    }
    
    pub async fn return_connection(&self, conn: Connection) {
        let mut connections = self.connections.lock().await;
        if connections.len() < self.max_connections {
            connections.push(conn);
        }
    }
}
```

### **Bulk Operations**
```rust
impl DatabaseService {
    pub async fn bulk_insert_accounts(&self, accounts: &[Account]) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        // Begin transaction
        let tx = conn.begin().await?;
        
        // Prepare statement
        let stmt = tx.prepare(
            "INSERT INTO accounts (id, project_id, environment_id, pubkey, label, balance, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        ).await?;
        
        // Execute bulk insert
        for account in accounts {
            stmt.execute((
                &account.id,
                &account.project_id,
                &account.environment_id,
                &account.pubkey,
                &account.label,
                &account.balance,
                &account.created_at,
                &account.updated_at,
            )).await?;
        }
        
        // Commit transaction
        tx.commit().await?;
        
        Ok(())
    }
}
```

## 🧪 TESTING

### **Test Database Setup**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    async fn setup_test_db() -> DatabaseService {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        
        let config = DatabaseConfig {
            database_path: db_path.to_string_lossy().to_string(),
            backend: DatabaseBackend::SQLite,
            turso_config: None,
            ..Default::default()
        };
        
        DatabaseService::new(config).await.unwrap()
    }
    
    #[tokio::test]
    async fn test_create_project() {
        let db = setup_test_db().await;
        
        let project = Project {
            id: "test-project".to_string(),
            name: "Test Project".to_string(),
            description: "A test project".to_string(),
            owner: "test-user".to_string(),
            settings: "{}".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };
        
        db.create_project(&project).await.unwrap();
        
        let projects = db.get_projects().await.unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].name, "Test Project");
    }
}
```

## 🔧 DEBUGGING & MONITORING

### **Database Health Check**
```rust
impl DatabaseService {
    pub async fn health_check(&self) -> Result<DatabaseHealth> {
        let mut conn = self.get_connection().await?;
        
        // Test basic connectivity
        let start = std::time::Instant::now();
        let _result: i64 = conn.query_row("SELECT 1", [], |row| row.get(0)).await?;
        let latency = start.elapsed();
        
        // Get table counts
        let project_count: i64 = conn.query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0)).await.unwrap_or(0);
        let environment_count: i64 = conn.query_row("SELECT COUNT(*) FROM environments", [], |row| row.get(0)).await.unwrap_or(0);
        
        Ok(DatabaseHealth {
            status: HealthStatus::Healthy,
            latency,
            project_count,
            environment_count,
            backend: self.config.backend.clone(),
        })
    }
}

#[derive(Debug)]
pub struct DatabaseHealth {
    pub status: HealthStatus,
    pub latency: Duration,
    pub project_count: i64,
    pub environment_count: i64,
    pub backend: DatabaseBackend,
}

#[derive(Debug)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}
```

## 🚀 DEPLOYMENT

### **Production Configuration**
```rust
pub fn create_production_config() -> DatabaseConfig {
    DatabaseConfig::from_turso(
        std::env::var("TURSO_URL").expect("TURSO_URL must be set"),
        std::env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set"),
        std::env::var("TURSO_DATABASE_NAME").expect("TURSO_DATABASE_NAME must be set"),
    )
}

pub fn create_development_config() -> DatabaseConfig {
    let mut config = DatabaseConfig::from_platform();
    
    // Override with Turso if available
    if let (Ok(url), Ok(token), Ok(name)) = (
        std::env::var("TURSO_URL"),
        std::env::var("TURSO_AUTH_TOKEN"),
        std::env::var("TURSO_DATABASE_NAME"),
    ) {
        config = config.with_turso(url, token, name);
    }
    
    config
}
```

## 🔗 RESOURCES

### **Official Documentation**
- [Turso Documentation](https://docs.turso.tech/)
- [libSQL GitHub](https://github.com/libsql/libsql)
- [Turso CLI Reference](https://docs.turso.tech/reference/turso-cli)

### **Rust Integration**
- [libsql Rust Client](https://docs.turso.tech/sdk/libsql/rust)
- [Turso Examples](https://github.com/tursodatabase/examples)
- [Migration Best Practices](https://docs.turso.tech/guides/migrations)

### **Community**
- [Turso Discord](https://discord.gg/turso)
- [GitHub Discussions](https://github.com/tursodatabase/libsql/discussions)
- [Stack Overflow](https://stackoverflow.com/questions/tagged/turso)

---

**This guide covers the complete integration of Turso database with SurfDesk. For more detailed information, refer to the official Turso documentation and the examples in this repository.**

🌊 **Happy coding with Turso!**