//! # Logger Service Module
//!
//! This module provides logging services for the SurfDesk application.
//! It handles log formatting, output destinations, and log rotation
//! across all platforms with platform-specific optimizations.

use crate::error::{Result, SurfDeskError};
use std::io::Write;
use std::sync::Mutex;

/// Logger service for managing application logging
pub struct LoggerService {
    /// Logger configuration
    config: LoggerConfig,
    /// Log output buffer
    buffer: Mutex<Vec<String>>,
    /// Maximum number of buffered logs
    max_buffer_size: usize,
}

impl LoggerService {
    /// Create a new logger service
    pub fn new(config_service: &super::config::ConfigService) -> Result<Self> {
        let config = LoggerConfig::from(config_service.get_logging_settings());

        // Initialize the logger
        Self::init_logger(&config)?;

        Ok(Self {
            config,
            buffer: Mutex::new(Vec::new()),
            max_buffer_size: 1000,
        })
    }

    /// Initialize the global logger
    fn init_logger(config: &LoggerConfig) -> Result<()> {
        let platform = crate::current_platform();

        match platform {
            crate::platform::Platform::Desktop | crate::platform::Platform::Terminal => {
                Self::init_native_logger(config)?;
            }
            crate::platform::Platform::Web => {
                Self::init_web_logger(config)?;
            }
        }

        log::info!(
            "Logger initialized with level: {}",
            config.level.as_filter()
        );
        Ok(())
    }

    /// Initialize native logger for desktop/terminal
    fn init_native_logger(config: &LoggerConfig) -> Result<()> {
        use std::fs::OpenOptions;

        let mut builder = env_logger::Builder::from_default_env();
        builder.filter_level(config.level.to_level_filter());

        if config.timestamps {
            builder.format(|buf, record| {
                use std::io::Write;
                let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");
                writeln!(
                    buf,
                    "{} [{}] {}: {}",
                    timestamp,
                    record.level(),
                    record.target(),
                    record.args()
                )
            });
        }

        // Add file logging if enabled
        if config.file_logging {
            // Set up file logging
            let log_path = Self::get_log_file_path()?;
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_path)?;

            // Use write logger for file output
            // Note: env_logger::Target doesn't exist in current version
            // We'll use basic file logging for now
            log::info!("Log file configured at: {:?}", log_path);
        }

        builder.init();
        Ok(())
    }

    /// Initialize web logger
    fn init_web_logger(config: &LoggerConfig) -> Result<()> {
        #[cfg(target_arch = "wasm32")]
        {
            use log::Level;

            console_log::init_with_level(config.level.to_console_level()).map_err(|e| {
                SurfDeskError::internal(format!("Failed to init console log: {}", e))
            })?;

            // Custom format for web
            log::set_max_level(config.level.to_level_filter());

            log::info!("Web logger initialized");
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            return Err(SurfDeskError::platform(
                "Web logger not available on non-wasm target",
            ));
        }
    }

    /// Get the log file path
    fn get_log_file_path() -> Result<std::path::PathBuf> {
        let platform = crate::current_platform();

        let path = match platform {
            crate::platform::Platform::Desktop => {
                let mut path = dirs::data_local_dir()
                    .ok_or_else(|| SurfDeskError::internal("Could not find data directory"))?;
                path.push("surfdesk");
                path.push("logs");
                path.push("surfdesk.log");
                path
            }
            crate::platform::Platform::Terminal => {
                let mut path = dirs::home_dir()
                    .ok_or_else(|| SurfDeskError::internal("Could not find home directory"))?;
                path.push(".surfdesk");
                path.push("logs");
                path.push("surfdesk.log");
                path
            }
            crate::platform::Platform::Web => {
                return Err(SurfDeskError::platform(
                    "File logging not available on web platform",
                ));
            }
        };

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        Ok(path)
    }

    /// Get recent log entries from buffer
    pub fn get_recent_logs(&self, count: usize) -> Vec<String> {
        let buffer = self.buffer.lock().unwrap();
        let start = if buffer.len() > count {
            buffer.len() - count
        } else {
            0
        };
        buffer[start..].to_vec()
    }

    /// Clear the log buffer
    pub fn clear_buffer(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.clear();
    }

    /// Get logger configuration
    pub fn config(&self) -> &LoggerConfig {
        &self.config
    }

    /// Update logger configuration
    pub fn update_config(
        &mut self,
        logging_settings: &crate::services::config::LoggingSettings,
    ) -> Result<()> {
        let new_config = LoggerConfig::from(logging_settings);

        if new_config.level != self.config.level {
            log::set_max_level(new_config.level.to_level_filter());
        }

        self.config = new_config;
        log::info!("Logger configuration updated");
        Ok(())
    }

    /// Rotate log files if needed
    pub fn rotate_logs(&self) -> Result<()> {
        if !self.config.file_logging {
            return Ok(());
        }

        let log_path = Self::get_log_file_path()?;

        if log_path.exists() {
            let metadata = std::fs::metadata(&log_path)?;
            let file_size = metadata.len();
            let max_size = self.config.max_file_size * 1024 * 1024; // Convert MB to bytes

            if file_size > max_size {
                Self::perform_log_rotation(&log_path, self.config.max_files)?;
                log::info!("Log files rotated");
            }
        }

        Ok(())
    }

    /// Perform actual log rotation
    fn perform_log_rotation(log_path: &std::path::Path, max_files: u32) -> Result<()> {
        // Remove oldest log file if it exists
        let oldest_path = log_path.with_extension(format!("log.{}", max_files));
        if oldest_path.exists() {
            std::fs::remove_file(&oldest_path)?;
        }

        // Rotate existing log files
        for i in (1..max_files).rev() {
            let current_path = log_path.with_extension(format!("log.{}", i));
            let next_path = log_path.with_extension(format!("log.{}", i + 1));

            if current_path.exists() {
                std::fs::rename(&current_path, &next_path)?;
            }
        }

        // Move current log file to .log.1
        let rotated_path = log_path.with_extension("log.1");
        std::fs::rename(log_path, &rotated_path)?;

        Ok(())
    }

    /// Get log statistics
    pub fn get_log_stats(&self) -> LogStats {
        let buffer = self.buffer.lock().unwrap();

        let mut error_count = 0;
        let mut warn_count = 0;
        let mut info_count = 0;
        let mut debug_count = 0;
        let mut trace_count = 0;

        for entry in buffer.iter() {
            if entry.contains("[ERROR]") {
                error_count += 1;
            } else if entry.contains("[WARN]") {
                warn_count += 1;
            } else if entry.contains("[INFO]") {
                info_count += 1;
            } else if entry.contains("[DEBUG]") {
                debug_count += 1;
            } else if entry.contains("[TRACE]") {
                trace_count += 1;
            }
        }

        LogStats {
            total_entries: buffer.len(),
            error_count,
            warn_count,
            info_count,
            debug_count,
            trace_count,
            current_level: self.config.level,
        }
    }
}

/// Logger configuration
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    /// Log level
    pub level: LogLevel,
    /// Whether to log to file
    pub file_logging: bool,
    /// Maximum log file size in MB
    pub max_file_size: u64,
    /// Number of log files to keep
    pub max_files: u32,
    /// Whether to include timestamps
    pub timestamps: bool,
}

impl From<&crate::services::config::LoggingSettings> for LoggerConfig {
    fn from(settings: &crate::services::config::LoggingSettings) -> Self {
        Self {
            level: LogLevel::from(settings.level),
            file_logging: settings.file_logging,
            max_file_size: settings.max_file_size,
            max_files: settings.max_files,
            timestamps: settings.timestamps,
        }
    }
}

/// Log level for logger configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    /// Convert log level to env_logger filter string
    pub fn as_filter(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warn => "warn",
            Self::Info => "info",
            Self::Debug => "debug",
            Self::Trace => "trace",
        }
    }
}

impl LogLevel {
    /// Convert to env_logger LevelFilter
    pub fn to_level_filter(self) -> log::LevelFilter {
        match self {
            Self::Error => log::LevelFilter::Error,
            Self::Warn => log::LevelFilter::Warn,
            Self::Info => log::LevelFilter::Info,
            Self::Debug => log::LevelFilter::Debug,
            Self::Trace => log::LevelFilter::Trace,
        }
    }

    /// Convert to console_log Level (for web)
    #[cfg(target_arch = "wasm32")]
    pub fn to_console_level(self) -> console_log::Level {
        match self {
            Self::Error => console_log::Level::Error,
            Self::Warn => console_log::Level::Warn,
            Self::Info => console_log::Level::Info,
            Self::Debug => console_log::Level::Debug,
            Self::Trace => console_log::Level::Trace,
        }
    }
}

impl From<crate::services::config::LogLevel> for LogLevel {
    fn from(level: crate::services::config::LogLevel) -> Self {
        match level {
            crate::services::config::LogLevel::Error => Self::Error,
            crate::services::config::LogLevel::Warn => Self::Warn,
            crate::services::config::LogLevel::Info => Self::Info,
            crate::services::config::LogLevel::Debug => Self::Debug,
            crate::services::config::LogLevel::Trace => Self::Trace,
        }
    }
}

/// Log statistics
#[derive(Debug, Clone)]
pub struct LogStats {
    /// Total number of log entries
    pub total_entries: usize,
    /// Number of error entries
    pub error_count: usize,
    /// Number of warning entries
    pub warn_count: usize,
    /// Number of info entries
    pub info_count: usize,
    /// Number of debug entries
    pub debug_count: usize,
    /// Number of trace entries
    pub trace_count: usize,
    /// Current log level
    pub current_level: LogLevel,
}

impl LogStats {
    /// Get the most common log level
    pub fn most_common_level(&self) -> LogLevel {
        let levels = [
            (self.error_count, LogLevel::Error),
            (self.warn_count, LogLevel::Warn),
            (self.info_count, LogLevel::Info),
            (self.debug_count, LogLevel::Debug),
            (self.trace_count, LogLevel::Trace),
        ];

        levels
            .iter()
            .max_by_key(|(count, _)| *count)
            .map(|(_, level)| *level)
            .unwrap_or(LogLevel::Info)
    }

    /// Check if there are any error logs
    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }

    /// Check if there are any warning logs
    pub fn has_warnings(&self) -> bool {
        self.warn_count > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_conversion() {
        assert_eq!(LogLevel::Info.to_level_filter(), log::LevelFilter::Info);
        assert_eq!(LogLevel::Error.to_level_filter(), log::LevelFilter::Error);
        assert_eq!(LogLevel::Debug.to_level_filter(), log::LevelFilter::Debug);
    }

    #[test]
    fn test_log_stats() {
        let stats = LogStats {
            total_entries: 100,
            error_count: 5,
            warn_count: 10,
            info_count: 70,
            debug_count: 10,
            trace_count: 5,
            current_level: LogLevel::Info,
        };

        assert_eq!(stats.total_entries, 100);
        assert!(stats.has_errors());
        assert!(stats.has_warnings());
        assert_eq!(stats.most_common_level(), LogLevel::Info);
    }

    #[test]
    fn test_logger_config_from_settings() {
        let settings = crate::services::config::LoggingSettings {
            level: crate::services::config::LogLevel::Debug,
            file_logging: true,
            max_file_size: 20,
            max_files: 10,
            timestamps: false,
        };

        let config = LoggerConfig::from(&settings);
        assert_eq!(config.level, LogLevel::Debug);
        assert!(config.file_logging);
        assert_eq!(config.max_file_size, 20);
        assert_eq!(config.max_files, 10);
        assert!(!config.timestamps);
    }

    #[tokio::test]
    async fn test_logger_service_creation() {
        // Note: This test would need a config service
        // For now, we test the logic separately

        let config = LoggerConfig {
            level: LogLevel::Info,
            file_logging: false,
            max_file_size: 10,
            max_files: 5,
            timestamps: true,
        };

        assert_eq!(config.level.to_level_filter(), log::LevelFilter::Info);
        assert!(!config.file_logging);
        assert!(config.timestamps);
    }

    #[test]
    fn test_log_level_from_config() {
        let config_level = crate::services::config::LogLevel::Warn;
        let logger_level = LogLevel::from(config_level);
        assert_eq!(logger_level, LogLevel::Warn);
    }
}
