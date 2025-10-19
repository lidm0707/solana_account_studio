//! Formatting utilities for Surfdesk
//!
//! This module provides formatting functions for displaying Solana addresses,
//! balances, timestamps, and other data types in user-friendly formats.

use crate::services::solana_rpc::{lamports_to_sol, sol_to_lamports};

/// Format a Solana address for display (truncated)
pub fn format_address(address: &str) -> String {
    if address.len() > 20 {
        format!("{}...{}", &address[..8], &address[address.len() - 8..])
    } else {
        address.to_string()
    }
}

/// Format address with full length option
pub fn format_address_full(address: &str, truncate: bool) -> String {
    if truncate && address.len() > 20 {
        format_address(address)
    } else {
        address.to_string()
    }
}

/// Format balance from lamports to SOL with decimal places
pub fn format_balance(lamports: u64, decimals: u32) -> String {
    let sol = lamports_to_sol(lamports);
    format!("{:.1$}", sol, decimals as usize)
}

/// Format balance with currency symbol
pub fn format_balance_with_currency(lamports: u64, decimals: u32) -> String {
    format!("{} SOL", format_balance(lamports, decimals))
}

/// Format lamports as human readable string
pub fn format_lamports(lamports: u64) -> String {
    if lamports >= 1_000_000_000 {
        format!("{:.4} SOL", lamports as f64 / 1_000_000_000.0)
    } else if lamports >= 1_000_000 {
        format!("{:.2} M lamports", lamports as f64 / 1_000_000.0)
    } else if lamports >= 1_000 {
        format!("{:.1} K lamports", lamports as f64 / 1_000.0)
    } else {
        format!("{} lamports", lamports)
    }
}

/// Format timestamp to human readable string
pub fn format_timestamp(timestamp: &str) -> String {
    match chrono::DateTime::parse_from_rfc3339(timestamp) {
        Ok(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        Err(_) => {
            // Try parsing as Unix timestamp
            match timestamp.parse::<i64>() {
                Ok(seconds) => {
                    let dt = chrono::DateTime::from_timestamp(seconds, 0)
                        .unwrap_or_else(|| chrono::Utc::now().into());
                    dt.format("%Y-%m-%d %H:%M:%S").to_string()
                }
                Err(_) => timestamp.to_string(),
            }
        }
    }
}

/// Format timestamp as relative time (e.g., "2 hours ago")
pub fn format_relative_time(timestamp: &str) -> String {
    let now = chrono::Utc::now();
    let past_time = match chrono::DateTime::parse_from_rfc3339(timestamp) {
        Ok(dt) => dt.with_timezone(&chrono::Utc),
        Err(_) => match timestamp.parse::<i64>() {
            Ok(seconds) => chrono::DateTime::from_timestamp(seconds, 0).unwrap_or_else(|| now),
            Err(_) => return timestamp.to_string(),
        },
    };

    let duration = now.signed_duration_since(past_time);

    if duration.num_days() > 0 {
        format!(
            "{} day{} ago",
            duration.num_days(),
            if duration.num_days() != 1 { "s" } else { "" }
        )
    } else if duration.num_hours() > 0 {
        format!(
            "{} hour{} ago",
            duration.num_hours(),
            if duration.num_hours() != 1 { "s" } else { "" }
        )
    } else if duration.num_minutes() > 0 {
        format!(
            "{} minute{} ago",
            duration.num_minutes(),
            if duration.num_minutes() != 1 { "s" } else { "" }
        )
    } else if duration.num_seconds() > 0 {
        format!(
            "{} second{} ago",
            duration.num_seconds(),
            if duration.num_seconds() != 1 { "s" } else { "" }
        )
    } else {
        "Just now".to_string()
    }
}

/// Format file size in human readable format
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Format percentage with decimal places
pub fn format_percentage(value: f64, decimals: u32) -> String {
    format!("{:.1$}%", value * 100.0, decimals as usize)
}

/// Format transaction signature (truncate)
pub fn format_signature(signature: &str) -> String {
    if signature.len() > 16 {
        format!(
            "{}...{}",
            &signature[..8],
            &signature[signature.len() - 8..]
        )
    } else {
        signature.to_string()
    }
}

/// Format network status
pub fn format_network_status(is_connected: bool, network: &str) -> String {
    if is_connected {
        format!("Connected to {}", network)
    } else {
        "Disconnected".to_string()
    }
}

/// Format program size
pub fn format_program_size(bytes: u64) -> String {
    format_file_size(bytes)
}

/// Format slot number
pub fn format_slot(slot: u64) -> String {
    if slot >= 1_000_000 {
        format!("{:.1}M", slot as f64 / 1_000_000.0)
    } else if slot >= 1_000 {
        format!("{:.1}K", slot as f64 / 1_000.0)
    } else {
        slot.to_string()
    }
}

/// Format duration in milliseconds to human readable
pub fn format_duration_ms(duration_ms: u64) -> String {
    if duration_ms >= 1000 {
        let seconds = duration_ms / 1000;
        let milliseconds = duration_ms % 1000;
        if seconds >= 60 {
            let minutes = seconds / 60;
            let remaining_seconds = seconds % 60;
            format!("{}m {}s", minutes, remaining_seconds)
        } else {
            format!("{}.{:03}s", seconds, milliseconds)
        }
    } else {
        format!("{}ms", duration_ms)
    }
}

/// Format gas/compute units
pub fn format_compute_units(units: u64) -> String {
    if units >= 1_000_000 {
        format!("{:.1}M CU", units as f64 / 1_000_000.0)
    } else if units >= 1_000 {
        format!("{:.1}K CU", units as f64 / 1_000.0)
    } else {
        format!("{} CU", units)
    }
}

/// Format error message for user display
pub fn format_error_message(error: &str) -> String {
    // Clean up common error patterns
    let cleaned = error
        .replace("Error: ", "")
        .replace("RPC error: ", "")
        .trim();

    // Capitalize first letter
    if let Some(first_char) = cleaned.chars().next() {
        let mut result = first_char.to_uppercase().collect::<String>();
        result.push_str(&cleaned[1..]);
        result
    } else {
        cleaned.to_string()
    }
}

/// Parse SOL amount from string
pub fn parse_sol_amount(amount_str: &str) -> Result<f64, String> {
    amount_str
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("Invalid amount: {}", amount_str))
        .and_then(|amount| {
            if amount >= 0.0 {
                Ok(amount)
            } else {
                Err("Amount cannot be negative".to_string())
            }
        })
}

/// Convert SOL string to lamports
pub fn sol_string_to_lamports(sol_str: &str) -> Result<u64, String> {
    let sol = parse_sol_amount(sol_str)?;
    Ok(sol_to_lamports(sol))
}

/// Format currency amount with proper decimal places
pub fn format_currency(amount: f64, symbol: &str, decimals: u32) -> String {
    format!(
        "{:.precision$} {}",
        amount,
        symbol,
        precision = decimals as usize
    )
}

/// Format a list of items as comma-separated string
pub fn format_list(items: &[String]) -> String {
    items.join(", ")
}

/// Format a list with "and" for the last item
pub fn format_list_with_and(items: &[String]) -> String {
    match items.len() {
        0 => String::new(),
        1 => items[0].clone(),
        2 => format!("{} and {}", items[0], items[1]),
        _ => {
            let mut result = items[..items.len() - 1].join(", ");
            result.push_str(&format!(", and {}", items[items.len() - 1]));
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_address() {
        let long_address = "11111111111111111111111111111111111111111";
        let short_address = "11111111";

        assert_eq!(format_address(long_address), "11111111...11111111");
        assert_eq!(format_address(short_address), "11111111");
    }

    #[test]
    fn test_format_balance() {
        assert_eq!(format_balance(1_500_000_000, 4), "1.5000");
        assert_eq!(format_balance(500_000_000, 2), "0.50");
    }

    #[test]
    fn test_format_lamports() {
        assert_eq!(format_lamports(1_500_000_000), "1.5000 SOL");
        assert_eq!(format_lamports(500_000_000), "500.0 M lamports");
        assert_eq!(format_lamports(50_000), "50.0 K lamports");
        assert_eq!(format_lamports(500), "500 lamports");
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(500), "500 B");
        assert_eq!(format_file_size(1_500), "1.5 KB");
        assert_eq!(format_file_size(1_500_000), "1.5 MB");
        assert_eq!(format_file_size(1_500_000_000), "1.5 GB");
    }

    #[test]
    fn test_format_percentage() {
        assert_eq!(format_percentage(0.1234, 2), "12.34%");
        assert_eq!(format_percentage(1.0, 0), "100%");
    }

    #[test]
    fn test_format_signature() {
        let long_sig = "1111111111111111111111111111111111111111111111111111111111111111";
        let short_sig = "11111111";

        assert_eq!(format_signature(long_sig), "11111111...11111111");
        assert_eq!(format_signature(short_sig), "11111111");
    }

    #[test]
    fn test_parse_sol_amount() {
        assert_eq!(parse_sol_amount("1.5"), Ok(1.5));
        assert!(parse_sol_amount("-1.0").is_err());
        assert!(parse_sol_amount("invalid").is_err());
    }

    #[test]
    fn test_sol_string_to_lamports() {
        assert_eq!(sol_string_to_lamports("1.5"), Ok(1_500_000_000));
        assert_eq!(sol_string_to_lamports("0.01"), Ok(10_000_000));
    }

    #[test]
    fn test_format_list() {
        let items = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(format_list(&items), "a, b, c");
    }

    #[test]
    fn test_format_list_with_and() {
        assert_eq!(format_list_with_and(&[]), "");
        assert_eq!(format_list_with_and(&["a".to_string()]), "a");
        assert_eq!(
            format_list_with_and(&["a".to_string(), "b".to_string()]),
            "a and b"
        );
        assert_eq!(
            format_list_with_and(&["a".to_string(), "b".to_string(), "c".to_string()]),
            "a, b, and c"
        );
    }
}
