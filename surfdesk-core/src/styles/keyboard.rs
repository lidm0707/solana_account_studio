#![allow(dead_code)]
//! # Keyboard Styles Module
//!
//! Keyboard shortcut and navigation styling for SurfDesk application.
//! Provides consistent keyboard interaction styling across all platforms.

use dioxus::prelude::*;

/// CSS for keyboard components
pub const CSS_STYLES: &str = include_str!("keyboard.css");

/// Keyboard style constants
pub mod constants {
    pub const KEYBOARD_FOCUS_WIDTH: &str = "2px";
    pub const KEYBOARD_FOCUS_OFFSET: &str = "2px";
    pub const KEYBOARD_TRANSITION: &str = "outline 0.2s ease";
    pub const KEYBOARD_SHORTCUT_BG: &str = "#f3f4f6";
    pub const KEYBOARD_SHORTCUT_BORDER: &str = "#d1d5db";
    pub const KEYBOARD_SHORTCUT_TEXT: &str = "#374151";
}

/// Keyboard style utilities
pub mod utils {
    use super::*;

    /// Generate focus styles for keyboard navigation
    pub fn focus_styles() -> String {
        "
            outline: 2px solid #3b82f6;
            outline-offset: 2px;
            transition: outline 0.2s ease;
        "
        .to_string()
    }

    /// Generate keyboard shortcut styles
    pub fn shortcut_styles() -> String {
        "
            display: inline-flex;
            align-items: center;
            padding: 0.25rem 0.5rem;
            background: #f3f4f6;
            border: 1px solid #d1d5db;
            border-radius: 0.25rem;
            color: #374151;
            font-size: 0.75rem;
            font-weight: 500;
            font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
            box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
        "
        .to_string()
    }

    /// Generate keyboard navigation hint styles
    pub fn navigation_hint_styles() -> String {
        "
            position: fixed;
            bottom: 1rem;
            right: 1rem;
            background: rgba(31, 41, 55, 0.95);
            color: #f9fafb;
            padding: 0.75rem 1rem;
            border-radius: 0.5rem;
            font-size: 0.75rem;
            backdrop-filter: blur(8px);
            z-index: 1000;
            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
        "
        .to_string()
    }
}

/// Get all keyboard styles as a single string
pub fn get_all_styles() -> String {
    r#"
    /* Keyboard Navigation Styles */

    /* Focus Management */
    .keyboard-focusable {
        position: relative;
    }

    .keyboard-focusable:focus-visible {
        outline: 2px solid #3b82f6;
        outline-offset: 2px;
        transition: outline 0.2s ease;
    }

    /* Skip to Content Link */
    .skip-link {
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
    }

    .skip-link:focus {
        top: 6px;
    }

    /* Keyboard Shortcuts */
    .keyboard-shortcut {
        display: inline-flex;
        align-items: center;
        gap: 0.25rem;
        margin: 0 0.25rem;
    }

    .keyboard-key {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        min-width: 1.5rem;
        height: 1.5rem;
        padding: 0 0.5rem;
        background: #ffffff;
        border: 1px solid #d1d5db;
        border-radius: 0.25rem;
        color: #374151;
        font-size: 0.75rem;
        font-weight: 500;
        font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
        box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
        line-height: 1;
    }

    .keyboard-key--modifier {
        background: #f3f4f6;
        color: #6b7280;
    }

    .keyboard-key--active {
        background: #3b82f6;
        color: white;
        border-color: #3b82f6;
        transform: translateY(1px);
        box-shadow: none;
    }

    /* Keyboard Shortcuts Panel */
    .keyboard-shortcuts-panel {
        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background: #ffffff;
        border: 1px solid #e5e7eb;
        border-radius: 0.75rem;
        box-shadow: 0 25px 50px rgba(0, 0, 0, 0.25);
        max-width: 500px;
        width: 90%;
        max-height: 80vh;
        overflow: hidden;
        z-index: 1000;
    }

    .keyboard-shortcuts-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 1.5rem;
        border-bottom: 1px solid #e5e7eb;
        background: #f9fafb;
    }

    .keyboard-shortcuts-title {
        font-size: 1.125rem;
        font-weight: 600;
        color: #111827;
    }

    .keyboard-shortcuts-close {
        background: none;
        border: none;
        color: #6b7280;
        cursor: pointer;
        padding: 0.5rem;
        border-radius: 0.375rem;
        font-size: 1.25rem;
        line-height: 1;
        transition: all 0.2s ease;
    }

    .keyboard-shortcuts-close:hover {
        background: #f3f4f6;
        color: #374151;
    }

    .keyboard-shortcuts-content {
        padding: 1.5rem;
        max-height: 60vh;
        overflow-y: auto;
    }

    .keyboard-shortcuts-section {
        margin-bottom: 1.5rem;
    }

    .keyboard-shortcuts-section:last-child {
        margin-bottom: 0;
    }

    .keyboard-shortcuts-section-title {
        font-size: 0.875rem;
        font-weight: 600;
        color: #111827;
        margin-bottom: 0.75rem;
        text-transform: uppercase;
        letter-spacing: 0.05em;
    }

    .keyboard-shortcut-item {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 0.5rem 0;
        border-bottom: 1px solid #f3f4f6;
    }

    .keyboard-shortcut-item:last-child {
        border-bottom: none;
    }

    .keyboard-shortcut-description {
        font-size: 0.875rem;
        color: #374151;
        flex: 1;
    }

    .keyboard-shortcut-keys {
        display: flex;
        align-items: center;
        gap: 0.25rem;
    }

    /* Keyboard Navigation Indicator */
    .keyboard-nav-indicator {
        position: fixed;
        top: 1rem;
        right: 1rem;
        background: rgba(31, 41, 55, 0.95);
        color: #f9fafb;
        padding: 0.5rem 0.75rem;
        border-radius: 0.375rem;
        font-size: 0.75rem;
        font-weight: 500;
        backdrop-filter: blur(8px);
        z-index: 1000;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .keyboard-nav-indicator--hidden {
        opacity: 0;
        pointer-events: none;
        transition: opacity 0.3s ease;
    }

    /* Tab Navigation */
    .tab-focus-trap {
        outline: none;
    }

    .tab-focus-trap:focus-visible {
        outline: 2px solid #3b82f6;
        outline-offset: 2px;
    }

    /* Grid Navigation */
    .grid-navigation {
        display: grid;
        gap: 0.5rem;
    }

    .grid-navigation-item {
        padding: 0.75rem;
        border: 1px solid #e5e7eb;
        border-radius: 0.5rem;
        background: #ffffff;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .grid-navigation-item:hover,
    .grid-navigation-item:focus-visible {
        border-color: #3b82f6;
        background: #f0f9ff;
    }

    .grid-navigation-item:focus-visible {
        outline: 2px solid #3b82f6;
        outline-offset: 2px;
    }

    /* Menu Navigation */
    .menu-navigation {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    .menu-navigation-item {
        display: flex;
        align-items: center;
        padding: 0.5rem 0.75rem;
        border-radius: 0.375rem;
        color: #374151;
        text-decoration: none;
        transition: all 0.2s ease;
    }

    .menu-navigation-item:hover,
    .menu-navigation-item:focus-visible {
        background: #f3f4f6;
        color: #111827;
    }

    .menu-navigation-item:focus-visible {
        outline: 2px solid #3b82f6;
        outline-offset: -2px;
    }

    .menu-navigation-item--active {
        background: #dbeafe;
        color: #1e40af;
        font-weight: 500;
    }

    /* Dark Theme */
    @media (prefers-color-scheme: dark) {
        .keyboard-key {
            background: #1f2937;
            border-color: #374151;
            color: #f9fafb;
        }

        .keyboard-key--modifier {
            background: #374151;
            color: #9ca3af;
        }

        .keyboard-key--active {
            background: #3b82f6;
            color: white;
            border-color: #3b82f6;
        }

        .keyboard-shortcuts-panel {
            background: #1f2937;
            border-color: #374151;
        }

        .keyboard-shortcuts-header {
            background: #111827;
            border-color: #374151;
        }

        .keyboard-shortcuts-title {
            color: #f9fafb;
        }

        .keyboard-shortcuts-close {
            color: #9ca3af;
        }

        .keyboard-shortcuts-close:hover {
            background: #374151;
            color: #f9fafb;
        }

        .keyboard-shortcuts-section-title {
            color: #f9fafb;
        }

        .keyboard-shortcut-item {
            border-color: #374151;
        }

        .keyboard-shortcut-description {
            color: #d1d5db;
        }

        .keyboard-nav-indicator {
            background: rgba(17, 24, 39, 0.95);
            color: #f9fafb;
        }

        .grid-navigation-item {
            background: #1f2937;
            border-color: #374151;
            color: #f9fafb;
        }

        .grid-navigation-item:hover,
        .grid-navigation-item:focus-visible {
            border-color: #3b82f6;
            background: #1e3a8a;
        }

        .menu-navigation-item {
            color: #d1d5db;
        }

        .menu-navigation-item:hover,
        .menu-navigation-item:focus-visible {
            background: #374151;
            color: #f9fafb;
        }

        .menu-navigation-item--active {
            background: #1e3a8a;
            color: #dbeafe;
        }
    }

    /* High Contrast Mode */
    @media (prefers-contrast: high) {
        .keyboard-focusable:focus-visible {
            outline-width: 3px;
            outline-color: #0000ff;
        }

        .keyboard-key {
            border-width: 2px;
            border-color: #000000;
        }

        .keyboard-key--active {
            background: #0000ff;
            color: #ffffff;
            border-color: #0000ff;
        }
    }

    /* Reduced Motion */
    @media (prefers-reduced-motion: reduce) {
        .keyboard-focusable:focus-visible {
            transition: none;
        }

        .keyboard-key {
            transition: none;
        }

        .keyboard-nav-indicator {
            transition: none;
        }
    }

    /* Mobile Responsive */
    @media (max-width: 640px) {
        .keyboard-shortcuts-panel {
            width: 95%;
            max-height: 90vh;
        }

        .keyboard-shortcuts-header,
        .keyboard-shortcuts-content {
            padding: 1rem;
        }

        .keyboard-nav-indicator {
            top: 0.5rem;
            right: 0.5rem;
            font-size: 0.625rem;
            padding: 0.375rem 0.5rem;
        }

        .keyboard-key {
            min-width: 1.25rem;
            height: 1.25rem;
            font-size: 0.625rem;
            padding: 0 0.375rem;
        }
    }
    "#
    .to_string()
}
