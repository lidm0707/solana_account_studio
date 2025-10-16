//! # Platform Detection and Utilities Module
//!
//! This module provides platform detection and platform-specific utilities
//! for the SurfDesk application. It abstracts platform differences and provides
//! a unified interface across desktop, web, and terminal platforms.

use crate::error::Result;
use std::collections::HashMap;

use std::fmt;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Platform enum representing the current runtime platform
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Platform {
    /// Desktop platform (native application)
    Desktop,
    /// Web platform (browser/WebAssembly)
    Web,
    /// Terminal platform (TUI)
    Terminal,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Platform::Desktop => write!(f, "Desktop"),
            Platform::Web => write!(f, "Web"),
            Platform::Terminal => write!(f, "Terminal"),
        }
    }
}

impl Platform {
    /// Get the current platform at runtime
    pub fn current() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            Self::Web
        }

        #[cfg(all(not(target_arch = "wasm32"), feature = "tui"))]
        {
            Self::Terminal
        }

        #[cfg(all(not(target_arch = "wasm32"), not(feature = "tui")))]
        {
            // Check if we're in a terminal environment
            if Self::is_terminal_environment() {
                Self::Terminal
            } else {
                Self::Desktop
            }
        }
    }

    /// Get the display name for the platform
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Desktop => "Desktop",
            Self::Web => "Web",
            Self::Terminal => "Terminal",
        }
    }

    /// Get CSS class for platform-specific styling
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Desktop => "platform-desktop",
            Self::Web => "platform-web",
            Self::Terminal => "platform-terminal",
        }
    }

    /// Check if the platform supports native file dialogs
    pub fn supports_native_file_dialogs(&self) -> bool {
        matches!(self, Self::Desktop)
    }

    /// Check if the platform supports system tray
    pub fn supports_system_tray(&self) -> bool {
        matches!(self, Self::Desktop)
    }

    /// Check if the platform supports hot reload
    pub fn supports_hot_reload(&self) -> bool {
        match self {
            Self::Desktop => cfg!(debug_assertions),
            Self::Web => cfg!(debug_assertions),
            Self::Terminal => false,
        }
    }

    /// Check if the platform supports clipboard access
    pub fn supports_clipboard(&self) -> bool {
        match self {
            Self::Desktop => true,
            Self::Web => Self::web_clipboard_supported(),
            Self::Terminal => false,
        }
    }

    /// Check if the platform supports notifications
    pub fn supports_notifications(&self) -> bool {
        match self {
            Self::Desktop => true,
            Self::Web => Self::web_notifications_supported(),
            Self::Terminal => false,
        }
    }

    /// Check if the platform supports multiple windows
    pub fn supports_multiple_windows(&self) -> bool {
        matches!(self, Self::Desktop)
    }

    /// Get platform-specific default settings
    pub fn default_settings(&self) -> PlatformSettings {
        match self {
            Self::Desktop => PlatformSettings {
                auto_save: true,
                auto_save_interval: 30,
                theme: crate::types::Theme::Auto,
                native_window_controls: true,
                system_tray: true,
                multiple_windows: true,
                keyboard_shortcuts: true,
                file_associations: true,
                auto_update: true,
            },
            Self::Web => PlatformSettings {
                auto_save: true,
                auto_save_interval: 60,
                theme: crate::types::Theme::Auto,
                native_window_controls: false,
                system_tray: false,
                multiple_windows: false,
                keyboard_shortcuts: true,
                file_associations: false,
                auto_update: false,
            },
            Self::Terminal => PlatformSettings {
                auto_save: false,
                auto_save_interval: 0,
                theme: crate::types::Theme::Dark,
                native_window_controls: false,
                system_tray: false,
                multiple_windows: false,
                keyboard_shortcuts: true,
                file_associations: false,
                auto_update: false,
            },
        }
    }

    /// Detect if running in terminal environment
    fn is_terminal_environment() -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Check for common terminal environment variables
            std::env::var("TERM").is_ok() ||
            std::env::var("TTY").is_ok() ||
            std::env::var("SSH_TTY").is_ok() ||
            // Check if stdout is a TTY
            unsafe { libc::isatty(libc::STDOUT_FILENO) != 0 }
        }

        #[cfg(target_arch = "wasm32")]
        {
            false // Web platform is never terminal
        }
    }

    /// Check if web clipboard is supported
    fn web_clipboard_supported() -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            // Check for Clipboard API support
            web_sys::window()
                .and_then(|w| w.navigator().clipboard().ok())
                .is_some()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            false
        }
    }

    /// Check if web notifications are supported
    fn web_notifications_supported() -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            // Check for Notification API support
            web_sys::window()
                .and_then(|w| w.navigator().notification().ok())
                .is_some()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            false
        }
    }
}

/// Platform-specific settings
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlatformSettings {
    /// Whether auto-save is enabled by default
    pub auto_save: bool,
    /// Auto-save interval in seconds
    pub auto_save_interval: u64,
    /// Default theme
    pub theme: crate::types::Theme,
    /// Whether native window controls are enabled
    pub native_window_controls: bool,
    /// Whether system tray is enabled
    pub system_tray: bool,
    /// Whether multiple windows are supported
    pub multiple_windows: bool,
    /// Whether keyboard shortcuts are enabled
    pub keyboard_shortcuts: bool,
    /// Whether file associations are enabled
    pub file_associations: bool,
    /// Whether auto-update is enabled
    pub auto_update: bool,
}

/// Platform-specific utilities and adapters
pub struct PlatformAdapter;

impl PlatformAdapter {
    /// Initialize platform-specific features
    pub async fn initialize() -> Result<()> {
        let platform = Platform::current();
        log::info!("Initializing platform: {}", platform.display_name());

        match platform {
            Platform::Desktop => Self::init_desktop().await?,
            Platform::Web => Self::init_web().await?,
            Platform::Terminal => Self::init_terminal().await?,
        }

        Ok(())
    }

    /// Initialize desktop-specific features
    async fn init_desktop() -> Result<()> {
        log::debug!("Initializing desktop platform features");

        // Set up native window handling
        // Configure system tray if supported
        // Set up file associations
        // Initialize auto-updater

        Ok(())
    }

    /// Initialize web-specific features
    async fn init_web() -> Result<()> {
        log::debug!("Initializing web platform features");

        // Set up service worker
        // Configure PWA features
        // Set up local storage
        // Initialize WebAssembly features

        Ok(())
    }

    /// Initialize terminal-specific features
    async fn init_terminal() -> Result<()> {
        log::debug!("Initializing terminal platform features");

        // Set up signal handlers for graceful shutdown
        // Configure terminal capabilities
        // Initialize color support
        // Set up raw mode if needed

        Ok(())
    }

    /// Get terminal dimensions
    fn get_terminal_size() -> Option<(u16, u16)> {
        #[cfg(all(unix, not(target_arch = "wasm32")))]
        {
            use std::fs::File;
            use std::os::unix::io::AsRawFd;

            if let Ok(mut file) = File::open("/dev/tty") {
                use libc::{ioctl, winsize, TIOCGWINSZ};
                use std::mem;

                unsafe {
                    let mut size: winsize = mem::zeroed();
                    if ioctl(file.as_raw_fd(), TIOCGWINSZ, &mut size) == 0 {
                        return Some((size.ws_col, size.ws_row));
                    }
                }
            }
        }

        None
    }

    /// Check if terminal supports colors
    fn supports_colors() -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::env::var("COLORTERM")
                .or_else(|_| std::env::var("TERM"))
                .map(|term| term.contains("color") || term.contains("256"))
                .unwrap_or(false)
        }
        #[cfg(target_arch = "wasm32")]
        {
            false
        }
    }

    /// Get system information
    fn get_system_info() -> HashMap<String, String> {
        let mut info = HashMap::new();

        // Basic system info
        info.insert("os".to_string(), std::env::consts::OS.to_string());
        info.insert("arch".to_string(), std::env::consts::ARCH.to_string());
        info.insert("family".to_string(), std::env::consts::FAMILY.to_string());

        // Platform-specific info
        let platform = Platform::current();
        info.insert("platform".to_string(), platform.display_name().to_string());

        #[cfg(target_arch = "wasm32")]
        {
            if let Ok(window) = web_sys::window() {
                if let Ok(user_agent) = window.navigator().user_agent() {
                    info.insert("user_agent".to_string(), user_agent);
                }

                if let Ok(lang) = window.navigator().language() {
                    info.insert("language".to_string(), lang);
                }
            }
        }

        /// Get the current platform at runtime (convenience function)
        pub fn current_platform() -> Platform {
            Platform::current()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(hostname) = std::env::var("HOSTNAME") {
                info.insert("hostname".to_string(), hostname);
            }

            if let Ok(username) = std::env::var("USER").or_else(|_| std::env::var("USERNAME")) {
                info.insert("username".to_string(), username);
            }
        }

        info
    }

    /// Set up platform-specific event listeners
    pub fn setup_event_listeners() -> Result<()> {
        let platform = Platform::current();

        match platform {
            Platform::Desktop => {
                // Set up window event listeners
                // Set up system tray event listeners
                // Set up global keyboard shortcuts
            }
            Platform::Web => {
                // Set up beforeunload event listener
                // Set up online/offline event listeners
                // Set up visibility change event listener
            }
            Platform::Terminal => {
                // Set up signal handlers
                // Set up terminal resize handlers
            }
        }

        Ok(())
    }

    /// Handle platform-specific cleanup
    pub async fn cleanup() -> Result<()> {
        let platform = Platform::current();
        log::info!("Cleaning up platform: {}", platform.display_name());

        match platform {
            Platform::Desktop => {
                // Clean up native resources
                // Hide system tray
                // Save window state
            }
            Platform::Web => {
                // Save data to localStorage
                // Unregister service workers if needed
            }
            Platform::Terminal => {
                // Restore terminal settings
                // Clean up signal handlers
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_display_name() {
        assert_eq!(Platform::Desktop.display_name(), "Desktop");
        assert_eq!(Platform::Web.display_name(), "Web");
        assert_eq!(Platform::Terminal.display_name(), "Terminal");
    }

    #[test]
    fn test_platform_css_class() {
        assert_eq!(Platform::Desktop.css_class(), "platform-desktop");
        assert_eq!(Platform::Web.css_class(), "platform-web");
        assert_eq!(Platform::Terminal.css_class(), "platform-terminal");
    }

    #[test]
    fn test_platform_capabilities() {
        assert!(Platform::Desktop.supports_native_file_dialogs());
        assert!(Platform::Desktop.supports_system_tray());
        assert!(Platform::Desktop.supports_multiple_windows());

        assert!(!Platform::Web.supports_native_file_dialogs());
        assert!(!Platform::Web.supports_system_tray());
        assert!(!Platform::Web.supports_multiple_windows());

        assert!(!Platform::Terminal.supports_native_file_dialogs());
        assert!(!Platform::Terminal.supports_system_tray());
        assert!(!Platform::Terminal.supports_multiple_windows());
    }

    #[test]
    fn test_platform_default_settings() {
        let desktop_settings = Platform::Desktop.default_settings();
        assert!(desktop_settings.auto_save);
        assert!(desktop_settings.native_window_controls);
        assert!(desktop_settings.system_tray);

        let web_settings = Platform::Web.default_settings();
        assert!(web_settings.auto_save);
        assert!(!web_settings.native_window_controls);
        assert!(!web_settings.system_tray);

        let terminal_settings = Platform::Terminal.default_settings();
        assert!(!terminal_settings.auto_save);
        assert!(!terminal_settings.native_window_controls);
        assert!(!terminal_settings.system_tray);
        assert_eq!(terminal_settings.theme, crate::types::Theme::Dark);
    }

    #[test]
    fn test_system_info() {
        let info = PlatformAdapter::get_system_info();
        assert!(info.contains_key("os"));
        assert!(info.contains_key("arch"));
        assert!(info.contains_key("family"));
        assert!(info.contains_key("platform"));
    }

    #[tokio::test]
    async fn test_platform_adapter_init() {
        let result = PlatformAdapter::initialize().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_platform_adapter_cleanup() {
        let result = PlatformAdapter::cleanup().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_web_capabilities() {
        #[cfg(target_arch = "wasm32")]
        {
            // These would need to be tested in a browser environment
            // For now, we just test that the functions don't panic
            let _ = Platform::web_clipboard_supported();
            let _ = Platform::web_notifications_supported();
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            assert!(!Platform::web_clipboard_supported());
            assert!(!Platform::web_notifications_supported());
        }
    }
}
