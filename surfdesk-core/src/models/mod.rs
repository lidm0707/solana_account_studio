//! Data models for Surfdesk
//!
//! This module contains the core data structures used throughout the application.

use serde::{Deserialize, Serialize};
// use std::collections::HashMap; // Commented out as unused

/// Account information model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub address: String,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
}

/// Program information model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub accounts: Vec<ProgramAccount>,
    pub instructions: Vec<Instruction>,
}

/// Program account model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramAccount {
    pub name: String,
    pub address: String,
    pub is_signer: bool,
    pub is_mutable: bool,
    pub account_type: AccountType,
}

/// Account type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    System,
    Program,
    Token,
    Custom(String),
}

/// Instruction model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub name: String,
    pub accounts: Vec<String>,
    pub args: Vec<InstructionArg>,
}

/// Instruction argument model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionArg {
    pub name: String,
    pub arg_type: ArgType,
    pub optional: bool,
}

/// Argument type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArgType {
    String,
    Number,
    Boolean,
    Array,
    Object,
    Custom(String),
}

/// Transaction status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

/// Transaction model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub signature: String,
    pub status: TransactionStatus,
    pub slot: u64,
    pub block_time: Option<i64>,
    pub fee: u64,
}

/// Surfpool process model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfpoolProcess {
    pub id: String,
    pub name: String,
    pub status: ProcessStatus,
    pub pid: Option<u32>,
    pub port: Option<u16>,
    pub endpoint: Option<String>,
}

/// Process status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessStatus {
    Running,
    Stopped,
    Error(String),
    Unknown,
}

/// Project configuration model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub description: Option<String>,
    pub network: Network,
    pub programs: Vec<Program>,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Network {
    Mainnet,
    Devnet,
    Testnet,
    Localhost,
    Custom(String),
}

/// API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}
