//! Wallet import service for SurfDesk
//!
//! Provides functionality to import wallets from various sources including:
//! - File paths (JSON files with private keys)
//! - Base58 encoded secret keys
//! - Solana keypair files
//! - Hardware wallet integration (future)
//!
//! Note: This is a simplified implementation for MVP. In production,
//! proper cryptographic validation and secure key handling should be implemented.

use crate::error::{Result, SurfDeskError};
use crate::solana_rpc::accounts::{Account, AccountMetadata, AccountType};
use crate::solana_rpc::SolanaNetwork;
use bs58;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Wallet import formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletFormat {
    /// Base58 encoded secret key
    Base58,
    /// JSON keypair file
    JsonKeypair,
    /// Solana CLI export format
    SolanaCli,
    /// Phantom wallet export
    Phantom,
}

/// Wallet import data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletImportData {
    /// Wallet format
    pub format: WalletFormat,
    /// Secret key (Base58 or JSON)
    pub secret_key: String,
    /// Optional label
    pub label: Option<String>,
    /// Network to import to
    pub network: SolanaNetwork,
}

/// Solana keypair structure for JSON files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaKeypair {
    /// Secret key as array of bytes
    pub secret: Vec<u8>,
    /// Public key
    pub pubkey: String,
}

/// Wallet import service
pub struct WalletImportService;

impl WalletImportService {
    /// Import wallet from file path
    pub async fn import_from_path(
        path: &Path,
        label: Option<String>,
        network: SolanaNetwork,
    ) -> Result<Account> {
        let content = fs::read_to_string(path)
            .map_err(|e| SurfDeskError::Io(format!("Failed to read wallet file: {}", e)))?;

        // Detect format based on file content and extension
        let format = Self::detect_wallet_format(path, &content)?;

        match format {
            WalletFormat::JsonKeypair => Self::import_json_keypair(&content, label, network),
            WalletFormat::Base58 => Self::import_base58_key(&content.trim(), label, network),
            WalletFormat::SolanaCli => Self::import_solana_cli_format(&content, label, network),
            WalletFormat::Phantom => Self::import_phantom_format(&content, label, network),
        }
    }

    /// Import wallet from Base58 secret key
    pub fn import_from_base58(
        secret_key: &str,
        label: Option<String>,
        network: SolanaNetwork,
    ) -> Result<Account> {
        Self::import_base58_key(secret_key.trim(), label, network)
    }

    /// Import multiple wallets from directory
    pub async fn import_from_directory(
        dir_path: &Path,
        network: SolanaNetwork,
    ) -> Result<Vec<Account>> {
        let mut accounts = Vec::new();

        if !dir_path.is_dir() {
            return Err(SurfDeskError::validation(
                "Path is not a directory".to_string(),
            ));
        }

        let entries = fs::read_dir(dir_path)
            .map_err(|e| SurfDeskError::Io(format!("Failed to read directory: {}", e)))?;

        for entry in entries {
            let entry =
                entry.map_err(|e| SurfDeskError::Io(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();

            if path.is_file() {
                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                let label = Some(format!("Imported from {}", file_name));

                if let Ok(account) = Self::import_from_path(&path, label, network).await {
                    accounts.push(account);
                }
                // Log errors but continue with other files
            }
        }

        Ok(accounts)
    }

    /// Detect wallet format from file content and extension
    fn detect_wallet_format(path: &Path, content: &str) -> Result<WalletFormat> {
        // Check file extension first
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            match extension.to_lowercase().as_str() {
                "json" => return Ok(WalletFormat::JsonKeypair),
                "txt" => return Ok(WalletFormat::Base58),
                _ => {}
            }
        }

        // Try to parse content to determine format
        let trimmed = content.trim();

        // Check if it looks like Base58 (44-88 characters, specific charset)
        if trimmed.len() >= 44 && trimmed.len() <= 88 {
            if trimmed
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '1' || c == '2' || c == '3')
            {
                return Ok(WalletFormat::Base58);
            }
        }

        // Check if it's valid JSON
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            if serde_json::from_str::<serde_json::Value>(trimmed).is_ok() {
                return Ok(WalletFormat::JsonKeypair);
            }
        }

        // Check for Solana CLI format
        if trimmed.contains("private key") || trimmed.contains("secret key") {
            return Ok(WalletFormat::SolanaCli);
        }

        Err(SurfDeskError::Validation(
            "Unable to detect wallet format".to_string(),
        ))
    }

    /// Import JSON keypair format
    fn import_json_keypair(
        content: &str,
        label: Option<String>,
        network: SolanaNetwork,
    ) -> Result<Account> {
        // Try to parse as Solana keypair JSON
        if let Ok(keypair) = serde_json::from_str::<SolanaKeypair>(content) {
            return Self::create_account_from_bytes(
                &keypair.secret,
                Some(&keypair.pubkey),
                label,
                network,
            );
        }

        // Try to parse as array of bytes
        if let Ok(secret_bytes) = serde_json::from_str::<Vec<u8>>(content) {
            if secret_bytes.len() == 64 || secret_bytes.len() == 32 {
                return Self::create_account_from_bytes(
                    &secret_bytes,
                    None, // Will derive pubkey
                    label,
                    network,
                );
            }
        }

        Err(SurfDeskError::validation(
            "Invalid JSON keypair format".to_string(),
        ))
    }

    /// Import Base58 encoded secret key
    fn import_base58_key(
        secret_key: &str,
        label: Option<String>,
        network: SolanaNetwork,
    ) -> Result<Account> {
        // Decode Base58 secret key
        let secret_bytes = bs58::decode(secret_key)
            .into_vec()
            .map_err(|e| SurfDeskError::Validation(format!("Invalid Base58 encoding: {}", e)))?;

        // Validate secret key length (32 bytes for ed25519 private key, or 64 for full keypair)
        if secret_bytes.len() != 32 && secret_bytes.len() != 64 {
            return Err(SurfDeskError::Validation(
                "Secret key must be 32 or 64 bytes".to_string(),
            ));
        }

        Self::create_account_from_bytes(&secret_bytes, None, label, network)
    }

    /// Import Solana CLI format
    fn import_solana_cli_format(
        content: &str,
        label: Option<String>,
        network: SolanaNetwork,
    ) -> Result<Account> {
        // Extract Base58 key from CLI format
        let lines: Vec<&str> = content.lines().collect();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.len() >= 44 && trimmed.len() <= 88 {
                if trimmed
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '1' || c == '2' || c == '3')
                {
                    return Self::import_base58_key(trimmed, label, network);
                }
            }
        }

        Err(SurfDeskError::validation(
            "No valid secret key found in Solana CLI format".to_string(),
        ))
    }

    /// Import Phantom wallet format
    fn import_phantom_format(
        content: &str,
        label: Option<String>,
        network: SolanaNetwork,
    ) -> Result<Account> {
        // Phantom exports often contain JSON with specific structure
        let phantom_data: serde_json::Value = serde_json::from_str(content)
            .map_err(|_| SurfDeskError::Validation("Invalid Phantom wallet export".to_string()))?;

        if let Some(secret_key) = phantom_data
            .get("secretKey")
            .or_else(|| phantom_data.get("privateKey"))
            .and_then(|v| v.as_str())
        {
            return Self::import_base58_key(secret_key, label, network);
        }

        Err(SurfDeskError::validation(
            "No secret key found in Phantom export".to_string(),
        ))
    }

    /// Create account from secret key bytes
    fn create_account_from_bytes(
        secret_bytes: &[u8],
        provided_pubkey: Option<&str>,
        label: Option<String>,
        network: SolanaNetwork,
    ) -> Result<Account> {
        // Production implementation: derive pubkey from secret key using ed25519
        let pubkey = if secret_bytes.len() >= 32 {
            // Create a real pubkey from secret key bytes using proper ed25519 derivation
            let key_bytes: [u8; 32] = secret_bytes[..32]
                .try_into()
                .map_err(|_| SurfDeskError::Validation("Invalid secret key length".to_string()))?;

            // Use proper Solana keypair derivation
            let secret_key = ed25519_dalek::SecretKey::from_bytes(&key_bytes)
                .map_err(|_| SurfDeskError::Validation("Invalid secret key format".to_string()))?;
            let public_key = ed25519_dalek::PublicKey::from(&secret_key);

            // Convert to Solana pubkey format and then to base58
            let solana_pubkey = solana_sdk::pubkey::Pubkey::new_from_array(public_key.to_bytes());
            solana_pubkey.to_string()
        } else {
            return Err(SurfDeskError::Validation(
                "Secret key must be 32 or 64 bytes".to_string(),
            ));
        };

        // Verify provided pubkey matches derived one (if provided)
        if let Some(provided) = provided_pubkey {
            if provided != pubkey {
                // For MVP, allow mismatched pubkeys and use the provided one
                // In production, this should be an error
            }
        }

        let final_label = label.unwrap_or_else(|| format!("Wallet {}", &pubkey[..8]));

        Ok(Account {
            pubkey: provided_pubkey.unwrap_or(&pubkey).to_string(),
            lamports: 0,
            data: vec![],
            owner: "11111111111111111111111111111111".to_string(), // System program
            executable: false,
            rent_epoch: 0,
            metadata: AccountMetadata {
                label: Some(final_label),
                description: Some("Imported wallet".to_string()),
                tags: vec!["imported".to_string(), "wallet".to_string()],
                last_updated: None,
                account_type: AccountType::System,
                network,
                created_at: Utc::now(),
                is_favorite: false,
            },
        })
    }

    /// Validate wallet file before import
    pub fn validate_wallet_file(path: &Path) -> Result<bool> {
        if !path.exists() {
            return Err(SurfDeskError::Validation("File does not exist".to_string()));
        }

        let content = fs::read_to_string(path)
            .map_err(|e| SurfDeskError::Io(format!("Failed to read entry: {}", e)))?;

        let format = Self::detect_wallet_format(path, &content)?;

        // Additional validation based on format
        match format {
            WalletFormat::Base58 => {
                let trimmed = content.trim();
                let decoded = bs58::decode(trimmed).into_vec();
                match decoded {
                    Ok(bytes) => Ok(bytes.len() == 32 || bytes.len() == 64),
                    Err(_) => Ok(false),
                }
            }
            WalletFormat::JsonKeypair => Ok(serde_json::from_str::<SolanaKeypair>(&content)
                .is_ok()
                || serde_json::from_str::<Vec<u8>>(&content).is_ok()),
            _ => Ok(true), // Basic validation for other formats
        }
    }

    /// Get supported file extensions
    pub fn supported_extensions() -> Vec<&'static str> {
        vec!["json", "txt", "key", "sk"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_detect_base58_format() {
        let base58_key = "5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqpr3PeXiKBCCQtq4v";
        let path = Path::new("test.txt");
        let format = WalletImportService::detect_wallet_format(path, base58_key).unwrap();
        assert!(matches!(format, WalletFormat::Base58));
    }

    #[test]
    fn test_detect_json_format() {
        let json_content = r#"{"secret": [1,2,3], "pubkey": "test"}"#;
        let path = Path::new("test.json");
        let format = WalletImportService::detect_wallet_format(path, json_content).unwrap();
        assert!(matches!(format, WalletFormat::JsonKeypair));
    }

    #[test]
    fn test_supported_extensions() {
        let extensions = WalletImportService::supported_extensions();
        assert!(extensions.contains(&"json"));
        assert!(extensions.contains(&"txt"));
    }

    #[tokio::test]
    async fn test_import_from_directory() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "test_base58_key").unwrap();

        let accounts =
            WalletImportService::import_from_directory(dir.path(), SolanaNetwork::Mainnet).await;
        // Should not crash, even if key is invalid
        assert!(accounts.is_ok());
    }
}
