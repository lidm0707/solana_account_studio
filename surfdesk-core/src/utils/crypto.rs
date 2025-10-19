//! Cryptographic utilities for Surfdesk
//!
//! This module provides cryptographic functions and utilities for Solana
//! address generation, keypair management, Base58 encoding/decoding,
//! and other cryptographic operations needed for the application.

use bs58;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use sha2::{Digest, Sha256};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Generate a new Solana keypair (mock implementation)
pub fn generate_keypair() -> (String, String) {
    // In a real implementation, this would generate actual Ed25519 keypair
    // For now, generate mock addresses using deterministic random
    let mut rng = ChaCha8Rng::from_entropy();
    let mut seed = [0u8; 32];
    rng.fill_bytes(&mut seed);

    let public_key = generate_public_key(&seed);
    let private_key = bs58::encode(&seed).into_string();

    (public_key, private_key)
}

/// Generate a public key from seed bytes
pub fn generate_public_key(seed: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(seed);
    hasher.update(b"public_key");
    let hash = hasher.finalize();

    // Use first 32 bytes for address
    bs58::encode(&hash[..32]).into_string()
}

/// Validate a Solana address (Base58 format)
pub fn validate_address(address: &str) -> bool {
    bs58::decode(address).into_vec().is_ok()
}

/// Generate a mock program ID from program data
pub fn generate_program_id(program_data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(program_data);
    hasher.update(chrono::Utc::now().timestamp_nanos().to_be_bytes());
    let hash = hasher.finalize();

    bs58::encode(&hash[..32]).into_string()
}

/// Create a hash from string input
pub fn hash_string(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let hash = hasher.finalize();

    format!("{:x}", hash)
}

/// Generate a deterministic mock address from string
pub fn generate_deterministic_address(seed: &str) -> String {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let hash = hasher.finish();

    // Convert to 32-byte array for Base58 encoding
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&hash.to_be_bytes());

    bs58::encode(bytes).into_string()
}

/// Generate random bytes
pub fn generate_random_bytes(length: usize) -> Vec<u8> {
    let mut rng = ChaCha8Rng::from_entropy();
    let mut bytes = vec![0u8; length];
    rng.fill_bytes(&mut bytes);
    bytes
}

/// Create a simple checksum for data validation
pub fn create_checksum(data: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

/// Verify checksum of data
pub fn verify_checksum(data: &[u8], expected: u64) -> bool {
    create_checksum(data) == expected
}

/// Generate a mock signature for transactions
pub fn generate_mock_signature() -> String {
    let signature_bytes = generate_random_bytes(64);
    bs58::encode(signature_bytes).into_string()
}

/// Convert bytes to hex string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Convert hex string to bytes
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, &'static str> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have even length");
    }

    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Invalid hex character")
}

/// Generate a unique identifier
pub fn generate_uuid() -> String {
    let mut hasher = Sha256::new();
    hasher.update(chrono::Utc::now().timestamp_nanos().to_be_bytes());
    hasher.update(generate_random_bytes(16));
    let hash = hasher.finalize();

    // Format as UUID-like string
    format!(
        "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]]),
        u16::from_be_bytes([hash[4], hash[5]]),
        u16::from_be_bytes([hash[6], hash[7]]),
        u16::from_be_bytes([hash[8], hash[9]]),
        u64::from_be_bytes([
            hash[10], hash[11], hash[12], hash[13], hash[14], hash[15], hash[16], hash[17]
        ])
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let (public_key, private_key) = generate_keypair();

        assert!(!public_key.is_empty());
        assert!(!private_key.is_empty());
        assert_ne!(public_key, private_key);
        assert!(validate_address(&public_key));
    }

    #[test]
    fn test_validate_address() {
        let valid_address = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM";
        let invalid_address = "invalid_address";

        assert!(validate_address(valid_address));
        assert!(!validate_address(invalid_address));
    }

    #[test]
    fn test_generate_program_id() {
        let program_data = b"test_program_data";
        let program_id = generate_program_id(program_data);

        assert!(!program_id.is_empty());
        assert!(validate_address(&program_id));
    }

    #[test]
    fn test_hash_string() {
        let input = "test_input";
        let hash1 = hash_string(input);
        let hash2 = hash_string(input);

        assert_eq!(hash1, hash2);
        assert!(!hash1.is_empty());
        assert_eq!(hash1.len(), 64); // SHA256 hex length
    }

    #[test]
    fn test_generate_deterministic_address() {
        let seed = "test_seed";
        let address1 = generate_deterministic_address(seed);
        let address2 = generate_deterministic_address(seed);

        assert_eq!(address1, address2);
        assert!(validate_address(&address1));
    }

    #[test]
    fn test_generate_random_bytes() {
        let bytes1 = generate_random_bytes(32);
        let bytes2 = generate_random_bytes(32);

        assert_eq!(bytes1.len(), 32);
        assert_eq!(bytes2.len(), 32);
        assert_ne!(bytes1, bytes2); // Should be different
    }

    #[test]
    fn test_checksum() {
        let data = b"test_data";
        let checksum = create_checksum(data);

        assert!(verify_checksum(data, checksum));
        assert!(!verify_checksum(b"different_data", checksum));
    }

    #[test]
    fn test_generate_mock_signature() {
        let signature = generate_mock_signature();

        assert!(!signature.is_empty());
        assert!(bs58::decode(&signature).into_vec().is_ok());
    }

    #[test]
    fn test_bytes_to_hex() {
        let bytes = vec![0x12, 0x34, 0x56, 0x78];
        let hex = bytes_to_hex(&bytes);

        assert_eq!(hex, "12345678");
    }

    #[test]
    fn test_hex_to_bytes() {
        let hex = "12345678";
        let bytes = hex_to_bytes(hex).unwrap();

        assert_eq!(bytes, vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_hex_to_bytes_invalid() {
        assert!(hex_to_bytes("123").is_err()); // Odd length
        assert!(hex_to_bytes("zz").is_err()); // Invalid hex
    }

    #[test]
    fn test_generate_uuid() {
        let uuid1 = generate_uuid();
        let uuid2 = generate_uuid();

        assert!(!uuid1.is_empty());
        assert!(!uuid2.is_empty());
        assert_ne!(uuid1, uuid2);

        // Check UUID format (8-4-4-4-12 characters with hyphens)
        assert!(uuid1
            .matches(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$")
            .next()
            .is_some());
    }
}
