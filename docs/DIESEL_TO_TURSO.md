# 🔄 Diesel to Turso Migration Guide

## 📋 Overview

This document provides a comprehensive guide for migrating SurfDesk from Diesel to Turso (libSQL). This migration improves performance, simplifies the codebase, and enables better cross-platform compatibility.

## 🎯 Migration Goals

- **Simplify Dependencies**: Remove complex Diesel setup and macros
- **Improve Performance**: Leverage Turso's serverless architecture
- **Better Async Support**: Native async/await throughout the stack
- **Cross-Platform**: Unified database experience across all platforms
- **Reduced Complexity**: Eliminate schema.rs and complex migrations

## 🏗️ Architecture Changes

### Before (Diesel)
```
├── diesel (2.1)
├── diesel_migrations (2.1)
├── schema.rs (auto-generated)
├── migrations/ (embedded migrations)
└── Complex query builders
```

### After (Turso)
```
├── libsql (0.5)
├── libsql-sys (0.5)
├── SQL literals (no schema generation)
├── Inline migrations
└── Direct SQL execution
```

## 📦 Dependency Changes

### Remove Diesel Dependencies
```toml
# Remove from workspace dependencies
diesel = { version = "2.1", features = ["sqlite", "r2d2", "chrono"] }
diesel_migrations = { version = "2.1" }

# Remove from surfdesk-core dependencies
diesel = { workspace = true, optional = true }
diesel_migrations = { workspace = true, optional = true }
```

### Add Turso Dependencies
```toml
# Add to workspace dependencies
libsql = { version = "0.5" }
libsql-sys = { version = "0.5", features = ["bindgen"] }

# Add to surfdesk-core dependencies
libsql = { workspace = true, optional = true }
```

### Update Features
```toml
[features]
default = ["database", "surfpool"]
database = ["libsql"]
turso = ["libsql"]
diesel = ["dep:diesel", "dep:diesel_migrations"]  # Legacy support
```

## 🔧 Code Changes

### 1. Database Connection

**Before (Diesel):**
```rust
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;

pub struct DatabaseService {
    pool: Arc<Pool<ConnectionManager<SqliteConnection>>>,
    config: DatabaseConfig,
}

impl DatabaseService {
    pub async fn new(config: DatabaseConfig) -> Result<Self> {
        let manager = ConnectionManager::<SqliteConnection>::new(&config.database_path);
        let pool = Pool::builder()
            .max_size(config.pool_size)
            .build(manager)
            .map_err(|e| SurfDeskError::database(e.to_string()))?;
            
        Ok(Self {
            pool: Arc::new(pool),
            config,
        })
    }
    
    pub async fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>> {
        self.pool.get()
            .map_err(|e| SurfDeskError::database(e.to_string()))
    }
}
```

**After (Turso):**
```rust
use libsql::{Database, Connection};

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
                let turso_config = config.turso_config.as_ref()
                    .ok_or_else(|| SurfDeskError::database("Turso config missing"))?;
                Database::builder()
                    .url(&turso_config.url)
                    .auth_token(&turso_config.auth_token)
                    .build()?
            }
        };
        
        Ok(Self {
            db: Arc::new(db),
            config,
        })
    }
    
    pub async fn get_connection(&self) -> Result<Connection> {
        self.db.connect()
            .map_err(|e| SurfDeskError::database(e.to_string()))
    }
}
```

### 2. Query Operations

**Before (Diesel):**
```rust
use diesel::prelude::*;
use crate::schema::projects;

pub async fn get_projects(&self) -> Result<Vec<Project>> {
    let mut conn = self.get_connection().await?;
    
    let results = projects::table
        .order(projects::created_at.desc())
        .load::<Project>(&mut conn)?;
        
    Ok(results)
}

pub async fn create_project(&self, project: &NewProject) -> Result<Project> {
    let mut conn = self.get_connection().await?;
    
    diesel::insert_into(projects::table)
        .values(project)
        .execute(&mut conn)?;
        
    let created = projects::table
        .order(projects::created_at.desc())
        .first(&mut conn)?;
        
    Ok(created)
}
```

**After (Turso):**
```rust
pub async fn get_projects(&self) -> Result<Vec<Project>> {
    let mut conn = self.get_connection().await?;
    let mut rows = conn.query(
        "SELECT * FROM projects ORDER BY created_at DESC",
        ()
    ).await?;
    
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
```

### 3. Model Changes

**Before (Diesel):**
```rust
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub owner: String,
    pub settings: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = projects)]
pub struct NewProject {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub owner: String,
    pub settings: String,
    pub created_at: String,
    pub updated_at: String,
}
```

**After (Turso):**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub owner: String,
    pub settings: String,
    pub created_at: String,
    pub updated_at: String,
}

// No separate NewProject struct needed - use Project directly
impl Project {
    pub fn new(
        id: String,
        name: String,
        description: Option<String>,
        owner: String,
        settings: String,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id,
            name,
            description,
            owner,
            settings,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}
```

### 4. Schema Removal

**Remove these files:**
- `src/database/schema.rs` (auto-generated by Diesel)
- `diesel.toml` (Diesel configuration)
- `migrations/` directory (Diesel migrations)

**Replace with inline migrations:**
```rust
impl DatabaseService {
    async fn run_migrations(&self) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        // Create migrations table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS migrations (
                version TEXT PRIMARY KEY,
                applied_at TEXT NOT NULL
            )",
            (),
        ).await?;
        
        // Run migrations
        let migrations = vec![
            ("2024_01_01_001_initial_schema", include_str!("migrations/2024_01_01_001.sql")),
            ("2024_01_02_002_add_indexes", include_str!("migrations/2024_01_02_002.sql")),
        ];
        
        for (version, sql) in migrations {
            let mut rows = conn.query(
                "SELECT 1 FROM migrations WHERE version = ?",
                (version,),
            ).await?;
            
            if rows.next().await?.is_none() {
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
}
```

## 🔄 Migration Steps

### Step 1: Update Dependencies
```bash
# Update workspace Cargo.toml
vim Cargo.toml

# Update surfdesk-core Cargo.toml  
vim surfdesk-core/Cargo.toml

# Remove Diesel files
rm -f surfdesk-core/src/database/schema.rs
rm -f diesel.toml
rm -rf surfdesk-core/migrations/
```

### Step 2: Update Database Service
```bash
# Replace database.rs implementation
cp surfdesk-core/src/services/database.rs surfdesk-core/src/services/database.rs.backup
# Apply new Turso implementation
```

### Step 3: Update Models
```bash
# Remove Diesel derives from all model structs
find surfdesk-core/src -name "*.rs" -exec sed -i '/Queryable\|Insertable\|#\[diesel/d' {} \;
```

### Step 4: Update Queries
```bash
# Replace Diesel queries with libsql queries
# This needs to be done manually for each query
```

### Step 5: Update Imports
```bash
# Remove Diesel imports
find surfdesk-core/src -name "*.rs" -exec sed -i '/use diesel/d' {} \;

# Add libsql imports where needed
```

### Step 6: Test Migration
```bash
# Run tests to ensure everything works
cargo test --workspace

# Check compilation
cargo check --workspace
```

## 🧪 Testing the Migration

### 1. Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_database_connection() {
        let config = DatabaseConfig::from_platform();
        let db = DatabaseService::new(config).await.unwrap();
        
        // Test basic connectivity
        let mut conn = db.get_connection().await.unwrap();
        let result: i64 = conn.query_row("SELECT 1", [], |row| row.get(0)).await.unwrap();
        assert_eq!(result, 1);
    }
    
    #[tokio::test]
    async fn test_project_crud() {
        let config = DatabaseConfig::from_platform();
        let db = DatabaseService::new(config).await.unwrap();
        
        // Create project
        let project = Project::new(
            "test-id".to_string(),
            "Test Project".to_string(),
            Some("Test Description".to_string()),
            "test-owner".to_string(),
            "{}".to_string(),
        );
        
        db.create_project(&project).await.unwrap();
        
        // Read project
        let projects = db.get_projects().await.unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].name, "Test Project");
        
        // Update project
        let mut updated = projects[0].clone();
        updated.name = "Updated Project".to_string();
        db.update_project(&updated).await.unwrap();
        
        // Delete project
        db.delete_project(&project.id).await.unwrap();
        let projects = db.get_projects().await.unwrap();
        assert_eq!(projects.len(), 0);
    }
}
```

### 2. Integration Tests
```rust
#[tokio::test]
async fn test_turso_integration() {
    // Test with actual Turso database if credentials are available
    if let (Ok(url), Ok(token), Ok(db_name)) = (
        std::env::var("TURSO_URL"),
        std::env::var("TURSO_AUTH_TOKEN"),
        std::env::var("TURSO_DATABASE_NAME"),
    ) {
        let config = DatabaseConfig::from_turso(url, token, db_name);
        let db = DatabaseService::new(config).await.unwrap();
        
        // Test operations
        test_project_crud_with_db(&db).await;
    }
}
```

## 🚨 Common Issues

### 1. Async/Await Mismatch
**Problem:** Diesel uses sync operations, Turso uses async.
**Solution:** Ensure all database operations are async and use `.await`.

### 2. Query Builder vs Raw SQL
**Problem:** Diesel provides type-safe query builders, Turso uses raw SQL.
**Solution:** Write careful SQL queries and use parameter binding.

### 3. Connection Pooling
**Problem:** Diesel uses r2d2 connection pooling, Turso has different approach.
**Solution:** Implement custom connection pooling if needed, or use single connection.

### 4. Error Handling
**Problem:** Diesel and libsql have different error types.
**Solution:** Update error handling to use libsql error types.

### 5. Schema Generation
**Problem:** No more auto-generated schema.
**Solution:** Manually maintain SQL schemas in migration files.

## 📈 Performance Benefits

### Before (Diesel)
- Connection pooling overhead
- Query builder compilation cost
- Complex macro expansion
- Sync operations blocking

### After (Turso)
- Direct SQL execution
- Native async support
- Simpler connection management
- Serverless scalability

## 🎯 Benefits Summary

### ✅ What We Gained
- **Simpler Dependencies**: No complex Diesel setup
- **Better Performance**: Direct SQL execution
- **Async Native**: Full async/await support
- **Cross-Platform**: Unified database experience
- **Serverless Ready**: Easy cloud deployment
- **Reduced Complexity**: No schema generation

### ⚠️ What We Lost
- **Compile-Time Query Validation**: SQL errors found at runtime
- **Type-Safe Query Builders**: Manual SQL writing required
- **Auto-Generated Schema**: Manual schema maintenance

### 🔄 Migration Trade-offs
The migration from Diesel to Turso represents a shift from compile-time safety to runtime flexibility. While we lose some type safety, we gain significant performance improvements and simplified architecture.

## 🔗 Resources

- [Turso Documentation](https://docs.turso.tech/)
- [libSQL Rust Client](https://docs.turso.tech/sdk/libsql/rust)
- [Migration Best Practices](https://docs.turso.tech/guides/migrations)
- [Turso vs SQLite Comparison](https://docs.turso.tech/overview/why-turso)

---

**This migration guide provides the complete roadmap for transitioning SurfDesk from Diesel to Turso. Follow these steps carefully to ensure a smooth migration process.**

🌊 **Happy migrating!**