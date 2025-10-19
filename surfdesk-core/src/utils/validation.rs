//! Validation utilities for Surfdesk
//!
//! This module provides validation functions for Solana addresses, amounts,
//! and other data types used throughout the application.

use crate::services::solana_rpc::validate_address;
use crate::utils::crypto::validate_address as crypto_validate_address;
use crate::utils::format::{parse_sol_amount, sol_string_to_lamports};

/// Validation result with error message
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    Invalid(String),
}

/// Validate a Solana address
pub fn validate_solana_address(address: &str) -> ValidationResult {
    if address.is_empty() {
        return ValidationResult::Invalid("Address cannot be empty".to_string());
    }

    if address.len() < 32 || address.len() > 44 {
        return ValidationResult::Invalid(
            "Address must be between 32 and 44 characters".to_string(),
        );
    }

    if !crypto_validate_address(address) {
        return ValidationResult::Invalid("Invalid Base58 address format".to_string());
    }

    ValidationResult::Valid
}

/// Validate SOL amount string
pub fn validate_sol_amount(amount: &str) -> ValidationResult {
    if amount.is_empty() {
        return ValidationResult::Invalid("Amount cannot be empty".to_string());
    }

    match parse_sol_amount(amount) {
        Ok(value) => {
            if value < 0.0 {
                ValidationResult::Invalid("Amount cannot be negative".to_string())
            } else if value > 50_000_000.0 {
                ValidationResult::Invalid("Amount exceeds maximum limit (50M SOL)".to_string())
            } else {
                ValidationResult::Valid
            }
        }
        Err(err) => ValidationResult::Invalid(err),
    }
}

/// Validate lamports amount
pub fn validate_lamports_amount(lamports: u64) -> ValidationResult {
    if lamports > 50_000_000 * 1_000_000_000 {
        ValidationResult::Invalid("Amount exceeds maximum limit".to_string())
    } else {
        ValidationResult::Valid
    }
}

/// Validate account label
pub fn validate_account_label(label: &str) -> ValidationResult {
    if label.is_empty() {
        return ValidationResult::Invalid("Label cannot be empty".to_string());
    }

    if label.len() > 50 {
        return ValidationResult::Invalid("Label cannot exceed 50 characters".to_string());
    }

    if label.trim() != label {
        return ValidationResult::Invalid("Label cannot start or end with whitespace".to_string());
    }

    // Check for invalid characters
    let invalid_chars = ['\n', '\r', '\t', '<', '>', '"', '\'', '\\'];
    for ch in label.chars() {
        if invalid_chars.contains(&ch) {
            return ValidationResult::Invalid(format!("Label contains invalid character: {}", ch));
        }
    }

    ValidationResult::Valid
}

/// Validate program name
pub fn validate_program_name(name: &str) -> ValidationResult {
    if name.is_empty() {
        return ValidationResult::Invalid("Program name cannot be empty".to_string());
    }

    if name.len() > 100 {
        return ValidationResult::Invalid("Program name cannot exceed 100 characters".to_string());
    }

    if !name.chars().next().unwrap().is_alphabetic() {
        return ValidationResult::Invalid("Program name must start with a letter".to_string());
    }

    // Check for valid characters (letters, numbers, spaces, hyphens, underscores)
    for ch in name.chars() {
        if !ch.is_alphanumeric() && ch != ' ' && ch != '-' && ch != '_' {
            return ValidationResult::Invalid(format!(
                "Program name contains invalid character: {}",
                ch
            ));
        }
    }

    ValidationResult::Valid
}

/// Validate program description
pub fn validate_program_description(description: &str) -> ValidationResult {
    if description.len() > 500 {
        return ValidationResult::Invalid("Description cannot exceed 500 characters".to_string());
    }

    ValidationResult::Valid
}

/// Validate transaction memo
pub fn validate_memo(memo: &str) -> ValidationResult {
    if memo.len() > 200 {
        return ValidationResult::Invalid("Memo cannot exceed 200 characters".to_string());
    }

    // Check for invalid characters that might cause issues
    let invalid_chars = ['\n', '\r', '\t'];
    for ch in memo.chars() {
        if invalid_chars.contains(&ch) {
            return ValidationResult::Invalid(format!("Memo contains invalid character: {}", ch));
        }
    }

    ValidationResult::Valid
}

/// Validate network URL
pub fn validate_network_url(url: &str) -> ValidationResult {
    if url.is_empty() {
        return ValidationResult::Invalid("URL cannot be empty".to_string());
    }

    if !url.starts_with("http://") && !url.starts_with("https://") {
        return ValidationResult::Invalid("URL must start with http:// or https://".to_string());
    }

    // Basic URL validation
    match url::Url::parse(url) {
        Ok(parsed_url) => {
            if parsed_url.host().is_none() {
                ValidationResult::Invalid("URL must have a valid host".to_string())
            } else {
                ValidationResult::Valid
            }
        }
        Err(_) => ValidationResult::Invalid("Invalid URL format".to_string()),
    }
}

/// Validate port number
pub fn validate_port(port: u16) -> ValidationResult {
    if port == 0 {
        return ValidationResult::Invalid("Port cannot be 0".to_string());
    }

    if port < 1024 {
        return ValidationResult::Invalid(
            "Port must be 1024 or higher (non-privileged ports)".to_string(),
        );
    }

    ValidationResult::Valid
}

/// Validate email address
pub fn validate_email(email: &str) -> ValidationResult {
    if email.is_empty() {
        return ValidationResult::Invalid("Email cannot be empty".to_string());
    }

    if !email.contains('@') || !email.contains('.') {
        return ValidationResult::Invalid("Invalid email format".to_string());
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return ValidationResult::Invalid("Invalid email format".to_string());
    }

    if parts[0].is_empty() || parts[1].is_empty() {
        return ValidationResult::Invalid("Invalid email format".to_string());
    }

    ValidationResult::Valid
}

/// Validate private key format (Base58)
pub fn validate_private_key(private_key: &str) -> ValidationResult {
    if private_key.is_empty() {
        return ValidationResult::Invalid("Private key cannot be empty".to_string());
    }

    // Check if it's valid Base58
    match bs58::decode(private_key).into_vec() {
        Ok(bytes) => {
            // Check for typical private key lengths (32, 64 bytes)
            if bytes.len() == 32 || bytes.len() == 64 {
                ValidationResult::Valid
            } else {
                ValidationResult::Invalid("Private key has invalid length".to_string())
            }
        }
        Err(_) => ValidationResult::Invalid("Private key must be in Base58 format".to_string()),
    }
}

/// Validate seed phrase (space-separated words)
pub fn validate_seed_phrase(seed_phrase: &str) -> ValidationResult {
    if seed_phrase.is_empty() {
        return ValidationResult::Invalid("Seed phrase cannot be empty".to_string());
    }

    let words: Vec<&str> = seed_phrase.trim().split_whitespace().collect();

    if words.len() != 12 && words.len() != 24 {
        return ValidationResult::Invalid("Seed phrase must be 12 or 24 words".to_string());
    }

    // Basic validation - check if all words contain only letters
    for word in words {
        if !word.chars().all(|c| c.is_alphabetic()) {
            return ValidationResult::Invalid(
                "Seed phrase words must contain only letters".to_string(),
            );
        }
    }

    ValidationResult::Valid
}

/// Validate file name
pub fn validate_file_name(file_name: &str) -> ValidationResult {
    if file_name.is_empty() {
        return ValidationResult::Invalid("File name cannot be empty".to_string());
    }

    if file_name.len() > 255 {
        return ValidationResult::Invalid("File name cannot exceed 255 characters".to_string());
    }

    // Check for invalid characters
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    for ch in file_name.chars() {
        if invalid_chars.contains(&ch) {
            return ValidationResult::Invalid(format!(
                "File name contains invalid character: {}",
                ch
            ));
        }
    }

    ValidationResult::Valid
}

/// Validate JSON string
pub fn validate_json(json_str: &str) -> ValidationResult {
    if json_str.is_empty() {
        return ValidationResult::Invalid("JSON cannot be empty".to_string());
    }

    match serde_json::from_str::<serde_json::Value>(json_str) {
        Ok(_) => ValidationResult::Valid,
        Err(err) => ValidationResult::Invalid(format!("Invalid JSON: {}", err)),
    }
}

/// Validate hex string
pub fn validate_hex_string(hex_str: &str) -> ValidationResult {
    if hex_str.is_empty() {
        return ValidationResult::Invalid("Hex string cannot be empty".to_string());
    }

    if hex_str.len() % 2 != 0 {
        return ValidationResult::Invalid("Hex string must have even length".to_string());
    }

    for ch in hex_str.chars() {
        if !ch.is_ascii_hexdigit() {
            return ValidationResult::Invalid(format!("Invalid hex character: {}", ch));
        }
    }

    ValidationResult::Valid
}

/// Validate that a value is within a range
pub fn validate_range<T: PartialOrd>(value: T, min: T, max: T) -> ValidationResult {
    if value < min {
        return ValidationResult::Invalid("Value is below minimum".to_string());
    }

    if value > max {
        return ValidationResult::Invalid("Value exceeds maximum".to_string());
    }

    ValidationResult::Valid
}

/// Validate string length
pub fn validate_string_length(string: &str, min: usize, max: usize) -> ValidationResult {
    if string.len() < min {
        return ValidationResult::Invalid(format!("String must be at least {} characters", min));
    }

    if string.len() > max {
        return ValidationResult::Invalid(format!("String cannot exceed {} characters", max));
    }

    ValidationResult::Valid
}

/// Validate that a string is not empty
pub fn validate_non_empty(string: &str) -> ValidationResult {
    if string.trim().is_empty() {
        ValidationResult::Invalid("Field cannot be empty".to_string())
    } else {
        ValidationResult::Valid
    }
}

/// Validate required field
pub fn validate_required<T>(value: Option<T>) -> ValidationResult {
    if value.is_none() {
        ValidationResult::Invalid("This field is required".to_string())
    } else {
        ValidationResult::Valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_solana_address() {
        let valid_address = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM";
        let invalid_address = "invalid_address";

        assert_eq!(
            validate_solana_address(valid_address),
            ValidationResult::Valid
        );
        assert!(matches!(
            validate_solana_address(invalid_address),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_sol_amount() {
        assert_eq!(validate_sol_amount("1.5"), ValidationResult::Valid);
        assert_eq!(validate_sol_amount("0"), ValidationResult::Valid);
        assert!(matches!(
            validate_sol_amount("-1"),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_sol_amount("invalid"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_account_label() {
        assert_eq!(
            validate_account_label("My Account"),
            ValidationResult::Valid
        );
        assert!(matches!(
            validate_account_label(""),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_account_label("  "),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_program_name() {
        assert_eq!(validate_program_name("My Program"), ValidationResult::Valid);
        assert_eq!(validate_program_name("program123"), ValidationResult::Valid);
        assert!(matches!(
            validate_program_name("123invalid"),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_program_name(""),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_network_url() {
        assert_eq!(
            validate_network_url("https://example.com"),
            ValidationResult::Valid
        );
        assert_eq!(
            validate_network_url("http://localhost:8899"),
            ValidationResult::Valid
        );
        assert!(matches!(
            validate_network_url("invalid"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_port() {
        assert_eq!(validate_port(8080), ValidationResult::Valid);
        assert_eq!(validate_port(8999), ValidationResult::Valid);
        assert!(matches!(validate_port(0), ValidationResult::Invalid(_)));
        assert!(matches!(validate_port(80), ValidationResult::Invalid(_)));
    }

    #[test]
    fn test_validate_email() {
        assert_eq!(validate_email("test@example.com"), ValidationResult::Valid);
        assert!(matches!(
            validate_email("invalid"),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(validate_email(""), ValidationResult::Invalid(_)));
    }

    #[test]
    fn test_validate_string_length() {
        assert_eq!(
            validate_string_length("hello", 1, 10),
            ValidationResult::Valid
        );
        assert!(matches!(
            validate_string_length("hello", 10, 20),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_string_length("very long string", 1, 5),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_non_empty() {
        assert_eq!(validate_non_empty("hello"), ValidationResult::Valid);
        assert!(matches!(
            validate_non_empty(""),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_non_empty("   "),
            ValidationResult::Invalid(_)
        ));
    }
}
