#![allow(dead_code)]
//! # Toast Styles Module
//!
//! Toast notification styling for SurfDesk application.
//! Provides consistent toast styling across all platforms.

use dioxus::prelude::*;

/// CSS for toast components
pub const CSS_STYLES: &str = include_str!("toast.css");

/// Toast style constants
pub mod constants {
    pub const TOAST_Z_INDEX: &str = "1100";
    pub const TOAST_MAX_WIDTH: &str = "400px";
    pub const TOAST_MIN_HEIGHT: &str = "60px";
    pub const TOAST_BORDER_RADIUS: &str = "0.5rem";
    pub const TOAST_SHADOW: &str =
        "0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)";
    pub const TOAST_DURATION: &str = "0.2s";
}

/// Toast style utilities
pub mod utils {
    use super::*;

    /// Generate toast container styles
    pub fn container_styles(position: &str) -> String {
        format!(
            r#"
            position: fixed;
            z-index: 1100;
            pointer-events: none;
            display: flex;
            flex-direction: column;
            gap: 0.75rem;
            max-width: 400px;
            width: 100%;
            {}
            "#,
            match position {
                "top-right" => "top: 1rem; right: 1rem;",
                "top-left" => "top: 1rem; left: 1rem;",
                "bottom-right" => "bottom: 1rem; right: 1rem;",
                "bottom-left" => "bottom: 1rem; left: 1rem;",
                "top-center" => "top: 1rem; left: 50%; transform: translateX(-50%);",
                "bottom-center" => "bottom: 1rem; left: 50%; transform: translateX(-50%);",
                _ => "top: 1rem; right: 1rem;",
            }
        )
    }

    /// Generate individual toast styles
    pub fn toast_styles(variant: &str) -> String {
        let base_styles = r#"
            display: flex;
            align-items: flex-start;
            gap: 0.75rem;
            padding: 1rem;
            border-radius: 0.5rem;
            box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
            background: #ffffff;
            border: 1px solid #e5e7eb;
            pointer-events: all;
            cursor: pointer;
            transition: all 0.2s ease;
            position: relative;
            overflow: hidden;
            min-height: 60px;
        "#;

        let variant_styles = match variant {
            "success" => {
                r#"
                background: linear-gradient(135deg, #f0fdf4 0%, #dcfce7 100%);
                border-color: #bbf7d0;
                color: #166534;
            "#
            }
            "error" => {
                r#"
                background: linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%);
                border-color: #fecaca;
                color: #991b1b;
            "#
            }
            "warning" => {
                r#"
                background: linear-gradient(135deg, #fffbeb 0%, #fef3c7 100%);
                border-color: #fde68a;
                color: #92400e;
            "#
            }
            "info" => {
                r#"
                background: linear-gradient(135deg, #eff6ff 0%, #dbeafe 100%);
                border-color: #bfdbfe;
                color: #1e40af;
            "#
            }
            _ => "",
        };

        format!("{} {}", base_styles, variant_styles)
    }

    /// Generate toast icon styles
    pub fn icon_styles(variant: &str) -> String {
        let base_styles = r#"
            display: flex;
            align-items: center;
            justify-content: center;
            width: 1.5rem;
            height: 1.5rem;
            border-radius: 50%;
            flex-shrink: 0;
            margin-top: 2px;
        "#;

        let variant_styles = match variant {
            "success" => "background: #22c55e; color: white;",
            "error" => "background: #ef4444; color: white;",
            "warning" => "background: #f59e0b; color: white;",
            "info" => "background: #3b82f6; color: white;",
            _ => "",
        };

        format!("{} {}", base_styles, variant_styles)
    }
}

/// Toast animation keyframes
pub mod animations {
    pub const TOAST_SLIDE_IN: &str = r#"
        @keyframes toast-slide-in {
            from {
                opacity: 0;
                transform: translateX(100%) scale(0.9);
            }
            to {
                opacity: 1;
                transform: translateX(0) scale(1);
            }
        }
    "#;

    pub const TOAST_SLIDE_OUT: &str = r#"
        @keyframes toast-slide-out {
            from {
                opacity: 1;
                transform: translateX(0) scale(1);
            }
            to {
                opacity: 0;
                transform: translateX(100%) scale(0.9);
            }
        }
    "#;

    pub const TOAST_SLIDE_IN_LEFT: &str = r#"
        @keyframes toast-slide-in-left {
            from {
                opacity: 0;
                transform: translateX(-100%) scale(0.9);
            }
            to {
                opacity: 1;
                transform: translateX(0) scale(1);
            }
        }
    "#;

    pub const TOAST_SLIDE_OUT_LEFT: &str = r#"
        @keyframes toast-slide-out-left {
            from {
                opacity: 1;
                transform: translateX(0) scale(1);
            }
            to {
                opacity: 0;
                transform: translateX(-100%) scale(0.9);
            }
        }
    "#;
}

/// Get all toast styles as a single string
pub fn get_all_styles() -> String {
    format!(
        r#"
        /* Toast Component Styles */

        /* Toast Container */
        .toast-container {{
            position: fixed;
            z-index: 1100;
            pointer-events: none;
            display: flex;
            flex-direction: column;
            gap: 0.75rem;
            max-width: 400px;
            width: 100%;
        }}

        /* Position Variants */
        .toast-container--top-right {{
            top: 1rem;
            right: 1rem;
        }}

        .toast-container--top-left {{
            top: 1rem;
            left: 1rem;
        }}

        .toast-container--bottom-right {{
            bottom: 1rem;
            right: 1rem;
        }}

        .toast-container--bottom-left {{
            bottom: 1rem;
            left: 1rem;
        }}

        .toast-container--top-center {{
            top: 1rem;
            left: 50%;
            transform: translateX(-50%);
        }}

        .toast-container--bottom-center {{
            bottom: 1rem;
            left: 50%;
            transform: translateX(-50%);
        }}

        /* Individual Toast */
        .toast {{
            display: flex;
            align-items: flex-start;
            gap: 0.75rem;
            padding: 1rem;
            border-radius: 0.5rem;
            box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
            background: #ffffff;
            border: 1px solid #e5e7eb;
            pointer-events: all;
            cursor: pointer;
            transition: all 0.2s ease;
            position: relative;
            overflow: hidden;
            min-height: 60px;
            opacity: 0;
            transform: translateX(100%) scale(0.9);
        }}

        .toast--visible {{
            opacity: 1;
            transform: translateX(0) scale(1);
            animation: toast-slide-in 0.3s ease-out;
        }}

        .toast--hiding {{
            opacity: 0;
            transform: translateX(100%) scale(0.9);
            animation: toast-slide-out 0.3s ease-in;
        }}

        /* Toast Variants */
        .toast--success {{
            background: linear-gradient(135deg, #f0fdf4 0%, #dcfce7 100%);
            border-color: #bbf7d0;
            color: #166534;
        }}

        .toast--error {{
            background: linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%);
            border-color: #fecaca;
            color: #991b1b;
        }}

        .toast--warning {{
            background: linear-gradient(135deg, #fffbeb 0%, #fef3c7 100%);
            border-color: #fde68a;
            color: #92400e;
        }}

        .toast--info {{
            background: linear-gradient(135deg, #eff6ff 0%, #dbeafe 100%);
            border-color: #bfdbfe;
            color: #1e40af;
        }}

        /* Toast Icon */
        .toast-icon {{
            display: flex;
            align-items: center;
            justify-content: center;
            width: 1.5rem;
            height: 1.5rem;
            border-radius: 50%;
            flex-shrink: 0;
            margin-top: 2px;
        }}

        .toast-icon-symbol {{
            font-size: 0.875rem;
            font-weight: 700;
            line-height: 1;
        }}

        .toast--success .toast-icon {{
            background: #22c55e;
            color: white;
        }}

        .toast--error .toast-icon {{
            background: #ef4444;
            color: white;
        }}

        .toast--warning .toast-icon {{
            background: #f59e0b;
            color: white;
        }}

        .toast--info .toast-icon {{
            background: #3b82f6;
            color: white;
        }}

        /* Toast Content */
        .toast-content {{
            flex: 1;
            display: flex;
            flex-direction: column;
            gap: 0.5rem;
            min-width: 0;
        }}

        .toast-message {{
            font-size: 0.875rem;
            font-weight: 500;
            line-height: 1.25;
            word-wrap: break-word;
        }}

        /* Toast Action Button */
        .toast-action {{
            background: none;
            border: 1px solid currentColor;
            color: inherit;
            padding: 0.25rem 0.5rem;
            border-radius: 0.25rem;
            font-size: 0.75rem;
            font-weight: 500;
            cursor: pointer;
            transition: all 0.15s ease;
            align-self: flex-start;
            text-decoration: none;
            display: inline-block;
        }}

        .toast-action:hover {{
            background: rgba(0, 0, 0, 0.1);
            transform: translateY(-1px);
        }}

        .toast-action:focus {{
            outline: 2px solid currentColor;
            outline-offset: 2px;
        }}

        /* Toast Close Button */
        .toast-close {{
            background: none;
            border: none;
            color: inherit;
            cursor: pointer;
            padding: 0.25rem;
            border-radius: 0.25rem;
            font-size: 1.125rem;
            font-weight: 300;
            line-height: 1;
            width: 1.5rem;
            height: 1.5rem;
            display: flex;
            align-items: center;
            justify-content: center;
            flex-shrink: 0;
            margin-top: 2px;
            transition: all 0.15s ease;
            opacity: 0.7;
        }}

        .toast-close:hover {{
            opacity: 1;
            background: rgba(0, 0, 0, 0.1);
            transform: scale(1.1);
        }}

        .toast-close:focus {{
            outline: 2px solid currentColor;
            outline-offset: 2px;
            opacity: 1;
        }}

        /* Toast Progress Bar */
        .toast-progress {{
            position: absolute;
            bottom: 0;
            left: 0;
            right: 0;
            height: 3px;
            background: rgba(0, 0, 0, 0.1);
            overflow: hidden;
        }}

        .toast-progress-bar {{
            height: 100%;
            background: currentColor;
            opacity: 0.3;
            transition: width 0.1s linear;
        }}

        .toast--success .toast-progress-bar {{
            background: #22c55e;
        }}

        .toast--error .toast-progress-bar {{
            background: #ef4444;
        }}

        .toast--warning .toast-progress-bar {{
            background: #f59e0b;
        }}

        .toast--info .toast-progress-bar {{
            background: #3b82f6;
        }}

        /* Hover Effects */
        .toast:hover {{
            transform: translateX(-2px) scale(1.02);
            box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.15), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
        }}

        .toast:hover .toast-progress-bar {{
            opacity: 0.1;
        }}

        /* Left-side animations */
        .toast-container--top-left .toast--visible,
        .toast-container--bottom-left .toast--visible {{
            animation: toast-slide-in-left 0.3s ease-out;
        }}

        .toast-container--top-left .toast--hiding,
        .toast-container--bottom-left .toast--hiding {{
            animation: toast-slide-out-left 0.3s ease-in;
        }}

        /* Stacking Animation */
        .toast-container > .toast:nth-child(1) {{
            animation-delay: 0ms;
        }}

        .toast-container > .toast:nth-child(2) {{
            animation-delay: 50ms;
        }}

        .toast-container > .toast:nth-child(3) {{
            animation-delay: 100ms;
        }}

        .toast-container > .toast:nth-child(4) {{
            animation-delay: 150ms;
        }}

        .toast-container > .toast:nth-child(5) {{
            animation-delay: 200ms;
        }}

        /* Dark Theme */
        @media (prefers-color-scheme: dark) {{
            .toast {{
                background: #1f2937;
                border-color: #374151;
                color: #f9fafb;
                box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.3), 0 10px 10px -5px rgba(0, 0, 0, 0.2);
            }}

            .toast--success {{
                background: linear-gradient(135deg, #064e3b 0%, #065f46 100%);
                border-color: #047857;
                color: #86efac;
            }}

            .toast--error {{
                background: linear-gradient(135deg, #7f1d1d 0%, #991b1b 100%);
                border-color: #b91c1c;
                color: #fca5a5;
            }}

            .toast--warning {{
                background: linear-gradient(135deg, #78350f 0%, #92400e 100%);
                border-color: #b45309;
                color: #fcd34d;
            }}

            .toast--info {{
                background: linear-gradient(135deg, #1e3a8a 0%, #1e40af 100%);
                border-color: #1d4ed8;
                color: #93c5fd;
            }}

            .toast-action {{
                border-color: currentColor;
            }}

            .toast-action:hover {{
                background: rgba(255, 255, 255, 0.1);
            }}

            .toast-close:hover {{
                background: rgba(255, 255, 255, 0.1);
            }}

            .toast-progress {{
                background: rgba(255, 255, 255, 0.1);
            }}
        }}

        /* High Contrast Mode */
        @media (prefers-contrast: high) {{
            .toast {{
                border-width: 2px;
                background: #ffffff;
                color: #000000;
            }}

            .toast--success {{
                background: #ffffff;
                border-color: #006600;
                color: #000000;
            }}

            .toast--error {{
                background: #ffffff;
                border-color: #cc0000;
                color: #000000;
            }}

            .toast--warning {{
                background: #ffffff;
                border-color: #cc6600;
                color: #000000;
            }}

            .toast--info {{
                background: #ffffff;
                border-color: #0066cc;
                color: #000000;
            }}

            .toast-icon {{
                background: currentColor !important;
                color: white !important;
            }}

            .theme-dark .toast {{
                background: #000000;
                color: #ffffff;
                border-color: #ffffff;
            }}
        }}

        /* Reduced Motion */
        @media (prefers-reduced-motion: reduce) {{
            .toast,
            .toast-close,
            .toast-action {{
                animation: none;
                transition: none;
            }}

            .toast--visible {{
                opacity: 1;
                transform: none;
            }}

            .toast--hiding {{
                opacity: 0;
                transform: none;
            }}
        }}

        /* Mobile Responsive */
        @media (max-width: 640px) {{
            .toast-container {{
                left: 0.5rem !important;
                right: 0.5rem !important;
                top: 0.5rem !important;
                bottom: 0.5rem !important;
                transform: none !important;
                max-width: none;
            }}

            .toast {{
                padding: 0.75rem;
                min-height: 50px;
            }}

            .toast-message {{
                font-size: 0.75rem;
            }}

            .toast-icon {{
                width: 1.25rem;
                height: 1.25rem;
            }}

            .toast-icon-symbol {{
                font-size: 0.75rem;
            }}

            .toast-close {{
                width: 1.25rem;
                height: 1.25rem;
                font-size: 1rem;
            }}
        }}

        /* Print Styles */
        @media print {{
            .toast-container {{
                display: none !important;
            }}
        }}

        /* Screen Reader Only Content */
        .sr-only {{
            position: absolute;
            width: 1px;
            height: 1px;
            padding: 0;
            margin: -1px;
            overflow: hidden;
            clip: rect(0, 0, 0, 0);
            white-space: nowrap;
            border: 0;
        }}

        /* Focus Management */
        .toast:focus-within {{
            outline: 2px solid #3b82f6;
            outline-offset: 2px;
        }}

        /* Animations */
        {}
        {}
        {}
        {}
        "#,
        animations::TOAST_SLIDE_IN,
        animations::TOAST_SLIDE_OUT,
        animations::TOAST_SLIDE_IN_LEFT,
        animations::TOAST_SLIDE_OUT_LEFT
    )
}
