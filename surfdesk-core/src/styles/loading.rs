//! # Loading Styles Module
//!
//! Loading animation and spinner styling for SurfDesk application.
//! Provides consistent loading indicator styling across all platforms.

use dioxus::prelude::*;

/// CSS for loading components
pub const CSS_STYLES: &str = include_str!("loading.css");

/// Loading style constants
pub mod constants {
    pub const LOADING_SPINNER_SIZE: &str = "1.5rem";
    pub const LOADING_SPINNER_COLOR: &str = "#3b82f6";
    pub const LOADING_OVERLAY_Z_INDEX: &str = "999";
    pub const LOADING_SKELETON_BG: &str = "#e5e7eb";
    pub const LOADING_SKELETON_ANIMATION: &str = "skeleton-loading 1.5s ease-in-out infinite";
}

/// Loading style utilities
pub mod utils {
    use super::*;

    /// Generate spinner styles
    pub fn spinner_styles(size: &str, color: &str) -> String {
        format!(
            r#"
            width: {};
            height: {};
            border: 2px solid transparent;
            border-top: 2px solid {};
            border-radius: 50%;
            animation: spin 1s linear infinite;
            "#,
            size, size, color
        )
    }

    /// Generate skeleton loading styles
    pub fn skeleton_styles() -> String {
        "
            background: linear-gradient(90deg, #f3f4f6 25%, #e5e7eb 50%, #f3f4f6 75%);
            background-size: 200% 100%;
            animation: skeleton-loading 1.5s ease-in-out infinite;
            border-radius: 0.375rem;
        "
        .to_string()
    }

    /// Generate overlay loading styles
    pub fn overlay_styles() -> String {
        "
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: rgba(255, 255, 255, 0.8);
            backdrop-filter: blur(2px);
            display: flex;
            align-items: center;
            justify-content: center;
            z-index: 999;
        "
        .to_string()
    }
}

/// Loading animation keyframes
pub mod animations {
    pub const SPIN_ANIMATION: &str = r#"
        @keyframes spin {
            from { transform: rotate(0deg); }
            to { transform: rotate(360deg); }
        }
    "#;

    pub const SKELETON_LOADING: &str = r#"
        @keyframes skeleton-loading {
            0% { background-position: 200% 0; }
            100% { background-position: -200% 0; }
        }
    "#;

    pub const PULSE_ANIMATION: &str = r#"
        @keyframes pulse {
            0%, 100% { opacity: 1; }
            50% { opacity: 0.5; }
        }
    "#;

    pub const BOUNCE_ANIMATION: &str = r#"
        @keyframes bounce {
            0%, 20%, 53%, 80%, 100% { transform: translate3d(0, 0, 0); }
            40%, 43% { transform: translate3d(0, -8px, 0); }
            70% { transform: translate3d(0, -4px, 0); }
            90% { transform: translate3d(0, -2px, 0); }
        }
    "#;
}

/// Get all loading styles as a single string
pub fn get_all_styles() -> String {
    format!(
        r#"
        /* Loading Component Styles */

        /* Loading Spinner */
        .loading-spinner {{
            width: 1.5rem;
            height: 1.5rem;
            border: 2px solid transparent;
            border-top: 2px solid #3b82f6;
            border-radius: 50%;
            animation: spin 1s linear infinite;
        }}

        .loading-spinner--small {{
            width: 1rem;
            height: 1rem;
            border-width: 1.5px;
        }}

        .loading-spinner--large {{
            width: 2rem;
            height: 2rem;
            border-width: 3px;
        }}

        .loading-spinner--primary {{
            border-top-color: #3b82f6;
        }}

        .loading-spinner--secondary {{
            border-top-color: #6b7280;
        }}

        .loading-spinner--success {{
            border-top-color: #22c55e;
        }}

        .loading-spinner--error {{
            border-top-color: #ef4444;
        }}

        .loading-spinner--warning {{
            border-top-color: #f59e0b;
        }}

        /* Loading Dots */
        .loading-dots {{
            display: flex;
            gap: 0.25rem;
            align-items: center;
        }}

        .loading-dot {{
            width: 0.5rem;
            height: 0.5rem;
            border-radius: 50%;
            background: #3b82f6;
            animation: loading-dot-pulse 1.4s ease-in-out infinite both;
        }}

        .loading-dot:nth-child(1) {{ animation-delay: -0.32s; }}
        .loading-dot:nth-child(2) {{ animation-delay: -0.16s; }}

        @keyframes loading-dot-pulse {{
            0%, 80%, 100% {{
                transform: scale(0.8);
                opacity: 0.5;
            }}
            40% {{
                transform: scale(1);
                opacity: 1;
            }}
        }}

        /* Loading Bar */
        .loading-bar {{
            width: 100%;
            height: 0.25rem;
            background: #e5e7eb;
            border-radius: 0.125rem;
            overflow: hidden;
        }}

        .loading-bar-progress {{
            height: 100%;
            background: #3b82f6;
            border-radius: 0.125rem;
            animation: loading-bar-progress 2s ease-in-out infinite;
        }}

        @keyframes loading-bar-progress {{
            0% {{ width: 0%; }}
            50% {{ width: 70%; }}
            100% {{ width: 100%; }}
        }}

        /* Skeleton Loading */
        .skeleton {{
            background: linear-gradient(90deg, #f3f4f6 25%, #e5e7eb 50%, #f3f4f6 75%);
            background-size: 200% 100%;
            animation: skeleton-loading 1.5s ease-in-out infinite;
            border-radius: 0.375rem;
        }}

        .skeleton--text {{
            height: 1rem;
            margin-bottom: 0.5rem;
        }}

        .skeleton--text:last-child {{
            margin-bottom: 0;
        }}

        .skeleton--title {{
            height: 1.5rem;
            width: 60%;
            margin-bottom: 0.75rem;
        }}

        .skeleton--avatar {{
            width: 2.5rem;
            height: 2.5rem;
            border-radius: 50%;
        }}

        .skeleton--button {{
            height: 2.5rem;
            width: 6rem;
        }}

        .skeleton--card {{
            padding: 1rem;
            border: 1px solid #e5e7eb;
            border-radius: 0.5rem;
        }}

        /* Loading Overlay */
        .loading-overlay {{
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: rgba(255, 255, 255, 0.8);
            backdrop-filter: blur(2px);
            display: flex;
            align-items: center;
            justify-content: center;
            z-index: 999;
        }}

        .loading-overlay--dark {{
            background: rgba(0, 0, 0, 0.8);
        }}

        .loading-overlay-content {{
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 1rem;
            padding: 2rem;
            background: white;
            border-radius: 0.5rem;
            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
        }}

        .loading-overlay-text {{
            font-size: 0.875rem;
            color: #6b7280;
            font-weight: 500;
        }}

        /* Inline Loading */
        .inline-loading {{
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
        }}

        .inline-loading--button {{
            position: relative;
        }}

        .inline-loading--button .loading-spinner {{
            position: absolute;
            left: 0.75rem;
            top: 50%;
            transform: translateY(-50%);
        }}

        .inline-loading--button button {{
            padding-left: 2.5rem;
        }}

        /* Progress Ring */
        .progress-ring {{
            width: 3rem;
            height: 3rem;
            transform: rotate(-90deg);
        }}

        .progress-ring-circle {{
            stroke: #e5e7eb;
            stroke-width: 0.25rem;
            fill: transparent;
        }}

        .progress-ring-progress {{
            stroke: #3b82f6;
            stroke-width: 0.25rem;
            fill: transparent;
            stroke-dasharray: 157;
            stroke-dashoffset: 157;
            animation: progress-ring 2s ease-in-out infinite;
        }}

        @keyframes progress-ring {{
            to {{
                stroke-dashoffset: 0;
            }}
        }}

        /* Dark Theme */
        @media (prefers-color-scheme: dark) {{
            .loading-overlay {{
                background: rgba(0, 0, 0, 0.8);
            }}

            .loading-overlay-content {{
                background: #1f2937;
                color: #f9fafb;
            }}

            .loading-overlay-text {{
                color: #9ca3af;
            }}

            .skeleton {{
                background: linear-gradient(90deg, #374151 25%, #4b5563 50%, #374151 75%);
            }}

            .skeleton--card {{
                border-color: #374151;
            }}

            .loading-bar {{
                background: #374151;
            }}

            .loading-bar-progress {{
                background: #3b82f6;
            }}

            .progress-ring-circle {{
                stroke: #4b5563;
            }}
        }}

        /* High Contrast Mode */
        @media (prefers-contrast: high) {{
            .loading-spinner {{
                border-top-color: #0000ff;
                border-width: 3px;
            }}

            .loading-overlay {{
                background: rgba(255, 255, 255, 0.95);
            }}

            .skeleton {{
                background: #000000;
                opacity: 0.2;
            }}
        }}

        /* Reduced Motion */
        @media (prefers-reduced-motion: reduce) {{
            .loading-spinner,
            .loading-dot,
            .loading-bar-progress,
            .skeleton,
            .progress-ring-progress {{
                animation: none;
            }}

            .loading-overlay {{
                backdrop-filter: none;
            }}
        }}

        /* Mobile Responsive */
        @media (max-width: 640px) {{
            .loading-overlay-content {{
                padding: 1.5rem;
                margin: 1rem;
            }}

            .loading-spinner {{
                width: 1.25rem;
                height: 1.25rem;
            }}

            .progress-ring {{
                width: 2.5rem;
                height: 2.5rem;
            }}
        }}

        /* Animations */
        {}
        {}
        {}
        {}
        "#,
        animations::SPIN_ANIMATION,
        animations::SKELETON_LOADING,
        animations::PULSE_ANIMATION,
        animations::BOUNCE_ANIMATION
    )
}
