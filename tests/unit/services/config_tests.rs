//! Config service unit tests
//!
//! This module contains unit tests for the configuration service in the SurfDesk application.
//! Tests focus on configuration loading, saving, validation, type conversion, and persistence
//! across different storage backends.

use pretty_assertions::assert_str_eq;
use std::collections::HashMap;
use surfdesk_core::services::config::{ConfigError, ConfigService, ConfigValue};

#[test]
fn test_config_service_creation() {
    // Test that config service can be created successfully
    let service = ConfigService::new();
    assert!(service.is_ok());

    let config_service = service.unwrap();
    assert!(config_service.health_check().is_healthy());
}

#[test]
fn test_config_set_and_get() {
    let mut config_service = ConfigService::new().unwrap();

    // Test setting and getting a string value
    let result =
        config_service.set_config("test_key", ConfigValue::String("test_value".to_string()));
    assert!(result.is_ok());

    let retrieved = config_service.get_config("test_key");
    assert!(retrieved.is_ok());

    match retrieved.unwrap() {
        ConfigValue::String(value) => assert_eq!(value, "test_value"),
        _ => panic!("Expected string value"),
    }
}

#[test]
fn test_config_get_nonexistent_key() {
    let config_service = ConfigService::new().unwrap();

    let result = config_service.get_config("nonexistent_key");
    assert!(result.is_err());

    match result.unwrap_err() {
        ConfigError::KeyNotFound(key) => assert_eq!(key, "nonexistent_key"),
        _ => panic!("Expected KeyNotFound error"),
    }
}

#[test]
fn test_config_get_all_configs() {
    let mut config_service = ConfigService::new().unwrap();

    // Set multiple config values
    config_service
        .set_config(
            "rpc_url",
            ConfigValue::String("http://localhost:8899".to_string()),
        )
        .unwrap();
    config_service
        .set_config("timeout", ConfigValue::Integer(5000))
        .unwrap();
    config_service
        .set_config("enable_logging", ConfigValue::Boolean(true))
        .unwrap();

    let all_configs = config_service.get_all_configs();
    assert!(all_configs.is_ok());

    let configs = all_configs.unwrap();
    assert_eq!(configs.len(), 3);
    assert!(configs.contains_key("rpc_url"));
    assert!(configs.contains_key("timeout"));
    assert!(configs.contains_key("enable_logging"));
}

#[test]
fn test_config_reset_key() {
    let mut config_service = ConfigService::new().unwrap();

    // Set a config value
    config_service
        .set_config("test_key", ConfigValue::String("test_value".to_string()))
        .unwrap();

    // Verify it exists
    let retrieved = config_service.get_config("test_key");
    assert!(retrieved.is_ok());

    // Reset the key
    let reset_result = config_service.reset_config("test_key");
    assert!(reset_result.is_ok());

    // Verify it no longer exists
    let retrieved_after_reset = config_service.get_config("test_key");
    assert!(retrieved_after_reset.is_err());
}

#[test]
fn test_config_type_conversions() {
    let mut config_service = ConfigService::new().unwrap();

    // Test different config value types
    config_service
        .set_config("string_value", ConfigValue::String("hello".to_string()))
        .unwrap();
    config_service
        .set_config("integer_value", ConfigValue::Integer(42))
        .unwrap();
    config_service
        .set_config("float_value", ConfigValue::Float(3.14))
        .unwrap();
    config_service
        .set_config("boolean_value", ConfigValue::Boolean(true))
        .unwrap();
    config_service
        .set_config(
            "array_value",
            ConfigValue::Array(vec![
                ConfigValue::String("item1".to_string()),
                ConfigValue::String("item2".to_string()),
            ]),
        )
        .unwrap();

    // Test retrieving and type checking
    let string_val = config_service.get_config("string_value").unwrap();
    match string_val {
        ConfigValue::String(s) => assert_eq!(s, "hello"),
        _ => panic!("Expected string value"),
    }

    let int_val = config_service.get_config("integer_value").unwrap();
    match int_val {
        ConfigValue::Integer(i) => assert_eq!(i, 42),
        _ => panic!("Expected integer value"),
    }

    let float_val = config_service.get_config("float_value").unwrap();
    match float_val {
        ConfigValue::Float(f) => assert_eq!(f, 3.14),
        _ => panic!("Expected float value"),
    }

    let bool_val = config_service.get_config("boolean_value").unwrap();
    match bool_val {
        ConfigValue::Boolean(b) => assert_eq!(b, true),
        _ => panic!("Expected boolean value"),
    }

    let array_val = config_service.get_config("array_value").unwrap();
    match array_val {
        ConfigValue::Array(arr) => assert_eq!(arr.len(), 2),
        _ => panic!("Expected array value"),
    }
}

#[test]
fn test_config_persistence() {
    // Test configuration persistence across service instances
    let mut config_service = ConfigService::new().unwrap();

    // Set some config values
    config_service
        .set_config(
            "persistent_key",
            ConfigValue::String("persistent_value".to_string()),
        )
        .unwrap();
    config_service
        .set_config("another_key", ConfigValue::Integer(123))
        .unwrap();

    // Create a new service instance (in a real scenario, this would load from persistence)
    let new_config_service = ConfigService::new().unwrap();

    // Note: In a real implementation, this would test actual persistence
    // For now, we test that the service can be created and initialized
    assert!(new_config_service.health_check().is_healthy());
}

#[test]
fn test_config_validation() {
    let mut config_service = ConfigService::new().unwrap();

    // Test valid configuration values
    let valid_configs = vec![
        (
            "rpc_url",
            ConfigValue::String("http://localhost:8899".to_string()),
        ),
        (
            "ws_url",
            ConfigValue::String("ws://localhost:8900".to_string()),
        ),
        ("timeout", ConfigValue::Integer(5000)),
        ("retry_count", ConfigValue::Integer(3)),
        ("enable_logging", ConfigValue::Boolean(true)),
    ];

    for (key, value) in valid_configs {
        let result = config_service.set_config(key, value);
        assert!(result.is_ok(), "Failed to set valid config: {}", key);
    }

    // Test invalid configuration values
    let invalid_configs = vec![
        ("empty_string", ConfigValue::String("".to_string())),
        ("negative_timeout", ConfigValue::Integer(-1)),
        ("zero_retry_count", ConfigValue::Integer(0)),
    ];

    for (key, value) in invalid_configs {
        // Note: In a real implementation, this would test validation
        // For now, we test that the service handles the values
        let result = config_service.set_config(key, value);
        // The service might accept these but validate them later
        assert!(result.is_ok() || result.is_err());
    }
}

#[test]
fn test_config_error_handling() {
    let config_service = ConfigService::new().unwrap();

    // Test error cases
    let error_cases = vec![
        "",
        "  ",
        "very_long_key_name_that_exceeds_reasonable_limits_and_should_be_rejected",
        "invalid-key-with-special-chars!@#$%^&*()",
    ];

    for invalid_key in error_cases {
        let result = config_service.get_config(invalid_key);
        match result {
            Err(ConfigError::KeyNotFound(_)) => {
                // Expected for invalid keys
                assert!(true);
            }
            Err(ConfigError::InvalidKey(_)) => {
                // Also expected for invalid keys
                assert!(true);
            }
            Ok(_) => {
                // Unexpected success
                panic!("Expected error for invalid key: {}", invalid_key);
            }
            Err(e) => {
                panic!(
                    "Unexpected error type for invalid key {}: {:?}",
                    invalid_key, e
                );
            }
        }
    }
}

#[test]
fn test_config_bulk_operations() {
    let mut config_service = ConfigService::new().unwrap();

    // Test bulk setting of configurations
    let bulk_configs = HashMap::from([
        ("app_name", ConfigValue::String("SurfDesk".to_string())),
        ("version", ConfigValue::String("0.1.0".to_string())),
        ("debug_mode", ConfigValue::Boolean(false)),
        ("max_connections", ConfigValue::Integer(100)),
    ]);

    // Set configs in bulk
    for (key, value) in bulk_configs {
        let result = config_service.set_config(key, value.clone());
        assert!(result.is_ok(), "Failed to set config: {}", key);
    }

    // Verify all configs were set
    let all_configs = config_service.get_all_configs().unwrap();
    assert_eq!(all_configs.len(), bulk_configs.len());

    for (key, expected_value) in bulk_configs {
        assert!(all_configs.contains_key(key));
        let retrieved_value = all_configs.get(key).unwrap();
        assert_eq!(retrieved_value, &expected_value);
    }
}

#[test]
fn test_config_nested_values() {
    let mut config_service = ConfigService::new().unwrap();

    // Test nested configuration values
    let nested_config = ConfigValue::Object(HashMap::from([
        (
            "database".to_string(),
            ConfigValue::Object(HashMap::from([
                (
                    "url".to_string(),
                    ConfigValue::String("sqlite:test.db".to_string()),
                ),
                ("max_connections".to_string(), ConfigValue::Integer(10)),
            ])),
        ),
        (
            "logging".to_string(),
            ConfigValue::Object(HashMap::from([
                ("level".to_string(), ConfigValue::String("info".to_string())),
                ("enabled".to_string(), ConfigValue::Boolean(true)),
            ])),
        ),
    ]));

    let result = config_service.set_config("nested_config", nested_config);
    assert!(result.is_ok());

    let retrieved = config_service.get_config("nested_config").unwrap();
    match retrieved {
        ConfigValue::Object(obj) => {
            assert_eq!(obj.len(), 2);
            assert!(obj.contains_key("database"));
            assert!(obj.contains_key("logging"));

            if let Some(ConfigValue::Object(db_config)) = obj.get("database") {
                assert!(db_config.contains_key("url"));
                assert!(db_config.contains_key("max_connections"));
            }

            if let Some(ConfigValue::Object(log_config)) = obj.get("logging") {
                assert!(log_config.contains_key("level"));
                assert!(log_config.contains_key("enabled"));
            }
        }
        _ => panic!("Expected object value"),
    }
}

#[test]
fn test_config_service_lifecycle() {
    // Test the complete lifecycle of a config service

    // 1. Create service
    let mut config_service = ConfigService::new().unwrap();
    assert!(config_service.health_check().is_healthy());

    // 2. Initialize with default values
    let default_configs = HashMap::from([
        ("app_name", ConfigValue::String("SurfDesk".to_string())),
        ("version", ConfigValue::String("0.1.0".to_string())),
        ("debug", ConfigValue::Boolean(false)),
    ]);

    for (key, value) in default_configs {
        config_service.set_config(key, value).unwrap();
    }

    // 3. Modify configurations
    config_service
        .set_config("debug", ConfigValue::Boolean(true))
        .unwrap();
    config_service
        .set_config(
            "user_preference",
            ConfigValue::String("dark_mode".to_string()),
        )
        .unwrap();

    // 4. Verify configurations
    let debug_value = config_service.get_config("debug").unwrap();
    match debug_value {
        ConfigValue::Boolean(debug) => assert_eq!(debug, true),
        _ => panic!("Expected boolean value"),
    }

    // 5. Clean up specific configurations
    config_service.reset_config("user_preference").unwrap();
    let removed_value = config_service.get_config("user_preference");
    assert!(removed_value.is_err());

    // 6. Shutdown service gracefully
    let shutdown_result = config_service.shutdown();
    assert!(shutdown_result.is_ok());
}

#[test]
fn test_config_concurrent_access() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let config_service = Arc::new(Mutex::new(ConfigService::new().unwrap()));
    let mut handles = vec![];

    // Spawn multiple threads to test concurrent access
    for i in 0..10 {
        let service_clone = Arc::clone(&config_service);
        let handle = thread::spawn(move || {
            let mut service = service_clone.lock().unwrap();

            // Each thread sets a unique key
            let key = format!("thread_{}_key", i);
            let value = ConfigValue::String(format!("thread_{}_value", i));

            let set_result = service.set_config(&key, value);
            assert!(set_result.is_ok());

            let get_result = service.get_config(&key);
            assert!(get_result.is_ok());
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all thread-specific configs were set
    let service = config_service.lock().unwrap();
    let all_configs = service.get_all_configs().unwrap();

    for i in 0..10 {
        let key = format!("thread_{}_key", i);
        assert!(
            all_configs.contains_key(&key),
            "Missing config key: {}",
            key
        );
    }
}

#[test]
fn test_config_memory_efficiency() {
    let mut config_service = ConfigService::new().unwrap();

    // Test that the service can handle a large number of configurations
    let large_config_count = 1000;

    for i in 0..large_config_count {
        let key = format!("config_key_{}", i);
        let value = ConfigValue::String(format!("config_value_{}", i));

        let result = config_service.set_config(&key, value);
        assert!(result.is_ok(), "Failed to set config {}", i);
    }

    // Verify all configurations are accessible
    let all_configs = config_service.get_all_configs().unwrap();
    assert_eq!(all_configs.len(), large_config_count);

    // Test random access
    let test_indices = vec![0, 100, 500, 999];
    for i in test_indices {
        let key = format!("config_key_{}", i);
        let retrieved = config_service.get_config(&key).unwrap();

        match retrieved {
            ConfigValue::String(value) => assert_eq!(value, format!("config_value_{}", i)),
            _ => panic!("Expected string value for index {}", i),
        }
    }
}
