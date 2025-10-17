// Styles module for SurfDesk
//! This module contains style-related utilities and constants for the SurfDesk application.

// Re-export commonly used style items
pub use std::collections::HashMap;

/// Style constants and utilities
pub mod constants {
    /// Default font family
    pub const FONT_FAMILY: &str = "system-ui, -apple-system, sans-serif";

    /// Default border radius
    pub const BORDER_RADIUS: &str = "8px";

    /// Default spacing unit
    pub const SPACING_UNIT: &str = "1rem";

    /// Animation duration
    pub const ANIMATION_DURATION: &str = "0.2s";
}

/// CSS class names used throughout the application
pub mod classes {
    pub const SURFDESK_DESKTOP: &str = "surfdesk-desktop";
    pub const MENU_BAR: &str = "menu-bar";
    pub const SIDEBAR: &str = "sidebar";
    pub const PAGE_CONTAINER: &str = "page-container";
    pub const BUTTON: &str = "button";
    pub const CARD: &str = "card";
    pub const INPUT: &str = "input";

    // Status classes
    pub const STATUS_RUNNING: &str = "status-running";
    pub const STATUS_STOPPED: &str = "status-stopped";
    pub const STATUS_ERROR: &str = "status-error";
    pub const STATUS_STARTING: &str = "status-starting";
    pub const STATUS_STOPPING: &str = "status-stopping";
}

/// Theme utilities
pub mod theme {
    use std::collections::HashMap;

    /// Get CSS variables for a theme
    pub fn get_theme_variables(theme: &str) -> HashMap<String, String> {
        let mut variables = HashMap::new();

        match theme {
            "dark" => {
                variables.insert("background".to_string(), "#1a1a1a".to_string());
                variables.insert("foreground".to_string(), "#ffffff".to_string());
                variables.insert("primary".to_string(), "#0084ff".to_string());
                variables.insert("secondary".to_string(), "#6c757d".to_string());
            }
            "light" => {
                variables.insert("background".to_string(), "#ffffff".to_string());
                variables.insert("foreground".to_string(), "#000000".to_string());
                variables.insert("primary".to_string(), "#0066cc".to_string());
                variables.insert("secondary".to_string(), "#6c757d".to_string());
            }
            _ => {
                // Auto theme (default)
                variables.insert("background".to_string(), "#ffffff".to_string());
                variables.insert("foreground".to_string(), "#000000".to_string());
                variables.insert("primary".to_string(), "#0066cc".to_string());
                variables.insert("secondary".to_string(), "#6c757d".to_string());
            }
        }

        variables
    }
}

/// Include all styles for the application
pub fn include_all_styles() -> String {
    r#"
.surfdesk-desktop {
    font-family: system-ui, -apple-system, sans-serif;
    background: #ffffff;
    color: #000000;
    margin: 0;
    padding: 0;
}

.menu-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 16px;
    background: #f8f9fa;
    border-bottom: 1px solid #dee2e6;
}

.sidebar {
    width: 250px;
    background: #f8f9fa;
    border-right: 1px solid #dee2e6;
    padding: 16px;
}

.page-container {
    flex: 1;
    padding: 16px;
    overflow-y: auto;
}

.button {
    padding: 8px 16px;
    border: 1px solid #ced4da;
    border-radius: 4px;
    background: #ffffff;
    cursor: pointer;
    transition: background-color 0.2s;
}

.button:hover {
    background: #e9ecef;
}

.card {
    border: 1px solid #dee2e6;
    border-radius: 8px;
    padding: 16px;
    margin-bottom: 16px;
    background: #ffffff;
}

.input {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid #ced4da;
    border-radius: 4px;
    font-size: 14px;
}

.status-running {
    color: #28a745;
}

.status-stopped {
    color: #6c757d;
}

.status-error {
    color: #dc3545;
}

.status-starting {
    color: #ffc107;
}

.status-stopping {
    color: #fd7e14;
}
"#
    .to_string()
}
