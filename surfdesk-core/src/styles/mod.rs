// Styles module for SurfDesk
//! This module contains style-related utilities and constants for the SurfDesk application.

// Re-export commonly used style items
pub use std::collections::HashMap;

/// 🎨 System Color Palette from system_color.md
pub mod colors {
    /// Glow — Neon Cyan (Crest / Highlight)
    pub const GLOW_CYAN: &str = "#00F5F0";

    /// Core Blue (Main Body of Wave)
    pub const CORE_BLUE: &str = "#00A8FF";

    /// Deep Blue (Shadow / Internal Depth)
    pub const DEEP_BLUE: &str = "#0066C9";

    /// Subtle Violet (Soft Purple Undertone)
    pub const SUBTLE_VIOLET: &str = "#8A3CFF";

    /// Magenta Base (Lower Wave / Warmth)
    pub const MAGENTA_BASE: &str = "#B33BFF";

    /// Background Glow — Teal / Green (Ambient Light)
    pub const BACKGROUND_GLOW: &str = "#0B6B5A";

    /// Deep Background — Dark Purple / Teal (Canvas)
    pub const DEEP_BACKGROUND: &str = "#100726";
}

/// Style constants and utilities
pub mod constants {
    use super::colors::*;

    /// Default font family
    pub const FONT_FAMILY: &str = "system-ui, -apple-system, sans-serif";

    /// Default border radius
    pub const BORDER_RADIUS: &str = "8px";

    /// Default spacing unit
    pub const SPACING_UNIT: &str = "1rem";

    /// Animation duration
    pub const ANIMATION_DURATION: &str = "0.2s";

    /// SurfDesk theme colors
    pub const PRIMARY_COLOR: &str = CORE_BLUE;
    pub const ACCENT_COLOR: &str = GLOW_CYAN;
    pub const BACKGROUND_COLOR: &str = DEEP_BACKGROUND;
    pub const SURFACE_COLOR: &str = DEEP_BLUE;
    pub const SECONDARY_COLOR: &str = SUBTLE_VIOLET;
    pub const HIGHLIGHT_COLOR: &str = MAGENTA_BASE;
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

    // 🎨 Color classes
    pub const GLOW_CYAN: &str = "glow-cyan";
    pub const CORE_BLUE: &str = "core-blue";
    pub const DEEP_BLUE: &str = "deep-blue";
    pub const SUBTLE_VIOLET: &str = "subtle-violet";
    pub const MAGENTA_BASE: &str = "magenta-base";
    pub const BACKGROUND_GLOW: &str = "background-glow";
    pub const DEEP_BACKGROUND: &str = "deep-background";

    // 🌊 SurfPool specific classes
    pub const SURFPOOL_CONTAINER: &str = "surfpool-container";
    pub const SURFPOOL_STATUS: &str = "surfpool-status";
    pub const SURFPOOL_LOGS: &str = "surfpool-logs";
    pub const SURFPOOL_CONTROLS: &str = "surfpool-controls";

    // Status classes
    pub const STATUS_RUNNING: &str = "status-running";
    pub const STATUS_STOPPED: &str = "status-stopped";
    pub const STATUS_ERROR: &str = "status-error";
    pub const STATUS_STARTING: &str = "status-starting";
    pub const STATUS_STOPPING: &str = "status-stopping";

    // Theme variant classes
    pub const THEME_DARK: &str = "theme-dark";
    pub const THEME_LIGHT: &str = "theme-light";
    pub const THEME_AUTO: &str = "theme-auto";
}

/// 🎨 Theme utilities with system colors
pub mod theme {
    use super::colors::*;
    use std::collections::HashMap;

    /// Get CSS variables for a theme using system colors
    pub fn get_theme_variables(theme: &str) -> HashMap<String, String> {
        let mut variables = HashMap::new();

        match theme {
            "dark" => {
                variables.insert("background".to_string(), DEEP_BACKGROUND.to_string());
                variables.insert("foreground".to_string(), GLOW_CYAN.to_string());
                variables.insert("primary".to_string(), CORE_BLUE.to_string());
                variables.insert("secondary".to_string(), SUBTLE_VIOLET.to_string());
                variables.insert("accent".to_string(), GLOW_CYAN.to_string());
                variables.insert("surface".to_string(), DEEP_BLUE.to_string());
                variables.insert("highlight".to_string(), MAGENTA_BASE.to_string());
                variables.insert("ambient".to_string(), BACKGROUND_GLOW.to_string());
            }
            "light" => {
                variables.insert("background".to_string(), "#ffffff".to_string());
                variables.insert("foreground".to_string(), DEEP_BACKGROUND.to_string());
                variables.insert("primary".to_string(), CORE_BLUE.to_string());
                variables.insert("secondary".to_string(), SUBTLE_VIOLET.to_string());
                variables.insert("accent".to_string(), GLOW_CYAN.to_string());
                variables.insert("surface".to_string(), "#f8f9fa".to_string());
                variables.insert("highlight".to_string(), MAGENTA_BASE.to_string());
                variables.insert("ambient".to_string(), BACKGROUND_GLOW.to_string());
            }
            _ => {
                // Auto theme (default - uses dark theme with system colors)
                variables.insert("background".to_string(), DEEP_BACKGROUND.to_string());
                variables.insert("foreground".to_string(), GLOW_CYAN.to_string());
                variables.insert("primary".to_string(), CORE_BLUE.to_string());
                variables.insert("secondary".to_string(), SUBTLE_VIOLET.to_string());
                variables.insert("accent".to_string(), GLOW_CYAN.to_string());
                variables.insert("surface".to_string(), DEEP_BLUE.to_string());
                variables.insert("highlight".to_string(), MAGENTA_BASE.to_string());
                variables.insert("ambient".to_string(), BACKGROUND_GLOW.to_string());
            }
        }

        variables
    }

    /// Generate CSS variables with system colors
    pub fn generate_css_variables() -> String {
        format!(
            r#"
:root {{
    /* 🌊 SurfDesk System Colors */
    --glow-cyan: {};
    --core-blue: {};
    --deep-blue: {};
    --subtle-violet: {};
    --magenta-base: {};
    --background-glow: {};
    --deep-background: {};

    /* Derived colors */
    --primary: var(--core-blue);
    --accent: var(--glow-cyan);
    --background: var(--deep-background);
    --surface: var(--deep-blue);
    --secondary: var(--subtle-violet);
    --highlight: var(--magenta-base);
    --ambient: var(--background-glow);
}}
"#,
            GLOW_CYAN,
            CORE_BLUE,
            DEEP_BLUE,
            SUBTLE_VIOLET,
            MAGENTA_BASE,
            BACKGROUND_GLOW,
            DEEP_BACKGROUND
        )
    }
}

/// 🎨 Include all styles for the application with system colors
pub fn include_all_styles() -> String {
    let css_variables = theme::generate_css_variables();

    format!(
        r#"
{}

/* 🌊 SurfDesk Base Styles */
.surfdesk-desktop {{
    font-family: system-ui, -apple-system, sans-serif;
    background: var(--deep-background);
    color: var(--glow-cyan);
    margin: 0;
    padding: 0;
    min-height: 100vh;
}}

/* 📱 Navigation Components */
.menu-bar {{
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 16px;
    background: var(--surface);
    border-bottom: 1px solid var(--core-blue);
    backdrop-filter: blur(10px);
}}

.sidebar {{
    width: 250px;
    background: var(--surface);
    border-right: 1px solid var(--core-blue);
    padding: 16px;
    box-shadow: 2px 0 10px rgba(0, 168, 255, 0.1);
}}

.page-container {{
    flex: 1;
    padding: 16px;
    overflow-y: auto;
    background: var(--background);
}}

/* 🎯 Interactive Components */
.button {{
    padding: 8px 16px;
    border: 1px solid var(--core-blue);
    border-radius: 4px;
    background: var(--surface);
    color: var(--glow-cyan);
    cursor: pointer;
    transition: all 0.2s ease;
    font-weight: 500;
}}

.button:hover {{
    background: var(--core-blue);
    box-shadow: 0 0 15px rgba(0, 245, 240, 0.3);
    transform: translateY(-1px);
}}

.button:active {{
    transform: translateY(0);
}}

.button.primary {{
    background: var(--core-blue);
    border-color: var(--glow-cyan);
}}

.button.primary:hover {{
    background: var(--glow-cyan);
    color: var(--deep-background);
    box-shadow: 0 0 20px rgba(0, 245, 240, 0.5);
}}

/* 📋 Card Components */
.card {{
    border: 1px solid var(--core-blue);
    border-radius: 8px;
    padding: 16px;
    margin-bottom: 16px;
    background: var(--surface);
    backdrop-filter: blur(10px);
    box-shadow: 0 4px 15px rgba(0, 102, 201, 0.1);
}}

.card:hover {{
    border-color: var(--glow-cyan);
    box-shadow: 0 6px 20px rgba(0, 245, 240, 0.2);
}}

/* 📝 Input Components */
.input {{
    width: 100%;
    padding: 8px 12px;
    border: 1px solid var(--core-blue);
    border-radius: 4px;
    font-size: 14px;
    background: var(--surface);
    color: var(--glow-cyan);
    transition: border-color 0.2s ease;
}}

.input:focus {{
    outline: none;
    border-color: var(--glow-cyan);
    box-shadow: 0 0 10px rgba(0, 245, 240, 0.3);
}}

/* 🌊 SurfPool Specific Styles */
.surfpool-container {{
    background: var(--surface);
    border: 1px solid var(--core-blue);
    border-radius: 8px;
    padding: 16px;
    margin-bottom: 16px;
}}

.surfpool-status {{
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    border-radius: 4px;
    background: var(--background);
}}

.surfpool-logs {{
    background: var(--deep-background);
    border: 1px solid var(--deep-blue);
    border-radius: 4px;
    padding: 12px;
    font-family: 'Courier New', monospace;
    font-size: 12px;
    max-height: 200px;
    overflow-y: auto;
}}

.surfpool-controls {{
    display: flex;
    gap: 8px;
    margin-top: 12px;
}}

/* 🚦 Status Colors */
.status-running {{
    color: #00ff88;
    text-shadow: 0 0 5px rgba(0, 255, 136, 0.5);
}}

.status-stopped {{
    color: var(--secondary);
}}

.status-error {{
    color: #ff4444;
    text-shadow: 0 0 5px rgba(255, 68, 68, 0.5);
}}

.status-starting {{
    color: var(--highlight);
    text-shadow: 0 0 5px rgba(179, 59, 255, 0.5);
}}

.status-stopping {{
    color: #ff8844;
    text-shadow: 0 0 5px rgba(255, 136, 68, 0.5);
}}

/* 🎨 Theme Variants */
.theme-dark {{
    --background: var(--deep-background);
    --foreground: var(--glow-cyan);
}}

.theme-light {{
    --background: #ffffff;
    --foreground: var(--deep-background);
}}

/* ✨ Animations */
@keyframes glow {{
    0% {{ box-shadow: 0 0 5px var(--glow-cyan); }}
    50% {{ box-shadow: 0 0 20px var(--glow-cyan), 0 0 30px var(--core-blue); }}
    100% {{ box-shadow: 0 0 5px var(--glow-cyan); }}
}}

.glow-effect {{
    animation: glow 2s ease-in-out infinite;
}}
"#,
        css_variables
    )
}
