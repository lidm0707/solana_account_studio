//! Mock data generators for SurfDesk testing
//!
//! This module provides mock data generators and fixtures for testing various components
//! and services in the SurfDesk application. It includes mock Solana accounts, transactions,
//! environments, and other test data structures.

use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use surfdesk_core::types::*;

/// Mock Solana account generator
pub struct MockAccount;

impl MockAccount {
    /// Create a mock Solana account with default values
    pub fn default() -> Account {
        Account {
            pubkey: Self::pubkey(),
            lamports: 1000000000, // 1 SOL
            data: vec![0u8; 1024],
            owner: Self::owner_program_id(),
            executable: false,
            rent_epoch: 100,
            // Mock additional fields based on your actual Account struct
            label: Some("Mock Account".to_string()),
            account_type: "system".to_string(),
            is_watched: true,
        }
    }

    /// Create a mock system account
    pub fn system() -> Account {
        Account {
            pubkey: Self::system_program_id(),
            lamports: 0,
            data: vec![],
            owner: Self::system_program_id(),
            executable: true,
            rent_epoch: 0,
            label: Some("System Program".to_string()),
            account_type: "program".to_string(),
            is_watched: false,
        }
    }

    /// Create a mock token account
    pub fn token() -> Account {
        Account {
            pubkey: Self::token_account_pubkey(),
            lamports: 500000000,  // 0.5 SOL
            data: vec![0u8; 165], // Token account data size
            owner: Self::token_program_id(),
            executable: false,
            rent_epoch: 150,
            label: Some("Token Account".to_string()),
            account_type: "token".to_string(),
            is_watched: true,
        }
    }

    /// Create a mock program account
    pub fn program() -> Account {
        Account {
            pubkey: Self::mock_program_id(),
            lamports: 1000000000,
            data: vec![1, 2, 3, 4], // Mock program data
            owner: Self::bpf_loader_program_id(),
            executable: true,
            rent_epoch: 200,
            label: Some("Mock Program".to_string()),
            account_type: "program".to_string(),
            is_watched: false,
        }
    }

    /// Generate a test public key
    pub fn pubkey() -> Pubkey {
        "11111111111111111111111111111112".parse().unwrap()
    }

    /// Generate another test public key
    pub fn pubkey_alt() -> Pubkey {
        "22222222222222222222222222222223".parse().unwrap()
    }

    pub fn system_program_id() -> Pubkey {
        "11111111111111111111111111111111".parse().unwrap()
    }

    pub fn token_program_id() -> Pubkey {
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            .parse()
            .unwrap()
    }

    pub fn token_account_pubkey() -> Pubkey {
        "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
            .parse()
            .unwrap()
    }

    pub fn owner_program_id() -> Pubkey {
        "11111111111111111111111111111111".parse().unwrap()
    }

    pub fn mock_program_id() -> Pubkey {
        "Abcdef123456789Abcdef123456789Abcdef123456"
            .parse()
            .unwrap()
    }

    pub fn bpf_loader_program_id() -> Pubkey {
        "BPFLoader1111111111111111111111111111111111"
            .parse()
            .unwrap()
    }
}

/// Mock transaction generator
pub struct MockTransaction;

impl MockTransaction {
    /// Create a mock confirmed transaction
    pub fn confirmed() -> Transaction {
        Transaction {
            signature: Self::signature(),
            slot: Some(123456789),
            block_time: Some(1640995200), // Jan 1, 2022
            ConfirmationStatus: Confirmed,
            err: None,
            meta: Some(Self::transaction_meta()),
            message: Self::transaction_message(),
            // Add other fields based on your Transaction struct
            fee: Some(5000),
            compute_units_consumed: Some(150000),
        }
    }

    /// Create a mock pending transaction
    pub fn pending() -> Transaction {
        Transaction {
            signature: Self::signature_pending(),
            slot: None,
            block_time: None,
            ConfirmationStatus: Pending,
            err: None,
            meta: None,
            message: Self::simple_transaction_message(),
            fee: None,
            compute_units_consumed: None,
        }
    }

    /// Create a mock failed transaction
    pub fn failed() -> Transaction {
        Transaction {
            signature: Self::signature_failed(),
            slot: Some(123456790),
            block_time: Some(1640995300),
            ConfirmationStatus: Failed,
            err: Some("Insufficient funds".to_string()),
            meta: Some(Self::failed_transaction_meta()),
            message: Self::transaction_message(),
            fee: Some(5000),
            compute_units_consumed: Some(200000),
        }
    }

    pub fn signature() -> String {
        "5j7s8X9Be1FQC2sV5V4qQkAyQh8q1LJj8X1q2V4w5X6Y7Z8A9B0C1D2E3F4G5H6".to_string()
    }

    pub fn signature_pending() -> String {
        "4i6r7W8Cd0EQB1rU4U3pPjZxG7p0KIKi7W0p1U3v4W5X6Y7Z8A9B0C1D2E3F4G5".to_string()
    }

    pub fn signature_failed() -> String {
        "3h5q6V7Be9DRB0qT3T2oOiYwF8q1JLJh9Z0p2U3x3W4V5X6Y7Z8A9B0C1D2E3F4".to_string()
    }

    fn transaction_meta() -> TransactionMeta {
        TransactionMeta {
            err: None,
            fee: 5000,
            pre_balances: vec![1000000000, 500000000],
            post_balances: vec![999995000, 500000000],
            inner_instructions: vec![],
            log_messages: vec!["Program 11111111111111111111111111111111 invoke [1]".to_string()],
            pre_token_balances: vec![],
            post_token_balances: vec![],
            rewards: vec![],
            loaded_addresses: LoadedAddresses {
                writable: vec![],
                readonly: vec![],
            },
        }
    }

    fn failed_transaction_meta() -> TransactionMeta {
        TransactionMeta {
            err: Some("Insufficient funds for transfer".to_string()),
            fee: 5000,
            pre_balances: vec![1000000],
            post_balances: vec![1000000],
            inner_instructions: vec![],
            log_messages: vec![],
            pre_token_balances: vec![],
            post_token_balances: vec![],
            rewards: vec![],
            loaded_addresses: LoadedAddresses {
                writable: vec![],
                readonly: vec![],
            },
        }
    }

    fn transaction_message() -> TransactionMessage {
        TransactionMessage {
            account_keys: vec![MockAccount::pubkey(), MockAccount::pubkey_alt()],
            recent_blockhash: "ETh8MnqqsJ6zvV9JqtKJm5eB1dzZqTjzrXyJzP2vW8Z".to_string(),
            instructions: vec![CompiledInstruction::new(0, vec![], vec![0, 1])],
        }
    }

    fn simple_transaction_message() -> TransactionMessage {
        TransactionMessage {
            account_keys: vec![MockAccount::pubkey()],
            recent_blockhash: "FTh9NprqK7zW0J0rKLn6fC2eA1arTlAzK1wA3qX9Y9A".to_string(),
            instructions: vec![CompiledInstruction::new(0, vec![], vec![0])],
        }
    }
}

/// Mock environment generator
pub struct MockEnvironment;

impl MockEnvironment {
    /// Create a mock development environment
    pub fn development() -> Environment {
        Environment {
            id: "dev-env-001".to_string(),
            name: "Development".to_string(),
            r#type: EnvironmentType::Development,
            rpc_url: "http://localhost:8899".to_string(),
            ws_url: Some("ws://localhost:8900".to_string()),
            is_default: true,
            priority: 1,
            config: EnvironmentConfig {
                commitment: CommitmentLevel::Confirmed,
                preflight_checks: true,
                timeout: 30000,
            },
        }
    }

    /// Create a mock mainnet environment
    pub fn mainnet() -> Environment {
        Environment {
            id: "mainnet-env-001".to_string(),
            name: "Mainnet Beta".to_string(),
            r#type: EnvironmentType::Mainnet,
            rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            ws_url: Some("wss://api.mainnet-beta.solana.com".to_string()),
            is_default: false,
            priority: 100,
            config: EnvironmentConfig {
                commitment: CommitmentLevel::Finalized,
                preflight_checks: true,
                timeout: 60000,
            },
        }
    }

    /// Create a mock devnet environment
    pub fn devnet() -> Environment {
        Environment {
            id: "devnet-env-001".to_string(),
            name: "Devnet".to_string(),
            r#type: EnvironmentType::Devnet,
            rpc_url: "https://api.devnet.solana.com".to_string(),
            ws_url: Some("wss://api.devnet.solana.com".to_string()),
            is_default: false,
            priority: 50,
            config: EnvironmentConfig {
                commitment: CommitmentLevel::Confirmed,
                preflight_checks: true,
                timeout: 45000,
            },
        }
    }

    /// Create a mock testnet environment
    pub fn testnet() -> Environment {
        Environment {
            id: "testnet-env-001".to_string(),
            name: "Testnet".to_string(),
            r#type: EnvironmentType::Testnet,
            rpc_url: "https://api.testnet.solana.com".to_string(),
            ws_url: Some("wss://api.testnet.solana.com".to_string()),
            is_default: false,
            priority: 25,
            config: EnvironmentConfig {
                commitment: CommitmentLevel::Confirmed,
                preflight_checks: true,
                timeout: 40000,
            },
        }
    }
}

/// Mock project generator
pub struct MockProject;

impl MockProject {
    /// Create a mock SurfPool project
    pub fn surfpool() -> Project {
        Project {
            id: "project-001".to_string(),
            name: "SurfPool Testing".to_string(),
            description: Some("Test project for SurfPool validator management".to_string()),
            created_at: 1640995200, // Jan 1, 2022
            updated_at: 1640995200,
            config: ProjectConfig {
                default_environment: "dev-env-001".to_string(),
                auto_save: true,
                backup_enabled: true,
            },
            status: ProjectStatus::Active,
            owner: Some("test-user".to_string()),
            tags: vec!["surfpool".to_string(), "testing".to_string()],
        }
    }

    /// Create a mock DeFi project
    pub fn defi() -> Project {
        Project {
            id: "project-002".to_string(),
            name: "DeFi Trading Bot".to_string(),
            description: Some("Automated trading bot for DeFi protocols".to_string()),
            created_at: 1640995300,
            updated_at: 1640995400,
            config: ProjectConfig {
                default_environment: "mainnet-env-001".to_string(),
                auto_save: false,
                backup_enabled: true,
            },
            status: ProjectStatus::Active,
            owner: Some("defi-user".to_string()),
            tags: vec!["defi".to_string(), "trading".to_string(), "bot".to_string()],
        }
    }
}

/// Mock SurfPool state generator
pub struct MockSurfPool;

impl MockSurfPool {
    /// Create a mock running SurfPool instance
    pub fn running() -> SurfPoolState {
        SurfPoolState {
            status: ValidatorStatus::Running,
            pid: Some(12345),
            rpc_url: "http://localhost:8899".to_string(),
            ws_url: Some("ws://localhost:8900".to_string()),
            version: Some("1.18.0".to_string()),
            uptime: Some(3600), // 1 hour
            slot_height: Some(123456789),
            health_check: HealthCheck {
                is_healthy: true,
                last_check: 1640995200,
                response_time: 50,
            },
            config: SurfPoolConfig {
                fork_url: Some("https://api.mainnet-beta.solana.com".to_string()),
                account_count: 1000,
                memory_limit: "4GB".to_string(),
                enable_gossip: true,
            },
        }
    }

    /// Create a mock stopped SurfPool instance
    pub fn stopped() -> SurfPoolState {
        SurfPoolState {
            status: ValidatorStatus::Stopped,
            pid: None,
            rpc_url: "http://localhost:8899".to_string(),
            ws_url: Some("ws://localhost:8900".to_string()),
            version: Some("1.18.0".to_string()),
            uptime: None,
            slot_height: None,
            health_check: HealthCheck {
                is_healthy: false,
                last_check: 1640995200,
                response_time: 0,
            },
            config: SurfPoolConfig {
                fork_url: None,
                account_count: 0,
                memory_limit: "2GB".to_string(),
                enable_gossip: false,
            },
        }
    }
}

/// Test data collections for bulk testing
pub struct MockCollections;

impl MockCollections {
    /// Generate a collection of mock accounts
    pub fn accounts(count: usize) -> Vec<Account> {
        (0..count)
            .map(|i| Account {
                pubkey: format!("account-{:03}", i).parse().unwrap(),
                lamports: (i + 1) as u64 * 1000000000,
                data: vec![0u8; 1024],
                owner: MockAccount::system_program_id(),
                executable: i % 5 == 0,
                rent_epoch: (i + 1) as u64 * 10,
                label: Some(format!("Test Account {}", i + 1)),
                account_type: if i % 5 == 0 {
                    "program".to_string()
                } else {
                    "system".to_string()
                },
                is_watched: i % 2 == 0,
            })
            .collect()
    }

    /// Generate a collection of mock transactions
    pub fn transactions(count: usize) -> Vec<Transaction> {
        (0..count)
            .map(|i| {
                if i % 10 == 0 {
                    MockTransaction::failed()
                } else if i % 3 == 0 {
                    MockTransaction::pending()
                } else {
                    MockTransaction::confirmed()
                }
            })
            .collect()
    }

    /// Generate a collection of mock environments
    pub fn environments() -> Vec<Environment> {
        vec![
            MockEnvironment::development(),
            MockEnvironment::devnet(),
            MockEnvironment::testnet(),
            MockEnvironment::mainnet(),
        ]
    }

    /// Generate a collection of mock projects
    pub fn projects(count: usize) -> Vec<Project> {
        (0..count)
            .map(|i| Project {
                id: format!("project-{:03}", i + 1),
                name: format!("Test Project {}", i + 1),
                description: Some(format!("Description for test project {}", i + 1)),
                created_at: 1640995200 + (i as u64 * 3600),
                updated_at: 1640995200 + (i as u64 * 3600),
                config: ProjectConfig {
                    default_environment: if i % 2 == 0 {
                        "dev-env-001".to_string()
                    } else {
                        "mainnet-env-001".to_string()
                    },
                    auto_save: i % 3 == 0,
                    backup_enabled: true,
                },
                status: if i % 10 == 0 {
                    ProjectStatus::Archived
                } else {
                    ProjectStatus::Active
                },
                owner: Some(format!("user-{:03}", i + 1)),
                tags: vec!["test".to_string(), format!("tag-{}", i + 1)],
            })
            .collect()
    }
}

// Helper structs for mock data (these should match your actual type definitions)
#[derive(Debug, Clone)]
pub struct Account {
    pub pubkey: Pubkey,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: u64,
    pub label: Option<String>,
    pub account_type: String,
    pub is_watched: bool,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub signature: String,
    pub slot: Option<u64>,
    pub block_time: Option<i64>,
    pub ConfirmationStatus: TransactionStatus,
    pub err: Option<String>,
    pub meta: Option<TransactionMeta>,
    pub message: TransactionMessage,
    pub fee: Option<u64>,
    pub compute_units_consumed: Option<u64>,
}

#[derive(Debug, Clone)]
pub enum TransactionStatus {
    Confirmed,
    Pending,
    Failed,
}

#[derive(Debug, Clone)]
pub struct TransactionMeta {
    pub err: Option<String>,
    pub fee: u64,
    pub pre_balances: Vec<u64>,
    pub post_balances: Vec<u64>,
    pub inner_instructions: Vec<InnerInstruction>,
    pub log_messages: Vec<String>,
    pub pre_token_balances: Vec<TokenBalance>,
    pub post_token_balances: Vec<TokenBalance>,
    pub rewards: Vec<Reward>,
    pub loaded_addresses: LoadedAddresses,
}

#[derive(Debug, Clone)]
pub struct TransactionMessage {
    pub account_keys: Vec<Pubkey>,
    pub recent_blockhash: String,
    pub instructions: Vec<CompiledInstruction>,
}

#[derive(Debug, Clone)]
pub struct CompiledInstruction {
    pub program_id_index: u8,
    pub account_key_indexes: Vec<u8>,
    pub data: Vec<u8>,
}

impl CompiledInstruction {
    pub fn new(program_id_index: u8, account_key_indexes: Vec<u8>, data: Vec<u8>) -> Self {
        Self {
            program_id_index,
            account_key_indexes,
            data,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InnerInstruction {
    pub instruction: CompiledInstruction,
    pub stack_height: Option<u8>,
}

#[derive(Debug, Clone)]
pub struct TokenBalance {
    pub account_index: u8,
    pub mint: String,
    pub ui_token_amount: UiTokenAmount,
    pub owner: Option<String>,
    pub program_id: String,
}

#[derive(Debug, Clone)]
pub struct UiTokenAmount {
    pub ui_amount: Option<f64>,
    pub decimals: u8,
    pub amount: String,
    pub ui_amount_string: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Reward {
    pub pubkey: String,
    pub lamports: i64,
    pub post_balance: u64,
    pub reward_type: Option<RewardType>,
}

#[derive(Debug, Clone)]
pub enum RewardType {
    Fee,
    Rent,
    Staking,
    Voting,
}

#[derive(Debug, Clone)]
pub struct LoadedAddresses {
    pub writable: Vec<Pubkey>,
    pub readonly: Vec<Pubkey>,
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub id: String,
    pub name: String,
    pub r#type: EnvironmentType,
    pub rpc_url: String,
    pub ws_url: Option<String>,
    pub is_default: bool,
    pub priority: u32,
    pub config: EnvironmentConfig,
}

#[derive(Debug, Clone)]
pub enum EnvironmentType {
    Development,
    Mainnet,
    Devnet,
    Testnet,
    Local,
}

#[derive(Debug, Clone)]
pub struct EnvironmentConfig {
    pub commitment: CommitmentLevel,
    pub preflight_checks: bool,
    pub timeout: u32,
}

#[derive(Debug, Clone)]
pub enum CommitmentLevel {
    Processed,
    Confirmed,
    Finalized,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub config: ProjectConfig,
    pub status: ProjectStatus,
    pub owner: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub default_environment: String,
    pub auto_save: bool,
    pub backup_enabled: bool,
}

#[derive(Debug, Clone)]
pub enum ProjectStatus {
    Active,
    Archived,
    Deleted,
}

#[derive(Debug, Clone)]
pub struct SurfPoolState {
    pub status: ValidatorStatus,
    pub pid: Option<u32>,
    pub rpc_url: String,
    pub ws_url: Option<String>,
    pub version: Option<String>,
    pub uptime: Option<u64>,
    pub slot_height: Option<u64>,
    pub health_check: HealthCheck,
    pub config: SurfPoolConfig,
}

#[derive(Debug, Clone)]
pub enum ValidatorStatus {
    Running,
    Stopped,
    Starting,
    Stopping,
    Error,
}

#[derive(Debug, Clone)]
pub struct HealthCheck {
    pub is_healthy: bool,
    pub last_check: i64,
    pub response_time: u64,
}

#[derive(Debug, Clone)]
pub struct SurfPoolConfig {
    pub fork_url: Option<String>,
    pub account_count: u32,
    pub memory_limit: String,
    pub enable_gossip: bool,
}
