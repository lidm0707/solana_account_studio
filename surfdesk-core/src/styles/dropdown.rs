//! # Dropdown Styles Module
//!
//! Dropdown component styling for SurfDesk application.
//! Provides consistent dropdown styling across all platforms.

use dioxus::prelude::*;

/// CSS for dropdown components
pub const CSS_STYLES: &str = include_str!("dropdown.css");

/// Dropdown style constants
pub mod constants {
    pub const DROPDOWN_Z_INDEX: &str = "1000";
    pub const DROPDOWN_MAX_HEIGHT: &str = "300px";
    pub const DROPDOWN_MIN_WIDTH: &str = "200px";
    pub const DROPDOWN_BORDER_RADIUS: &str = "0.5rem";
    pub const DROPDOWN_SHADOW: &str =
        "0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)";
}

/// Dropdown style utilities
pub mod utils {
    use super::*;

    /// Generate dropdown trigger styles
    pub fn trigger_styles(is_open: bool, is_disabled: bool) -> String {
        let base_styles = "
            width: 100%;
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0.75rem 1rem;
            border: 1px solid #d1d5db;
            border-radius: 0.5rem;
            background: #ffffff;
            color: #111827;
            font-size: 0.875rem;
            font-weight: 500;
            cursor: pointer;
            transition: all 0.2s ease;
            outline: none;
            min-height: 44px;
        ";

        let state_styles = if is_disabled {
            "
                background: #f9fafb;
                color: #6b7280;
                cursor: not-allowed;
                opacity: 0.6;
            "
        } else if is_open {
            "
                border-color: #3b82f6;
                box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
            "
        } else {
            ""
        };

        format!("{} {}", base_styles, state_styles)
    }

    /// Generate dropdown menu styles
    pub fn menu_styles() -> String {
        "
            position: absolute;
            top: 100%;
            left: 0;
            right: 0;
            z-index: 1000;
            background: #ffffff;
            border: 1px solid #d1d5db;
            border-radius: 0.5rem;
            box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
            margin-top: 0.25rem;
            max-height: 300px;
            overflow: hidden;
            opacity: 0;
            transform: translateY(-10px) scale(0.95);
            animation: dropdown-appear 0.2s ease-out forwards;
        "
        .to_string()
    }

    /// Generate dropdown option styles
    pub fn option_styles(is_focused: bool, is_selected: bool, is_disabled: bool) -> String {
        let base_styles = "
            width: 100%;
            display: flex;
            align-items: center;
            padding: 0.5rem 0.75rem;
            border: none;
            border-radius: 0.375rem;
            background: transparent;
            color: #111827;
            font-size: 0.875rem;
            font-weight: 500;
            text-align: left;
            cursor: pointer;
            transition: all 0.15s ease;
            outline: none;
            min-height: 40px;
        ";

        let state_styles = if is_disabled {
            "
                color: #6b7280;
                cursor: not-allowed;
                opacity: 0.6;
            "
        } else if is_selected {
            "
                background: #dbeafe;
                color: #1e40af;
                font-weight: 600;
            "
        } else if is_focused {
            "
                background: #eff6ff;
                color: #1e40af;
            "
        } else {
            "
                &:hover {
                    background: #f3f4f6;
                    color: #111827;
                }
            "
        };

        format!("{} {}", base_styles, state_styles)
    }

    /// Generate search input styles
    pub fn search_styles() -> String {
        "
            width: 100%;
            padding: 0.5rem 0.75rem;
            border: 1px solid #d1d5db;
            border-radius: 0.375rem;
            background: #f9fafb;
            color: #111827;
            font-size: 0.875rem;
            outline: none;
            transition: all 0.15s ease;

            &:focus {
                border-color: #3b82f6;
                box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
            }

            &::placeholder {
                color: #6b7280;
            }
        "
        .to_string()
    }
}

/// Dropdown animation keyframes
pub mod animations {
    pub const DROPDOWN_APPEAR: &str = r#"
        @keyframes dropdown-appear {
            from {
                opacity: 0;
                transform: translateY(-10px) scale(0.95);
            }
            to {
                opacity: 1;
                transform: translateY(0) scale(1);
            }
        }
    "#;

    pub const DROPDOWN_DISAPPEAR: &str = r#"
        @keyframes dropdown-disappear {
            from {
                opacity: 1;
                transform: translateY(0) scale(1);
            }
            to {
                opacity: 0;
                transform: translateY(-10px) scale(0.95);
            }
        }
    "#;
}

/// Get all dropdown styles as a single string
pub fn get_all_styles() -> String {
    format!(
        r#"
        /* Dropdown Component Styles */

        /* Base Dropdown Container */
        .dropdown {{
            position: relative;
            display: inline-block;
            width: 100%;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
        }}

        /* Dropdown Trigger */
        .dropdown-trigger {{
            width: 100%;
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0.75rem 1rem;
            border: 1px solid #d1d5db;
            border-radius: 0.5rem;
            background: #ffffff;
            color: #111827;
            font-size: 0.875rem;
            font-weight: 500;
            cursor: pointer;
            transition: all 0.2s ease;
            outline: none;
            min-height: 44px;
        }}

        .dropdown-trigger:hover:not(:disabled) {{
            border-color: #9ca3af;
            background: #f9fafb;
        }}

        .dropdown-trigger:focus {{
            border-color: #3b82f6;
            box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
        }}

        .dropdown-trigger:disabled {{
            background: #f9fafb;
            color: #6b7280;
            cursor: not-allowed;
            opacity: 0.6;
        }}

        .dropdown.open .dropdown-trigger {{
            border-color: #3b82f6;
            box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
        }}

        /* Dropdown Arrow */
        .dropdown-arrow {{
            margin-left: 0.5rem;
            color: #6b7280;
            font-size: 0.75rem;
            transition: transform 0.2s ease;
            flex-shrink: 0;
        }}

        .dropdown.open .dropdown-arrow {{
            transform: rotate(180deg);
        }}

        /* Dropdown Menu */
        .dropdown-menu {{
            position: absolute;
            top: 100%;
            left: 0;
            right: 0;
            z-index: 1000;
            background: #ffffff;
            border: 1px solid #d1d5db;
            border-radius: 0.5rem;
            box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
            margin-top: 0.25rem;
            max-height: 300px;
            overflow: hidden;
        }}

        /* Dropdown Search */
        .dropdown-search {{
            padding: 0.75rem;
            border-bottom: 1px solid #e5e7eb;
            background: #f9fafb;
        }}

        .dropdown-search-input {{
            width: 100%;
            padding: 0.5rem 0.75rem;
            border: 1px solid #d1d5db;
            border-radius: 0.375rem;
            background: #ffffff;
            color: #111827;
            font-size: 0.875rem;
            outline: none;
            transition: all 0.15s ease;
        }}

        .dropdown-search-input:focus {{
            border-color: #3b82f6;
            box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
        }}

        .dropdown-search-input::placeholder {{
            color: #6b7280;
        }}

        /* Dropdown Options */
        .dropdown-options {{
            max-height: 250px;
            overflow-y: auto;
            padding: 0.25rem;
        }}

        .dropdown-option {{
            width: 100%;
            display: flex;
            align-items: center;
            padding: 0.5rem 0.75rem;
            border: none;
            border-radius: 0.375rem;
            background: transparent;
            color: #111827;
            font-size: 0.875rem;
            font-weight: 500;
            text-align: left;
            cursor: pointer;
            transition: all 0.15s ease;
            outline: none;
            min-height: 40px;
        }}

        .dropdown-option:hover:not(:disabled) {{
            background: #f3f4f6;
            color: #111827;
        }}

        .dropdown-option:focus,
        .dropdown-option.focused {{
            background: #eff6ff;
            color: #1e40af;
            outline: 2px solid #3b82f6;
            outline-offset: -2px;
        }}

        .dropdown-option.selected {{
            background: #dbeafe;
            color: #1e40af;
            font-weight: 600;
        }}

        .dropdown-option:disabled {{
            color: #6b7280;
            cursor: not-allowed;
            opacity: 0.6;
        }}

        /* Multi-select Specific */
        .dropdown.multi-select .dropdown-trigger {{
            padding: 0.5rem 0.75rem;
            min-height: 48px;
        }}

        .dropdown-selected-tags {{
            flex: 1;
            display: flex;
            flex-wrap: wrap;
            align-items: center;
            gap: 0.5rem;
        }}

        .dropdown-tag {{
            display: inline-flex;
            align-items: center;
            gap: 0.25rem;
            padding: 0.25rem 0.5rem;
            background: #dbeafe;
            color: #1e40af;
            border-radius: 0.375rem;
            font-size: 0.75rem;
            font-weight: 500;
            max-width: 150px;
        }}

        .dropdown-tag-remove {{
            background: none;
            border: none;
            color: #1e40af;
            cursor: pointer;
            padding: 0;
            border-radius: 0.125rem;
            font-size: 0.875rem;
            line-height: 1;
            width: 16px;
            height: 16px;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: all 0.15s ease;
        }}

        .dropdown-tag-remove:hover {{
            background: #bfdbfe;
            color: #1e3a8a;
        }}

        /* No Results */
        .dropdown-no-results {{
            padding: 1rem;
            text-align: center;
            color: #6b7280;
            font-size: 0.875rem;
            font-style: italic;
        }}

        /* Dark Theme */
        @media (prefers-color-scheme: dark) {{
            .dropdown-trigger {{
                background: #1f2937;
                border-color: #374151;
                color: #f9fafb;
            }}

            .dropdown-trigger:hover:not(:disabled) {{
                border-color: #4b5563;
                background: #374151;
            }}

            .dropdown-trigger:focus {{
                border-color: #60a5fa;
                box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.2);
            }}

            .dropdown-menu {{
                background: #1f2937;
                border-color: #374151;
                box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.3);
            }}

            .dropdown-search {{
                background: #111827;
                border-color: #374151;
            }}

            .dropdown-search-input {{
                background: #1f2937;
                border-color: #4b5563;
                color: #f9fafb;
            }}

            .dropdown-option {{
                color: #f9fafb;
            }}

            .dropdown-option:hover:not(:disabled) {{
                background: #374151;
            }}

            .dropdown-option:focus,
            .dropdown-option.focused {{
                background: #1e3a8a;
                color: #dbeafe;
            }}

            .dropdown-option.selected {{
                background: #1e3a8a;
                color: #dbeafe;
            }}
        }}

        /* Animations */
        {}
        {}
        "#,
        animations::DROPDOWN_APPEAR,
        animations::DROPDOWN_DISAPPEAR
    )
}
