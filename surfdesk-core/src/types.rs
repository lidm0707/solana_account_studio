//! # Shared Types Module
//!
//! This module defines all the shared data structures and types used across
//! the SurfDesk application. These types are platform-agnostic and provide
//! a consistent data model for all components.

use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use uuid::Uuid;

/// Project identifier
pub type ProjectId = Uuid;

/// Environment identifier
pub type EnvironmentId = Uuid;

/// Account identifier
pub type AccountId = Uuid;

/// Transaction identifier
pub type TransactionId = Uuid;

/// Solana public key wrapper with serialization support
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SolanaPubkey(#[serde(with = "solana_pubkey_serde")] Pubkey);

impl SolanaPubkey {
    /// Create a new SolanaPubkey from a string
    pub fn from_str(s: &str) -> Result<Self, crate::error::SurfDeskError> {
        let pubkey = s
            .parse::<Pubkey>()
            .map_err(|_| crate::error::SurfDeskError::validation("Invalid Solana public key"))?;
        Ok(Self(pubkey))
    }

    /// Get the underlying Pubkey
    pub fn inner(&self) -> &Pubkey {
        &self.0
    }

    /// Get the string representation
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<Pubkey> for SolanaPubkey {
    fn from(pubkey: Pubkey) -> Self {
        Self(pubkey)
    }
}

/// Solana public key serialization module
mod solana_pubkey_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use solana_sdk::pubkey::Pubkey;

    pub fn serialize<S>(pubkey: &Pubkey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&pubkey.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<Pubkey>().map_err(serde::de::Error::custom)
    }
}

/// Project structure for managing Solana development projects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Unique project identifier
    pub id: ProjectId,
    /// Project name
    pub name: String,
    /// Project description
    pub description: Option<String>,
    /// Project owner
    pub owner: String,
    /// Project settings
    pub settings: ProjectSettings,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Project-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    /// Default Solana network
    pub default_network: SolanaNetwork,
    /// Auto-save enabled
    pub auto_save: bool,
    /// Auto-save interval in seconds
    pub auto_save_interval: u64,
    /// Maximum number of saved snapshots
    pub max_snapshots: usize,
    /// Custom RPC endpoints
    pub custom_endpoints: Vec<String>,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            default_network: SolanaNetwork::Devnet,
            auto_save: true,
            auto_save_interval: 30,
            max_snapshots: 100,
            custom_endpoints: Vec::new(),
        }
    }
}

/// Solana network types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SolanaNetwork {
    /// Mainnet-beta
    Mainnet,
    /// Devnet
    Devnet,
    /// Testnet
    Testnet,
    /// Local validator
    Local,
    /// Custom endpoint
    Custom,
}

impl SolanaNetwork {
    /// Get the RPC URL for this network
    pub fn rpc_url(&self) -> &'static str {
        match self {
            Self::Mainnet => crate::DEFAULT_SOLANA_RPC_URL,
            Self::Devnet => crate::DEVNET_URL,
            Self::Testnet => crate::TESTNET_URL,
            Self::Local => crate::LOCAL_VALIDATOR_URL,
            Self::Custom => panic!("Custom network requires explicit URL"),
        }
    }

    /// Get display name for this network
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Mainnet => "Mainnet",
            Self::Devnet => "Devnet",
            Self::Testnet => "Testnet",
            Self::Local => "Local",
            Self::Custom => "Custom",
        }
    }
}

/// Environment configuration for different deployment contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    /// Unique environment identifier
    pub id: EnvironmentId,
    /// Parent project ID
    pub project_id: ProjectId,
    /// Environment name
    pub name: String,
    /// Environment type
    pub env_type: EnvironmentType,
    /// Solana network
    pub network: SolanaNetwork,
    /// Custom RPC URL (for Custom network type)
    pub rpc_url: Option<String>,
    /// Environment-specific configuration
    pub config: EnvironmentConfig,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Environment types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnvironmentType {
    /// Development environment
    Development,
    /// Testing environment
    Testing,
    /// Staging environment
    Staging,
    /// Production environment
    Production,
}

impl EnvironmentType {
    /// Get display name for this environment type
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Development => "Development",
            Self::Testing => "Testing",
            Self::Staging => "Staging",
            Self::Production => "Production",
        }
    }
}

/// Environment-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    /// Key pairs available in this environment
    pub keypairs: HashMap<String, KeypairInfo>,
    /// Program deployments
    pub programs: HashMap<String, ProgramInfo>,
    /// Environment variables
    pub env_vars: HashMap<String, String>,
    /// Airdrop configuration
    pub airdrop_config: AirdropConfig,
}

/// Keypair information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeypairInfo {
    /// Keypair name
    pub name: String,
    /// Public key
    pub pubkey: SolanaPubkey,
    /// Whether this keypair is encrypted
    pub encrypted: bool,
    /// Key metadata
    pub metadata: HashMap<String, String>,
}

/// Program deployment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramInfo {
    /// Program name
    pub name: String,
    /// Program ID
    pub program_id: SolanaPubkey,
    /// Program version
    pub version: String,
    /// Program path
    pub path: Option<String>,
    /// Deployment timestamp
    pub deployed_at: chrono::DateTime<chrono::Utc>,
}

/// Airdrop configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirdropConfig {
    /// Whether airdrop is enabled
    pub enabled: bool,
    /// Default airdrop amount in lamports
    pub default_amount: u64,
    /// Maximum daily airdrop amount
    pub max_daily_amount: u64,
    /// Airdrop history
    pub history: Vec<AirdropRecord>,
}

/// Airdrop record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirdropRecord {
    /// Recipient public key
    pub recipient: SolanaPubkey,
    /// Amount airdropped
    pub amount: u64,
    /// Transaction signature
    pub signature: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Unique account identifier
    pub id: AccountId,
    /// Parent environment ID
    pub environment_id: EnvironmentId,
    /// Account public key
    pub pubkey: SolanaPubkey,
    /// Account owner program
    pub owner: SolanaPubkey,
    /// Account balance in lamports
    pub balance: u64,
    /// Account data
    pub data: AccountData,
    /// Whether the account is executable
    pub executable: bool,
    /// Account rent epoch
    pub rent_epoch: u64,
    /// Last updated timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Account data with type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountData {
    /// Raw account data as base64
    pub raw_data: String,
    /// Parsed data if available
    pub parsed_data: Option<serde_json::Value>,
    /// Data type information
    pub data_type: AccountDataType,
    /// Account schema if available
    pub schema: Option<AccountSchema>,
}

/// Account data type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountDataType {
    /// Unknown data type
    Unknown,
    /// System program account
    System,
    /// Token account
    Token,
    /// Mint account
    Mint,
    /// Program account
    Program,
    /// Custom account with IDL
    Custom,
    /// Raw data
    Raw,
}

/// Account schema information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSchema {
    /// Schema name
    pub name: String,
    /// Schema version
    pub version: String,
    /// Field definitions
    pub fields: Vec<SchemaField>,
}

/// Schema field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: SchemaFieldType,
    /// Field offset in bytes
    pub offset: usize,
    /// Field size in bytes
    pub size: usize,
    /// Whether the field is optional
    pub optional: bool,
}

/// Schema field type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchemaFieldType {
    /// Unsigned integer
    U8,
    U16,
    U32,
    U64,
    U128,
    /// Signed integer
    I8,
    I16,
    I32,
    I64,
    I128,
    /// Boolean
    Bool,
    /// String
    String,
    /// Bytes
    Bytes,
    /// Public key
    Pubkey,
    /// Array type
    Array {
        /// Element type
        element_type: Box<SchemaFieldType>,
        /// Array length
        length: usize,
    },
    /// Vector type
    Vector {
        /// Element type
        element_type: Box<SchemaFieldType>,
    },
    /// Option type
    Option {
        /// Inner type
        inner_type: Box<SchemaFieldType>,
    },
    /// Custom struct
    Struct {
        /// Struct name
        name: String,
        /// Fields
        fields: Vec<SchemaField>,
    },
    /// Custom enum
    Enum {
        /// Enum name
        name: String,
        /// Variants
        variants: Vec<EnumVariant>,
    },
}

/// Enum variant definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    /// Variant name
    pub name: String,
    /// Variant discriminator
    pub discriminator: u8,
    /// Variant fields
    pub fields: Vec<SchemaField>,
}

/// Transaction information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Unique transaction identifier
    pub id: TransactionId,
    /// Parent environment ID
    pub environment_id: EnvironmentId,
    /// Transaction signature
    pub signature: String,
    /// Transaction status
    pub status: TransactionStatus,
    /// Transaction details
    pub details: TransactionDetails,
    /// Simulation results if available
    pub simulation: Option<SimulationResult>,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Confirmation timestamp
    pub confirmed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Transaction status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    /// Transaction is being prepared
    Preparing,
    /// Transaction is being simulated
    Simulating,
    /// Transaction is being sent
    Sending,
    /// Transaction is pending confirmation
    Pending,
    /// Transaction confirmed successfully
    Confirmed,
    /// Transaction failed
    Failed,
}

/// Transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDetails {
    /// Transaction fee in lamports
    pub fee: u64,
    /// Compute units consumed
    pub compute_units: Option<u64>,
    /// Log messages
    pub logs: Vec<String>,
    /// Error message if any
    pub error: Option<String>,
    /// Block height
    pub block_height: Option<u64>,
    /// Block time
    pub block_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// Simulation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    /// Whether simulation was successful
    pub success: bool,
    /// Compute units consumed
    pub compute_units_consumed: u64,
    /// Simulation logs
    pub logs: Vec<String>,
    /// Error message if any
    pub error: Option<String>,
    /// Account changes
    pub account_changes: Vec<AccountChange>,
}

/// Account change from simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountChange {
    /// Account public key
    pub pubkey: SolanaPubkey,
    /// Change type
    pub change_type: AccountChangeType,
    /// Previous balance
    pub previous_balance: Option<u64>,
    /// New balance
    pub new_balance: Option<u64>,
    /// Data change description
    pub data_change: Option<String>,
}

/// Account change type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountChangeType {
    /// Account created
    Created,
    /// Account updated
    Updated,
    /// Account deleted
    Deleted,
    /// Account balance changed
    BalanceChanged,
}

/// UI state for platform-specific rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIState {
    /// Current theme
    pub theme: Theme,
    /// Sidebar state
    pub sidebar: SidebarState,
    /// Main content state
    pub main_content: MainContentState,
    /// Modal state
    pub modal: Option<ModalState>,
    /// Notification state
    pub notifications: Vec<Notification>,
}

/// Theme configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    /// Light theme
    Light,
    /// Dark theme
    Dark,
    /// Auto (system preference)
    Auto,
}

/// Sidebar state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidebarState {
    /// Whether sidebar is expanded
    pub expanded: bool,
    /// Active section
    pub active_section: SidebarSection,
    /// Collapsed sections
    pub collapsed_sections: Vec<SidebarSection>,
}

/// Sidebar sections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SidebarSection {
    /// Projects
    Projects,
    /// Environments
    Environments,
    /// Accounts
    Accounts,
    /// Transactions
    Transactions,
    /// Programs
    Programs,
    /// Settings
    Settings,
}

/// Main content state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainContentState {
    /// Current view
    pub current_view: ContentView,
    /// View-specific state
    pub view_state: serde_json::Value,
}

/// Content view types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentView {
    /// Dashboard
    Dashboard,
    /// Account explorer
    AccountExplorer,
    /// Transaction builder
    TransactionBuilder,
    /// Program manager
    ProgramManager,
    /// Settings
    Settings,
}

/// Modal state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModalState {
    /// Modal type
    pub modal_type: ModalType,
    /// Modal title
    pub title: String,
    /// Modal content
    pub content: serde_json::Value,
    /// Whether modal is closable
    pub closable: bool,
}

/// Modal types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModalType {
    /// Information modal
    Info,
    /// Confirmation modal
    Confirm,
    /// Error modal
    Error,
    /// Input modal
    Input,
    /// Custom modal
    Custom,
}

/// Notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Notification ID
    pub id: String,
    /// Notification type
    pub notification_type: NotificationType,
    /// Notification title
    pub title: String,
    /// Notification message
    pub message: String,
    /// Whether notification is persistent
    pub persistent: bool,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Notification types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationType {
    /// Success notification
    Success,
    /// Error notification
    Error,
    /// Warning notification
    Warning,
    /// Info notification
    Info,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solana_pubkey() {
        let pubkey_str = "11111111111111111111111111111112";
        let pubkey = SolanaPubkey::from_str(pubkey_str).unwrap();
        assert_eq!(pubkey.to_string(), pubkey_str);
    }

    #[test]
    fn test_solana_network() {
        assert_eq!(SolanaNetwork::Mainnet.display_name(), "Mainnet");
        assert_eq!(SolanaNetwork::Devnet.rpc_url(), crate::DEVNET_URL);
    }

    #[test]
    fn test_project_settings_default() {
        let settings = ProjectSettings::default();
        assert_eq!(settings.default_network, SolanaNetwork::Devnet);
        assert!(settings.auto_save);
        assert_eq!(settings.auto_save_interval, 30);
    }

    #[test]
    fn test_serialization() {
        let project = Project {
            id: Uuid::new_v4(),
            name: "Test Project".to_string(),
            description: Some("A test project".to_string()),
            owner: "test-user".to_string(),
            settings: ProjectSettings::default(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let serialized = serde_json::to_string(&project).unwrap();
        let deserialized: Project = serde_json::from_str(&serialized).unwrap();
        assert_eq!(project.name, deserialized.name);
        assert_eq!(project.owner, deserialized.owner);
    }
}
