//! # Input Styles Module
//!
//! Input component styling for SurfDesk application.
//! Provides consistent input styling across all platforms.

use dioxus::prelude::*;

/// CSS for input components
pub const CSS_STYLES: &str = include_str!("input.css");

/// Input style constants
pub mod constants {
    pub const INPUT_HEIGHT: &str = "44px";
    pub const INPUT_PADDING: &str = "0.75rem 1rem";
    pub const INPUT_BORDER_RADIUS: &str = "0.5rem";
    pub const INPUT_BORDER_WIDTH: &str = "1px";
    pub const INPUT_FONT_SIZE: &str = "0.875rem";
    pub const INPUT_TRANSITION: &str = "all 0.2s ease";
}

/// Input style utilities
pub mod utils {
    use super::*;

    /// Generate input base styles
    pub fn input_base_styles() -> String {
        "
            width: 100%;
            padding: 0.75rem 1rem;
            border: 1px solid #d1d5db;
            border-radius: 0.5rem;
            background: #ffffff;
            color: #111827;
            font-size: 0.875rem;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
            transition: all 0.2s ease;
            outline: none;
            min-height: 44px;
        "
        .to_string()
    }

    /// Generate input focus styles
    pub fn input_focus_styles() -> String {
        "
            border-color: #3b82f6;
            box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
        "
        .to_string()
    }

    /// Generate input hover styles
    pub fn input_hover_styles() -> String {
        "
            border-color: #9ca3af;
            background: #f9fafb;
        "
        .to_string()
    }

    /// Generate input disabled styles
    pub fn input_disabled_styles() -> String {
        "
            background: #f9fafb;
            color: #6b7280;
            cursor: not-allowed;
            opacity: 0.6;
        "
        .to_string()
    }

    /// Generate input error styles
    pub fn input_error_styles() -> String {
        "
            border-color: #ef4444;
            box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
        "
        .to_string()
    }

    /// Generate textarea specific styles
    pub fn textarea_styles() -> String {
        "
            resize: vertical;
            min-height: 100px;
            line-height: 1.5;
        "
        .to_string()
    }
}

/// Input animation keyframes
pub mod animations {
    pub const INPUT_ERROR_SHAKE: &str = r#"
        @keyframes input-error-shake {
            0%, 100% { transform: translateX(0); }
            10%, 30%, 50%, 70%, 90% { transform: translateX(-2px); }
            20%, 40%, 60%, 80% { transform: translateX(2px); }
        }
    "#;

    pub const INPUT_FOCUS_RING: &str = r#"
        @keyframes input-focus-ring {
            0% { box-shadow: 0 0 0 0 rgba(59, 130, 246, 0.4); }
            100% { box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1); }
        }
    "#;
}

/// Get all input styles as a single string
pub fn get_all_styles() -> String {
    format!(
        r#"
        /* Input Component Styles */

        /* Base Input Wrapper */
        .surf-input-wrapper {{
            display: flex;
            flex-direction: column;
            gap: 0.5rem;
            width: 100%;
        }}

        /* Input Label */
        .surf-input-label {{
            font-size: 0.875rem;
            font-weight: 500;
            color: #374151;
            margin-bottom: 0.25rem;
            display: flex;
            align-items: center;
            gap: 0.25rem;
        }}

        .required-indicator {{
            color: #ef4444;
            font-weight: 700;
        }}

        /* Input Container */
        .surf-input-container {{
            position: relative;
            display: flex;
            align-items: center;
            width: 100%;
        }}

        /* Base Input Styles */
        .surf-input {{
            width: 100%;
            padding: 0.75rem 1rem;
            border: 1px solid #d1d5db;
            border-radius: 0.5rem;
            background: #ffffff;
            color: #111827;
            font-size: 0.875rem;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
            transition: all 0.2s ease;
            outline: none;
            min-height: 44px;
        }}

        .surf-input:hover:not(:disabled) {{
            border-color: #9ca3af;
            background: #f9fafb;
        }}

        .surf-input:focus {{
            border-color: #3b82f6;
            box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
            animation: input-focus-ring 0.2s ease-out;
        }}

        .surf-input:disabled {{
            background: #f9fafb;
            color: #6b7280;
            cursor: not-allowed;
            opacity: 0.6;
        }}

        .surf-input--error {{
            border-color: #ef4444;
            box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
            animation: input-error-shake 0.3s ease-in-out;
        }}

        .surf-input--error:focus {{
            border-color: #ef4444;
            box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
        }}

        /* Input Type Variants */
        .surf-input--password {{
            padding-right: calc(1rem + 2.5rem);
        }}

        .surf-input--number {{
            -moz-appearance: textfield;
            -webkit-appearance: textfield;
            appearance: textfield;
        }}

        .surf-input--number::-webkit-inner-spin-button,
        .surf-input--number::-webkit-outer-spin-button {{
            -webkit-appearance: none;
            margin: 0;
        }}

        .surf-input--number::-webkit-calendar-picker-indicator {{
            display: none;
        }}

        .surf-input--email {{
            text-transform: lowercase;
        }}

        .surf-input--search {{
            padding-left: calc(1rem + 2rem);
            background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke='%236b7280' stroke-width='2'%3E%3Ccircle cx='11' cy='11' r='8'/%3E%3Cpath d='m21 21-4.35-4.35'/%3E%3C/svg%3E");
            background-repeat: no-repeat;
            background-position: left 1rem center;
            background-size: 1rem 1rem;
        }}

        /* Password Toggle Button */
        .surf-input-toggle {{
            position: absolute;
            right: 0.75rem;
            top: 50%;
            transform: translateY(-50%);
            background: none;
            border: none;
            color: #6b7280;
            cursor: pointer;
            padding: 0.25rem;
            border-radius: 0.375rem;
            transition: all 0.2s ease;
            display: flex;
            align-items: center;
            justify-content: center;
        }}

        .surf-input-toggle:hover {{
            color: #374151;
            background: #f3f4f6;
        }}

        .surf-input-toggle:disabled {{
            cursor: not-allowed;
            opacity: 0.5;
        }}

        .toggle-icon {{
            font-size: 1rem;
            user-select: none;
        }}

        /* Textarea Styles */
        .surf-textarea-wrapper {{
            width: 100%;
        }}

        .surf-textarea {{
            width: 100%;
            padding: 0.75rem 1rem;
            border: 1px solid #d1d5db;
            border-radius: 0.5rem;
            background: #ffffff;
            color: #111827;
            font-size: 0.875rem;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
            line-height: 1.5;
            transition: all 0.2s ease;
            outline: none;
            resize: vertical;
            min-height: 100px;
        }}

        .surf-textarea:focus {{
            border-color: #3b82f6;
            box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
        }}

        .surf-textarea:hover:not(:disabled) {{
            border-color: #9ca3af;
            background: #f9fafb;
        }}

        .surf-textarea:disabled {{
            background: #f9fafb;
            color: #6b7280;
            cursor: not-allowed;
            opacity: 0.6;
        }}

        /* Helper Text */
        .surf-input-helper {{
            font-size: 0.75rem;
            color: #6b7280;
            margin-top: 0.25rem;
        }}

        /* Error Message */
        .surf-input-error {{
            font-size: 0.75rem;
            color: #dc2626;
            background: rgba(239, 68, 68, 0.1);
            padding: 0.5rem 0.75rem;
            border-radius: 0.375rem;
            border: 1px solid rgba(239, 68, 68, 0.2);
            margin-top: 0.5rem;
        }}

        /* Input Group */
        .surf-input-group {{
            display: flex;
            flex-direction: column;
            gap: 0.75rem;
            width: 100%;
        }}

        .surf-input-group-header {{
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 0.5rem;
        }}

        .surf-input-group-label {{
            font-size: 1rem;
            font-weight: 600;
            color: #111827;
            display: flex;
            align-items: center;
            gap: 0.25rem;
        }}

        .surf-input-group-description {{
            font-size: 0.875rem;
            color: #6b7280;
            margin-bottom: 0.75rem;
            line-height: 1.5;
        }}

        .surf-input-group-content {{
            display: flex;
            flex-direction: column;
            gap: 0.75rem;
        }}

        /* Size Variants */
        .surf-input--small {{
            padding: 0.5rem 0.75rem;
            font-size: 0.75rem;
            min-height: 36px;
        }}

        .surf-input--large {{
            padding: 1rem 1.25rem;
            font-size: 1rem;
            min-height: 52px;
        }}

        /* Dark Theme */
        @media (prefers-color-scheme: dark) {{
            .surf-input {{
                background: #1f2937;
                border-color: #374151;
                color: #f9fafb;
            }}

            .surf-input:hover:not(:disabled) {{
                border-color: #4b5563;
                background: #374151;
            }}

            .surf-input:focus {{
                border-color: #60a5fa;
                box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.2);
            }}

            .surf-input:disabled {{
                background: #111827;
                color: #6b7280;
            }}

            .surf-input-label {{
                color: #d1d5db;
            }}

            .surf-input-helper {{
                color: #9ca3af;
            }}

            .surf-input-toggle {{
                color: #9ca3af;
            }}

            .surf-input-toggle:hover {{
                background: #374151;
                color: #d1d5db;
            }}

            .surf-textarea {{
                background: #1f2937;
                border-color: #374151;
                color: #f9fafb;
            }}

            .surf-textarea:focus {{
                border-color: #60a5fa;
                box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.2);
            }}
        }}

        /* High Contrast Mode */
        @media (prefers-contrast: high) {{
            .surf-input,
            .surf-textarea {{
                border-width: 2px;
                border-color: #000000;
            }}

            .surf-input:focus,
            .surf-textarea:focus {{
                border-color: #0000ff;
                box-shadow: 0 0 0 2px #0000ff;
            }}

            .surf-input--error,
            .surf-textarea--error {{
                border-color: #000000;
                background: rgba(239, 68, 68, 0.2);
            }}
        }}

        /* Reduced Motion */
        @media (prefers-reduced-motion: reduce) {{
            .surf-input,
            .surf-textarea,
            .surf-input-toggle {{
                transition: none;
            }}

            .surf-input:focus {{
                animation: none;
            }}
        }}

        /* Mobile Responsive */
        @media (max-width: 640px) {{
            .surf-input-wrapper {{
                gap: 0.25rem;
            }}

            .surf-input-label {{
                font-size: 0.75rem;
            }}

            .surf-input-group-label {{
                font-size: 0.875rem;
            }}

            .surf-input-group-description {{
                font-size: 0.75rem;
            }}

            .surf-input {{
                padding: 0.5rem 0.75rem;
                font-size: 0.75rem;
                min-height: 40px;
            }}

            .surf-textarea {{
                padding: 0.5rem 0.75rem;
                font-size: 0.75rem;
            }}
        }}

        /* Animations */
        {}
        {}
        "#,
        animations::INPUT_ERROR_SHAKE,
        animations::INPUT_FOCUS_RING
    )
}
