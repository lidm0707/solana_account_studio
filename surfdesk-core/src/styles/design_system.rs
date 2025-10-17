#![allow(dead_code)]
//! # Design System Module
//!
//! Core design system variables and utilities for SurfDesk application.
//! Provides consistent theming, colors, spacing, and typography across all platforms.

#![allow(dead_code)]

// use dioxus::prelude::*;  // Currently unused

/// CSS variables for design system
pub const CSS_VARIABLES: &str = include_str!("design-system.css");

/// Color palette
pub mod colors {
    /// Primary colors
    pub const PRIMARY_50: &str = "#eff6ff";
    pub const PRIMARY_100: &str = "#dbeafe";
    pub const PRIMARY_200: &str = "#bfdbfe";
    pub const PRIMARY_300: &str = "#93c5fd";
    pub const PRIMARY_400: &str = "#60a5fa";
    pub const PRIMARY_500: &str = "#3b82f6";
    pub const PRIMARY_600: &str = "#2563eb";
    pub const PRIMARY_700: &str = "#1d4ed8";
    pub const PRIMARY_800: &str = "#1e40af";
    pub const PRIMARY_900: &str = "#1e3a8a";

    /// Gray colors
    pub const GRAY_50: &str = "#f9fafb";
    pub const GRAY_100: &str = "#f3f4f6";
    pub const GRAY_200: &str = "#e5e7eb";
    pub const GRAY_300: &str = "#d1d5db";
    pub const GRAY_400: &str = "#9ca3af";
    pub const GRAY_500: &str = "#6b7280";
    pub const GRAY_600: &str = "#4b5563";
    pub const GRAY_700: &str = "#374151";
    pub const GRAY_800: &str = "#1f2937";
    pub const GRAY_900: &str = "#111827";

    /// Success colors
    pub const SUCCESS_50: &str = "#f0fdf4";
    pub const SUCCESS_100: &str = "#dcfce7";
    pub const SUCCESS_500: &str = "#22c55e";
    pub const SUCCESS_600: &str = "#16a34a";
    pub const SUCCESS_700: &str = "#15803d";

    /// Error colors
    pub const ERROR_50: &str = "#fef2f2";
    pub const ERROR_100: &str = "#fee2e2";
    pub const ERROR_500: &str = "#ef4444";
    pub const ERROR_600: &str = "#dc2626";
    pub const ERROR_700: &str = "#b91c1c";

    /// Warning colors
    pub const WARNING_50: &str = "#fffbeb";
    pub const WARNING_100: &str = "#fef3c7";
    pub const WARNING_500: &str = "#f59e0b";
    pub const WARNING_600: &str = "#d97706";
    pub const WARNING_700: &str = "#b45309";

    /// Info colors
    pub const INFO_50: &str = "#eff6ff";
    pub const INFO_100: &str = "#dbeafe";
    pub const INFO_500: &str = "#3b82f6";
    pub const INFO_600: &str = "#2563eb";
    pub const INFO_700: &str = "#1d4ed8";
}

/// Spacing scale
pub mod spacing {
    pub const XS: &str = "0.25rem"; // 4px
    pub const SM: &str = "0.5rem"; // 8px
    pub const MD: &str = "0.75rem"; // 12px
    pub const LG: &str = "1rem"; // 16px
    pub const XL: &str = "1.25rem"; // 20px
    pub const XL2: &str = "1.5rem"; // 24px
    pub const XL3: &str = "1.875rem"; // 30px
    pub const XL4: &str = "2rem"; // 32px
    pub const XL5: &str = "2.5rem"; // 40px
    pub const XL6: &str = "3rem"; // 48px
}

/// Typography scale
pub mod typography {
    pub const TEXT_XS: &str = "0.75rem"; // 12px
    pub const TEXT_SM: &str = "0.875rem"; // 14px
    pub const TEXT_BASE: &str = "1rem"; // 16px
    pub const TEXT_LG: &str = "1.125rem"; // 18px
    pub const TEXT_XL: &str = "1.25rem"; // 20px
    pub const TEXT_2XL: &str = "1.5rem"; // 24px
    pub const TEXT_3XL: &str = "1.875rem"; // 30px
    pub const TEXT_4XL: &str = "2.25rem"; // 36px
    pub const TEXT_5XL: &str = "3rem"; // 48px

    /// Font weights
    pub const FONT_THIN: &str = "100";
    pub const FONT_LIGHT: &str = "300";
    pub const FONT_NORMAL: &str = "400";
    pub const FONT_MEDIUM: &str = "500";
    pub const FONT_SEMIBOLD: &str = "600";
    pub const FONT_BOLD: &str = "700";
    pub const FONT_EXTRABOLD: &str = "800";
    pub const FONT_BLACK: &str = "900";

    /// Line heights
    pub const LEADING_NONE: &str = "1";
    pub const LEADING_TIGHT: &str = "1.25";
    pub const LEADING_SNUG: &str = "1.375";
    pub const LEADING_NORMAL: &str = "1.5";
    pub const LEADING_RELAXED: &str = "1.625";
    pub const LEADING_LOOSE: &str = "2";
}

/// Border radius scale
pub mod radius {
    pub const NONE: &str = "0";
    pub const SM: &str = "0.125rem"; // 2px
    pub const DEFAULT: &str = "0.25rem"; // 4px
    pub const MD: &str = "0.375rem"; // 6px
    pub const LG: &str = "0.5rem"; // 8px
    pub const XL: &str = "0.75rem"; // 12px
    pub const XL2: &str = "1rem"; // 16px
    pub const XL3: &str = "1.5rem"; // 24px
    pub const FULL: &str = "9999px";
}

/// Shadows
pub mod shadows {
    pub const SM: &str = "0 1px 2px 0 rgba(0, 0, 0, 0.05)";
    pub const DEFAULT: &str = "0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06)";
    pub const MD: &str = "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)";
    pub const LG: &str = "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)";
    pub const XL: &str =
        "0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)";
    pub const XL2: &str = "0 25px 50px -12px rgba(0, 0, 0, 0.25)";
    pub const INNER: &str = "inset 0 2px 4px 0 rgba(0, 0, 0, 0.06)";
}

/// Transitions
pub mod transitions {
    pub const DURATION_75: &str = "75ms";
    pub const DURATION_100: &str = "100ms";
    pub const DURATION_150: &str = "150ms";
    pub const DURATION_200: &str = "200ms";
    pub const DURATION_300: &str = "300ms";
    pub const DURATION_500: &str = "500ms";
    pub const DURATION_700: &str = "700ms";
    pub const DURATION_1000: &str = "1000ms";

    pub const EASE_LINEAR: &str = "linear";
    pub const EASE_IN: &str = "cubic-bezier(0.4, 0, 1, 1)";
    pub const EASE_OUT: &str = "cubic-bezier(0, 0, 0.2, 1)";
    pub const EASE_IN_OUT: &str = "cubic-bezier(0.4, 0, 0.2, 1)";
}

/// Z-index scale
pub mod z_index {
    pub const AUTO: &str = "auto";
    pub const ZERO: &str = "0";
    pub const TEN: &str = "10";
    pub const TWENTY: &str = "20";
    pub const THIRTY: &str = "30";
    pub const FORTY: &str = "40";
    pub const FIFTY: &str = "50";
    pub const DROPDOWN: &str = "1000";
    pub const STICKY: &str = "1020";
    pub const FIXED: &str = "1030";
    pub const MODAL_BACKDROP: &str = "1040";
    pub const MODAL: &str = "1050";
    pub const POPOVER: &str = "1060";
    pub const TOOLTIP: &str = "1070";
    pub const TOAST: &str = "1080";
}

/// Breakpoints for responsive design
pub mod breakpoints {
    pub const SM: &str = "640px";
    pub const MD: &str = "768px";
    pub const LG: &str = "1024px";
    pub const XL: &str = "1280px";
    pub const XL2: &str = "1536px";
}

/// Utility functions for generating CSS
pub mod utils {
    // use super::*;  // Currently unused

    /// Generate CSS custom property
    pub fn css_var(name: &str, value: &str) -> String {
        format!("--{}: {};", name, value)
    }

    /// Generate CSS class with utility pattern
    pub fn utility_class(prefix: &str, suffix: &str, value: &str) -> String {
        format!(".{}-{} {{ {} }}", prefix, suffix, value)
    }

    /// Generate responsive CSS
    pub fn responsive(breakpoint: &str, css: &str) -> String {
        format!("@media (min-width: {}) {{ {} }}", breakpoint, css)
    }

    /// Generate CSS for dark theme
    pub fn dark_theme(css: &str) -> String {
        format!("@media (prefers-color-scheme: dark) {{ {} }}", css)
    }
}

/// Common CSS class generators
pub mod classes {
    use super::*;

    /// Generate spacing classes
    pub fn spacing_classes() -> String {
        format!(
            r#"
.p-xs {{ padding: {}; }}
.p-sm {{ padding: {}; }}
.p-md {{ padding: {}; }}
.p-lg {{ padding: {}; }}
.p-xl {{ padding: {}; }}
.p-xl2 {{ padding: {}; }}
.p-xl3 {{ padding: {}; }}

.m-xs {{ margin: {}; }}
.m-sm {{ margin: {}; }}
.m-md {{ margin: {}; }}
.m-lg {{ margin: {}; }}
.m-xl {{ margin: {}; }}
.m-xl2 {{ margin: {}; }}
.m-xl3 {{ margin: {}; }}
            "#,
            spacing::XS,
            spacing::SM,
            spacing::MD,
            spacing::LG,
            spacing::XL,
            spacing::XL2,
            spacing::XL3,
            spacing::XS,
            spacing::SM,
            spacing::MD,
            spacing::LG,
            spacing::XL,
            spacing::XL2,
            spacing::XL3
        )
    }

    /// Generate typography classes
    pub fn typography_classes() -> String {
        format!(
            r#"
.text-xs {{ font-size: {}; }}
.text-sm {{ font-size: {}; }}
.text-base {{ font-size: {}; }}
.text-lg {{ font-size: {}; }}
.text-xl {{ font-size: {}; }}
.text-2xl {{ font-size: {}; }}
.text-3xl {{ font-size: {}; }}

.font-thin {{ font-weight: {}; }}
.font-light {{ font-weight: {}; }}
.font-normal {{ font-weight: {}; }}
.font-medium {{ font-weight: {}; }}
.font-semibold {{ font-weight: {}; }}
.font-bold {{ font-weight: {}; }}
            "#,
            typography::TEXT_XS,
            typography::TEXT_SM,
            typography::TEXT_BASE,
            typography::TEXT_LG,
            typography::TEXT_XL,
            typography::TEXT_2XL,
            typography::TEXT_3XL,
            typography::FONT_THIN,
            typography::FONT_LIGHT,
            typography::FONT_NORMAL,
            typography::FONT_MEDIUM,
            typography::FONT_SEMIBOLD,
            typography::FONT_BOLD
        )
    }

    /// Generate color classes
    pub fn color_classes() -> String {
        format!(
            r#"
.text-primary {{ color: {}; }}
.text-primary-600 {{ color: {}; }}
.text-gray-500 {{ color: {}; }}
.text-gray-900 {{ color: {}; }}

.bg-primary {{ background-color: {}; }}
.bg-primary-600 {{ background-color: {}; }}
.bg-gray-50 {{ background-color: {}; }}
.bg-white {{ background-color: #ffffff; }}

.border-primary {{ border-color: {}; }}
.border-gray-200 {{ border-color: {}; }}
            "#,
            colors::PRIMARY_500,
            colors::PRIMARY_600,
            colors::GRAY_500,
            colors::GRAY_900,
            colors::PRIMARY_500,
            colors::PRIMARY_600,
            colors::GRAY_50,
            colors::PRIMARY_500,
            colors::GRAY_200
        )
    }
}

/// Get all design system CSS as a single string
pub fn get_all_styles() -> String {
    format!(
        r#"
/* Design System Variables */
:root {{
    --color-primary-50: {};
    --color-primary-500: {};
    --color-primary-600: {};
    --color-gray-50: {};
    --color-gray-500: {};
    --color-gray-900: {};
    --spacing-sm: {};
    --spacing-md: {};
    --spacing-lg: {};
    --spacing-xl: {};
    --text-sm: {};
    --text-base: {};
    --text-lg: {};
    --radius-md: {};
    --radius-lg: {};
    --shadow-md: {};
    --shadow-lg: {};
    --transition-duration: {};
    --transition-ease: {};
}}

/* Utility Classes */
{}

/* Typography Classes */
{}

/* Color Classes */
{}
        "#,
        colors::PRIMARY_50,
        colors::PRIMARY_500,
        colors::PRIMARY_600,
        colors::GRAY_50,
        colors::GRAY_500,
        colors::GRAY_900,
        spacing::SM,
        spacing::MD,
        spacing::LG,
        spacing::XL,
        typography::TEXT_SM,
        typography::TEXT_BASE,
        typography::TEXT_LG,
        radius::MD,
        radius::LG,
        shadows::MD,
        shadows::LG,
        transitions::DURATION_200,
        transitions::EASE_OUT,
        classes::spacing_classes(),
        classes::typography_classes(),
        classes::color_classes()
    )
}
