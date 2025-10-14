# Database Schema & Data Architecture - SurfDesk

## Overview

This document defines the database schema and data architecture for the SurfDesk application. The database design supports local development environments, project management, transaction history, account state tracking, and AI-assisted testing workflows.

## Data Architecture Principles

### 1. **Local-First Design**
- All data stored locally using SQLite for portability
- Cloud sync as optional feature (future enhancement)
- Full offline functionality
- Atomic transactions for data consistency

### 2. **Time-Series Support**
- Historical tracking of all state changes
- Account state versioning
- Transaction history with full context
- Performance metrics and analytics

### 3. **Extensible Schema**
- Plugin-compatible data structures
- Flexible configuration storage
- Support for custom program metadata
- Upgrade-safe schema migrations

## Database Schema

### Core Tables

#### 1. Projects
```sql
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    workspace_path TEXT NOT NULL,
    is_active BOOLEAN DEFAULT FALSE,
    metadata JSON, -- Additional project settings
    config JSON -- Project-specific configuration
);

CREATE INDEX idx_projects_name ON projects(name);
CREATE INDEX idx_projects_active ON projects(is_active);
```

#### 2. Environments
```sql
CREATE TABLE environments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    type TEXT NOT NULL CHECK (type IN ('local', 'fork', 'testnet', 'devnet')),
    network_endpoint TEXT,
    fork_network TEXT, -- 'mainnet-beta', 'devnet', etc.
    fork_slot INTEGER,
    port_range_start INTEGER,
    port_range_end INTEGER,
    auto_start BOOLEAN DEFAULT FALSE,
    status TEXT DEFAULT 'stopped' CHECK (status IN ('starting', 'running', 'stopping', 'stopped', 'error')),
    config JSON, -- Environment-specific settings
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);
```

#### 3. Programs
```sql
CREATE TABLE programs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    program_id TEXT, -- Solana program public key
    program_type TEXT CHECK (type IN ('anchor', 'native', 'sealevel')),
    source_path TEXT NOT NULL,
    build_path TEXT,
    idl_path TEXT,
    binary_path TEXT,
    version TEXT,
    build_status TEXT DEFAULT 'not_built' CHECK (build_status IN ('not_built', 'building', 'built', 'failed', 'outdated')),
    deployed_at DATETIME,
    deployment_slot INTEGER,
    metadata JSON, -- Program metadata, features, etc.
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    UNIQUE(project_id, name)
);

CREATE INDEX idx_programs_project_id ON programs(project_id);
CREATE INDEX idx_programs_program_id ON programs(program_id);
```

#### 4. Program_IDLs
```sql
CREATE TABLE program_idls (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    program_id INTEGER NOT NULL,
    version TEXT NOT NULL,
    idl_content TEXT NOT NULL, -- JSON IDL content
    is_active BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (program_id) REFERENCES programs(id) ON DELETE CASCADE,
    UNIQUE(program_id, version)
);
```

#### 5. Accounts
```sql
CREATE TABLE accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    environment_id INTEGER NOT NULL,
    address TEXT NOT NULL,
    owner_program_id INTEGER, -- References programs.id
    account_type TEXT, -- 'system', 'program', 'token', 'custom'
    lamports INTEGER DEFAULT 0,
    data BLOB, -- Account data in bytes
    data_schema JSON, -- Schema for parsing account data
    is_frozen BOOLEAN DEFAULT FALSE,
    slot_updated INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE,
    FOREIGN KEY (owner_program_id) REFERENCES programs(id) ON DELETE SET NULL,
    UNIQUE(project_id, environment_id, address)
);

CREATE INDEX idx_accounts_address ON accounts(address);
CREATE INDEX idx_accounts_owner ON accounts(owner_program_id);
CREATE INDEX idx_accounts_type ON accounts(account_type);
```

#### 6. Account_State_History
```sql
CREATE TABLE account_state_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,
    slot INTEGER NOT NULL,
    lamports INTEGER NOT NULL,
    data BLOB,
    changed_fields JSON, -- Diff of what changed
    transaction_signature TEXT,
    operation_type TEXT CHECK (operation_type IN ('create', 'update', 'delete', 'transfer')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
);

CREATE INDEX idx_account_history_account_id ON account_state_history(account_id);
CREATE INDEX idx_account_history_slot ON account_state_history(slot);
```

#### 7. Transactions
```sql
CREATE TABLE transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    environment_id INTEGER NOT NULL,
    signature TEXT NOT NULL UNIQUE,
    slot INTEGER,
    block_time DATETIME,
    status TEXT DEFAULT 'pending' CHECK (status IN ('pending', 'confirmed', 'finalized', 'failed')),
    error_message TEXT,
    fee_lamports INTEGER DEFAULT 0,
    compute_units_consumed INTEGER,
    recent_blockhash TEXT,
    transaction_data BLOB, -- Serialized transaction
    instructions JSON, -- Array of instruction details
    signers JSON, -- Array of signer public keys
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE
);

CREATE INDEX idx_transactions_signature ON transactions(signature);
CREATE INDEX idx_transactions_status ON transactions(status);
CREATE INDEX idx_transactions_slot ON transactions(slot);
CREATE INDEX idx_transactions_created ON transactions(created_at);
```

#### 8. Test_Plans
```sql
CREATE TABLE test_plans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    plan_type TEXT CHECK (plan_type IN ('manual', 'ai_generated', 'template')),
    goal TEXT, -- Natural language test goal
    schema_version TEXT DEFAULT '1.0',
    plan_content JSON NOT NULL, -- Structured test plan JSON
    status TEXT DEFAULT 'draft' CHECK (status IN ('draft', 'ready', 'running', 'completed', 'failed')),
    created_by TEXT, -- 'user' or 'ai'
    ai_prompt TEXT, -- Original AI prompt if applicable
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE INDEX idx_test_plans_project_id ON test_plans(project_id);
CREATE INDEX idx_test_plans_status ON test_plans(status);
```

#### 9. Test_Executions
```sql
CREATE TABLE test_executions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    test_plan_id INTEGER NOT NULL,
    environment_id INTEGER NOT NULL,
    execution_id TEXT NOT NULL UNIQUE,
    status TEXT DEFAULT 'running' CHECK (status IN ('pending', 'running', 'completed', 'failed', 'cancelled')),
    started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    completed_at DATETIME,
    total_steps INTEGER DEFAULT 0,
    completed_steps INTEGER DEFAULT 0,
    results JSON, -- Execution results summary
    logs TEXT, -- Full execution logs
    metrics JSON, -- Performance metrics
    FOREIGN KEY (test_plan_id) REFERENCES test_plans(id) ON DELETE CASCADE,
    FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE
);

CREATE INDEX idx_test_executions_plan_id ON test_executions(test_plan_id);
CREATE INDEX idx_test_executions_status ON test_executions(status);
```

#### 10. Test_Steps
```sql
CREATE TABLE test_steps (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    execution_id INTEGER NOT NULL,
    step_order INTEGER NOT NULL,
    step_type TEXT CHECK (step_type IN ('instruction', 'assertion', 'time_travel', 'account_override', 'wait')),
    step_data JSON NOT NULL, -- Step-specific data
    status TEXT DEFAULT 'pending' CHECK (status IN ('pending', 'running', 'completed', 'failed', 'skipped')),
    result JSON, -- Step execution result
    error_message TEXT,
    execution_time_ms INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    completed_at DATETIME,
    FOREIGN KEY (execution_id) REFERENCES test_executions(id) ON DELETE CASCADE
);

CREATE INDEX idx_test_steps_execution_id ON test_steps(execution_id);
CREATE INDEX idx_test_steps_status ON test_steps(status);
```

### Configuration & Metadata Tables

#### 11. Wallets
```sql
CREATE TABLE wallets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    wallet_type TEXT CHECK (wallet_type IN ('keypair', 'ledger', 'derived')),
    public_key TEXT NOT NULL UNIQUE,
    private_key_encrypted BLOB, -- Encrypted private key
    derivation_path TEXT, -- For derived wallets
    balance_lamports INTEGER DEFAULT 0,
    is_default BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    UNIQUE(project_id, name)
);
```

#### 12. Settings
```sql
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value JSON NOT NULL,
    value_type TEXT CHECK (value_type IN ('string', 'number', 'boolean', 'object', 'array')),
    description TEXT,
    is_user_setting BOOLEAN DEFAULT FALSE,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

#### 13. Logs
```sql
CREATE TABLE logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    level TEXT CHECK (level IN ('debug', 'info', 'warn', 'error')) NOT NULL,
    component TEXT NOT NULL, -- 'surfpool', 'rpc', 'ui', 'mcp', etc.
    message TEXT NOT NULL,
    details JSON, -- Additional structured data
    source TEXT, -- Source file/function
    line_number INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_logs_level ON logs(level);
CREATE INDEX idx_logs_component ON logs(component);
CREATE INDEX idx_logs_created_at ON logs(created_at);
```

## Data Types & Schemas

### Program IDL Schema
```json
{
  "version": "1.0",
  "name": "program_name",
  "instructions": [
    {
      "name": "instruction_name",
      "accounts": [
        {
          "name": "account_name",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "arg_name",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "account_name",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "field_name",
            "type": "u64"
          }
        ]
      }
    }
  ]
}
```

### Test Plan Schema
```json
{
  "version": "1.0",
  "name": "Test Plan Name",
  "description": "Test description",
  "setup": {
    "environment": "devnet",
    "accounts": [
      {
        "address": "account_address",
        "initial_lamports": 1000000000,
        "data": "base64_encoded_data"
      }
    ]
  },
  "steps": [
    {
      "type": "instruction",
      "program_id": "program_address",
      "instruction": "instruction_name",
      "accounts": {
        "account_name": "account_address"
      },
      "args": {
        "arg_name": "value"
      },
      "expected_result": {
        "status": "success",
        "account_changes": [
          {
            "address": "account_address",
            "field": "field_name",
            "expected_value": "expected_value"
          }
        ]
      }
    }
  ],
  "cleanup": {
    "reset_accounts": ["account_address"],
    "close_transactions": ["transaction_signature"]
  }
}
```

### Account Data Schema
```json
{
  "schema_type": "anchor",
  "account_type": "MyAccount",
  "fields": [
    {
      "name": "field1",
      "type": "u64",
      "offset": 0,
      "size": 8
    },
    {
      "name": "field2", 
      "type": "Pubkey",
      "offset": 8,
      "size": 32
    }
  ]
}
```

## Database Operations

### Connection Management
```rust
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_path: &str) -> Result<Self, DatabaseError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect(&format!("sqlite:{}", database_path))
            .await?;
            
        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        Ok(Database { pool })
    }
    
    pub async fn transaction<F, R>(&self, f: F) -> Result<R, DatabaseError>
    where
        F: FnOnce(&mut Transaction<Sqlite>) -> BoxFuture<'_, Result<R, DatabaseError>>,
    {
        let mut tx = self.pool.begin().await?;
        let result = f(&mut tx).await?;
        tx.commit().await?;
        Ok(result)
    }
}
```

### Query Patterns
```rust
// Project Management
impl Database {
    pub async fn create_project(&self, project: &NewProject) -> Result<i64, DatabaseError> {
        let result = sqlx::query!(
            "INSERT INTO projects (name, description, workspace_path) VALUES (?, ?, ?)",
            project.name,
            project.description,
            project.workspace_path
        )
        .execute(&self.pool)
        .await?;
        
        Ok(result.last_insert_rowid())
    }
    
    pub async fn get_active_project(&self) -> Result<Option<Project>, DatabaseError> {
        sqlx::query_as!(
            Project,
            "SELECT * FROM projects WHERE is_active = TRUE LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await
    }
}

// Account Management
impl Database {
    pub async fn upsert_account(
        &self,
        account: &Account,
    ) -> Result<(), DatabaseError> {
        self.transaction(|tx| Box::pin(async move {
            sqlx::query!(
                r#"
                INSERT OR REPLACE INTO accounts 
                (project_id, environment_id, address, owner_program_id, 
                 lamports, data, slot_updated)
                VALUES (?, ?, ?, ?, ?, ?, ?)
                "#,
                account.project_id,
                account.environment_id,
                account.address,
                account.owner_program_id,
                account.lamports,
                account.data,
                account.slot_updated
            )
            .execute(&mut *tx)
            .await?;
            
            // Create history entry
            sqlx::query!(
                r#"
                INSERT INTO account_state_history 
                (account_id, slot, lamports, data, transaction_signature, operation_type)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
                // Get account_id from previous INSERT
                account.slot_updated,
                account.lamports,
                account.data,
                account.transaction_signature,
                account.operation_type
            )
            .execute(&mut *tx)
            .await?;
            
            Ok(())
        }))
        .await
    }
}
```

## Performance Optimizations

### Indexes
- Primary indexes on frequently queried columns
- Composite indexes for common query patterns
- Partial indexes for filtered queries

### Data Partitioning
```sql
-- Time-based partitioning for large tables (future enhancement)
CREATE TABLE logs_2025_01 PARTITION OF logs
FOR VALUES FROM ('2025-01-01') TO ('2025-02-01');

CREATE TABLE logs_2025_02 PARTITION OF logs  
FOR VALUES FROM ('2025-02-01') TO ('2025-03-01');
```

### Caching Strategy
- In-memory cache for frequently accessed accounts
- LRU cache for transaction metadata
- Session cache for environment status

## Data Retention & Cleanup

### Automatic Cleanup
```sql
-- Delete old logs (keep last 30 days)
DELETE FROM logs WHERE created_at < datetime('now', '-30 days');

-- Delete old test execution logs (keep last 7 days)
DELETE FROM test_executions 
WHERE completed_at < datetime('now', '-7 days') 
AND status IN ('completed', 'failed');

-- Clean up account history (keep last 1000 versions per account)
DELETE FROM account_state_history 
WHERE id NOT IN (
    SELECT id FROM account_state_history 
    WHERE account_id = ? 
    ORDER BY slot DESC 
    LIMIT 1000
);
```

### Data Export & Import
```rust
impl Database {
    pub async fn export_project(&self, project_id: i64) -> Result<ProjectExport, DatabaseError> {
        // Export all project data in structured format
        todo!()
    }
    
    pub async fn import_project(&self, export: ProjectExport) -> Result<i64, DatabaseError> {
        // Import project data with validation
        todo!()
    }
}
```

## Backup & Recovery

### Automated Backups
```rust
pub struct BackupConfig {
    pub interval: Duration,
    pub max_backups: usize,
    pub compression: bool,
    pub encryption_key: Option<String>,
}

impl Database {
    pub async fn create_backup(&self, path: &str) -> Result<(), DatabaseError> {
        // Create full database backup
        todo!()
    }
    
    pub async fn restore_backup(&self, path: &str) -> Result<(), DatabaseError> {
        // Restore from backup with validation
        todo!()
    }
}
```

## Schema Migrations

### Migration Strategy
```rust
// migrations/V1__initial_schema.sql
-- Initial schema as defined above

// migrations/V2__add_ai_features.sql  
ALTER TABLE test_plans ADD COLUMN ai_prompt TEXT;
ALTER TABLE test_plans ADD COLUMN created_by TEXT;

// migrations/V3__account_data_compression.sql
ALTER TABLE accounts ADD COLUMN data_compressed BOOLEAN DEFAULT FALSE;
ALTER TABLE account_state_history ADD COLUMN data_compressed BOOLEAN DEFAULT FALSE;
```

### Migration Runner
```rust
#[async_trait]
pub trait Migration {
    fn version(&self) -> u32;
    fn description(&self) -> &str;
    async fn up(&self, pool: &SqlitePool) -> Result<(), DatabaseError>;
    async fn down(&self, pool: &SqlitePool) -> Result<(), DatabaseError>;
}
```

## Security Considerations

### Data Encryption
- Private keys encrypted at rest
- Sensitive configuration values encrypted
- Database file can be encrypted with password

### Access Control
```sql
-- Role-based access (future enhancement)
CREATE TABLE user_permissions (
    user_id TEXT,
    resource_type TEXT,
    resource_id TEXT,
    permission TEXT CHECK (permission IN ('read', 'write', 'admin')),
    granted_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### Data Integrity
- Foreign key constraints enabled
- CHECK constraints for data validation
- Transaction atomicity for related operations

## Monitoring & Analytics

### Database Metrics
```sql
-- Performance monitoring queries
SELECT 
    name,
    sql,
    stat1 as 'cache_hit',
    stat2 as 'cache_miss'
FROM sqlite_master 
JOIN pragma_stats WHERE name NOT LIKE 'sqlite_%';

-- Table sizes
SELECT 
    name,
    COUNT(*) as row_count,
    ROUND(SUM(LENGTH(data)) / 1024.0 / 1024.0, 2) as size_mb
FROM pragma_table_info() 
LEFT JOIN accounts ON 1=1
GROUP BY name;
```

### Health Checks
```rust
impl Database {
    pub async fn health_check(&self) -> Result<DatabaseHealth, DatabaseError> {
        Ok(DatabaseHealth {
            connection_count: self.pool.size(),
            is_writable: self.test_write().await.is_ok(),
            total_size: self.get_database_size().await?,
            last_backup: self.get_last_backup_time().await?,
        })
    }
}
```

---

**Version**: 1.0  
**Created**: 2025-06-18  
**Database Engine**: SQLite  
**Schema Version**: 1  
**Migration Framework**: sqlx-cli  

This schema provides a comprehensive foundation for the SurfDesk application, supporting all the features outlined in the PLAN.md while maintaining performance, security, and extensibility requirements.