//! # Modal Styles Module
//!
//! Modal dialog and overlay styling for SurfDesk application.
//! Provides consistent modal styling across all platforms.

use dioxus::prelude::*;

/// CSS for modal components
pub const CSS_STYLES: &str = include_str!("modal.css");

/// Modal style constants
pub mod constants {
    pub const MODAL_Z_INDEX: &str = "1050";
    pub const MODAL_BACKDROP_Z_INDEX: &str = "1040";
    pub const MODAL_MAX_WIDTH: &str = "640px";
    pub const MODAL_BORDER_RADIUS: &str = "0.75rem";
    pub const MODAL_SHADOW: &str = "0 25px 50px -12px rgba(0, 0, 0, 0.25)";
    pub const MODAL_BACKDROP_BLUR: &str = "blur(4px)";
}

/// Modal style utilities
pub mod utils {
    use super::*;

    /// Generate modal overlay styles
    pub fn overlay_styles() -> String {
        "
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: rgba(0, 0, 0, 0.6);
            backdrop-filter: blur(4px);
            display: flex;
            align-items: center;
            justify-content: center;
            z-index: 1040;
            opacity: 0;
            animation: modal-overlay-fade-in 0.2s ease-out forwards;
        "
        .to_string()
    }

    /// Generate modal container styles
    pub fn modal_styles() -> String {
        "
            background: #ffffff;
            border-radius: 0.75rem;
            box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
            max-height: 90vh;
            overflow: hidden;
            display: flex;
            flex-direction: column;
            opacity: 0;
            transform: scale(0.95) translateY(20px);
            animation: modal-appear 0.3s ease-out 0.1s forwards;
            outline: none;
            border: 1px solid #e5e7eb;
        "
        .to_string()
    }

    /// Generate modal header styles
    pub fn header_styles() -> String {
        "
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 1.5rem;
            border-bottom: 1px solid #e5e7eb;
            background: #f9fafb;
        "
        .to_string()
    }

    /// Generate modal body styles
    pub fn body_styles() -> String {
        "
            padding: 1.5rem;
            overflow-y: auto;
            flex: 1;
            color: #374151;
            line-height: 1.625;
        "
        .to_string()
    }

    /// Generate modal footer styles
    pub fn footer_styles() -> String {
        "
            padding: 1rem 1.5rem;
            border-top: 1px solid #e5e7eb;
            background: #f9fafb;
            display: flex;
            justify-content: flex-end;
            gap: 0.75rem;
        "
        .to_string()
    }
}

/// Modal animation keyframes
pub mod animations {
    pub const MODAL_OVERLAY_FADE_IN: &str = r#"
        @keyframes modal-overlay-fade-in {
            from { opacity: 0; }
            to { opacity: 1; }
        }
    "#;

    pub const MODAL_OVERLAY_FADE_OUT: &str = r#"
        @keyframes modal-overlay-fade-out {
            from { opacity: 1; }
            to { opacity: 0; }
        }
    "#;

    pub const MODAL_APPEAR: &str = r#"
        @keyframes modal-appear {
            from {
                opacity: 0;
                transform: scale(0.95) translateY(20px);
            }
            to {
                opacity: 1;
                transform: scale(1) translateY(0);
            }
        }
    "#;

    pub const MODAL_DISAPPEAR: &str = r#"
        @keyframes modal-disappear {
            from {
                opacity: 1;
                transform: scale(1) translateY(0);
            }
            to {
                opacity: 0;
                transform: scale(0.95) translateY(-20px);
            }
        }
    "#;
}

/// Modal size variants
pub mod sizes {
    pub const SMALL: &str = "max-width: 320px;";
    pub const MEDIUM: &str = "max-width: 480px;";
    pub const LARGE: &str = "max-width: 640px;";
    pub const FULLSCREEN: &str =
        "width: 100%; height: 100%; max-width: none; max-height: none; border-radius: 0;";
    pub const AUTO: &str = "width: auto; min-width: 320px; max-width: 90vw;";
}

/// Get all modal styles as a single string
pub fn get_all_styles() -> String {
    format!(
        r#"
        /* Modal Component Styles */

        /* Modal Overlay */
        .modal-overlay {{
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: rgba(0, 0, 0, 0.6);
            backdrop-filter: blur(4px);
            display: flex;
            align-items: center;
            justify-content: center;
            z-index: 1040;
            opacity: 0;
            animation: modal-overlay-fade-in 0.2s ease-out forwards;
            padding: 1rem;
        }}

        .modal-overlay.closing {{
            animation: modal-overlay-fade-out 0.2s ease-in forwards;
        }}

        /* Modal Container */
        .modal {{
            background: #ffffff;
            border-radius: 0.75rem;
            box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
            max-height: 90vh;
            overflow: hidden;
            display: flex;
            flex-direction: column;
            opacity: 0;
            transform: scale(0.95) translateY(20px);
            animation: modal-appear 0.3s ease-out 0.1s forwards;
            outline: none;
            border: 1px solid #e5e7eb;
        }}

        .modal.closing {{
            animation: modal-disappear 0.2s ease-in forwards;
        }}

        /* Size Variants */
        .modal--small {{ {} }}
        .modal--medium {{ {} }}
        .modal--large {{ {} }}
        .modal--fullscreen {{ {} }}
        .modal--auto {{ {} }}

        /* Variant Styling */
        .modal--alert {{
            border-top: 4px solid #f59e0b;
        }}

        .modal--alert .modal-title {{
            color: #d97706;
        }}

        .modal--confirm {{
            border-top: 4px solid #3b82f6;
        }}

        .modal--confirm .modal-title {{
            color: #1d4ed8;
        }}

        .modal--success {{
            border-top: 4px solid #22c55e;
        }}

        .modal--success .modal-title {{
            color: #15803d;
        }}

        .modal--error {{
            border-top: 4px solid #ef4444;
        }}

        .modal--error .modal-title {{
            color: #b91c1c;
        }}

        .modal--info {{
            border-top: 4px solid #3b82f6;
        }}

        .modal--info .modal-title {{
            color: #1d4ed8;
        }}

        /* Modal Header */
        .modal-header {{
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 1.5rem;
            border-bottom: 1px solid #e5e7eb;
            background: #f9fafb;
        }}

        .modal-title {{
            font-size: 1.125rem;
            font-weight: 600;
            color: #111827;
            margin: 0;
            line-height: 1.25;
        }}

        /* Close Button */
        .modal-close-button {{
            background: none;
            border: none;
            color: #6b7280;
            cursor: pointer;
            padding: 0.5rem;
            border-radius: 0.375rem;
            transition: all 0.2s ease;
            display: flex;
            align-items: center;
            justify-content: center;
            width: 2rem;
            height: 2rem;
            font-size: 1.25rem;
            font-weight: 300;
            line-height: 1;
        }}

        .modal-close-button:hover {{
            background: #f3f4f6;
            color: #374151;
        }}

        .modal-close-button:focus {{
            outline: 2px solid #3b82f6;
            outline-offset: 2px;
        }}

        .close-icon {{
            display: block;
            user-select: none;
        }}

        /* Modal Body */
        .modal-body {{
            padding: 1.5rem;
            overflow-y: auto;
            flex: 1;
            color: #374151;
            line-height: 1.625;
        }}

        .modal-content-wrapper {{
            text-align: center;
        }}

        .modal-message {{
            font-size: 1rem;
            color: #6b7280;
            margin: 0;
            line-height: 1.625;
        }}

        /* Modal Footer */
        .modal-footer {{
            padding: 1rem 1.5rem;
            border-top: 1px solid #e5e7eb;
            background: #f9fafb;
        }}

        .modal-actions {{
            display: flex;
            justify-content: flex-end;
            gap: 0.75rem;
            align-items: center;
        }}

        .modal-actions .btn {{
            min-width: 6.25rem;
        }}

        /* Focus Management */
        .modal:focus-within {{
            box-shadow: 0 0 0 2px #3b82f6, 0 25px 50px -12px rgba(0, 0, 0, 0.25);
        }}

        /* Skip to content link */
        .modal-skip-link {{
            position: absolute;
            top: -40px;
            left: 6px;
            background: #3b82f6;
            color: white;
            padding: 8px;
            text-decoration: none;
            border-radius: 4px;
            z-index: 1001;
            transition: top 0.3s;
        }}

        .modal-skip-link:focus {{
            top: 6px;
        }}

        /* Dark Theme */
        @media (prefers-color-scheme: dark) {{
            .modal {{
                background: #1f2937;
                border-color: #374151;
                box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
            }}

            .modal-header {{
                background: #111827;
                border-color: #374151;
            }}

            .modal-title {{
                color: #f9fafb;
            }}

            .modal-close-button {{
                color: #9ca3af;
            }}

            .modal-close-button:hover {{
                background: #374151;
                color: #f9fafb;
            }}

            .modal-body {{
                color: #d1d5db;
            }}

            .modal-footer {{
                background: #111827;
                border-color: #374151;
            }}

            .modal-message {{
                color: #9ca3af;
            }}

            .modal--alert {{
                border-top-color: #d97706;
            }}

            .modal--alert .modal-title {{
                color: #d97706;
            }}

            .modal--confirm {{
                border-top-color: #60a5fa;
            }}

            .modal--confirm .modal-title {{
                color: #60a5fa;
            }}

            .modal--success {{
                border-top-color: #34d399;
            }}

            .modal--success .modal-title {{
                color: #34d399;
            }}

            .modal--error {{
                border-top-color: #f87171;
            }}

            .modal--error .modal-title {{
                color: #f87171;
            }}

            .modal--info {{
                border-top-color: #60a5fa;
            }}

            .modal--info .modal-title {{
                color: #60a5fa;
            }}
        }}

        /* High Contrast Mode */
        @media (prefers-contrast: high) {{
            .modal-overlay {{
                background: rgba(0, 0, 0, 0.9);
            }}

            .modal {{
                border-width: 2px;
                border-color: #000000;
                background: #ffffff;
            }}

            .modal-title {{
                color: #000000;
            }}

            .modal-close-button {{
                color: #000000;
                border: 1px solid #000000;
            }}

            .modal-close-button:hover {{
                background: #000000;
                color: #ffffff;
            }}

            .modal:focus-within {{
                outline: 2px solid #0000ff;
            }}
        }}

        /* Reduced Motion */
        @media (prefers-reduced-motion: reduce) {{
            .modal-overlay,
            .modal,
            .modal-close-button {{
                animation: none;
                transition: none;
            }}

            .modal-overlay {{
                opacity: 1;
            }}

            .modal {{
                opacity: 1;
                transform: none;
            }}
        }}

        /* Mobile Responsive */
        @media (max-width: 640px) {{
            .modal-overlay {{
                padding: 0.5rem;
            }}

            .modal--small,
            .modal--medium,
            .modal--large {{
                max-width: 100%;
                width: 100%;
            }}

            .modal-header {{
                padding: 1rem;
            }}

            .modal-title {{
                font-size: 1rem;
            }}

            .modal-body {{
                padding: 1rem;
            }}

            .modal-footer {{
                padding: 0.75rem 1rem;
            }}

            .modal-actions {{
                flex-direction: column-reverse;
                gap: 0.5rem;
            }}

            .modal-actions .btn {{
                width: 100%;
                min-width: auto;
            }}
        }}

        /* Custom Scrollbar */
        .modal-body::-webkit-scrollbar {{
            width: 6px;
        }}

        .modal-body::-webkit-scrollbar-track {{
            background: #f3f4f6;
        }}

        .modal-body::-webkit-scrollbar-thumb {{
            background: #d1d5db;
            border-radius: 3px;
        }}

        .modal-body::-webkit-scrollbar-thumb:hover {{
            background: #9ca3af;
        }}

        /* Backdrop blur support */
        @supports (backdrop-filter: blur(4px)) {{
            .modal-overlay {{
                backdrop-filter: blur(4px);
            }}
        }}

        /* Fallback for browsers without backdrop-filter */
        @supports not (backdrop-filter: blur(4px)) {{
            .modal-overlay {{
                background: rgba(0, 0, 0, 0.8);
            }}
        }}

        /* Animations */
        {}
        {}
        {}
        {}
        "#,
        sizes::SMALL,
        sizes::MEDIUM,
        sizes::LARGE,
        sizes::FULLSCREEN,
        sizes::AUTO,
        animations::MODAL_OVERLAY_FADE_IN,
        animations::MODAL_OVERLAY_FADE_OUT,
        animations::MODAL_APPEAR,
        animations::MODAL_DISAPPEAR
    )
}
