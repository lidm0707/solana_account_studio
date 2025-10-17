//! # Keyboard Shortcuts Module
//!
//! Comprehensive keyboard shortcut system for the SurfDesk desktop application.
//! Provides global hotkeys, action handlers, and shortcut management.

#![allow(dead_code)]

use dioxus::prelude::*;
use log::{debug, info};
use std::collections::HashMap;
// use web_sys::KeyboardEvent; // Remove web_sys dependency for now

/// Keyboard shortcut definition
#[derive(Debug, Clone, PartialEq)]
pub struct Shortcut {
    /// Key combination (e.g., "Ctrl+N")
    pub combination: String,
    /// Action description
    pub description: String,
    /// Action category
    pub category: ShortcutCategory,
    /// Whether the shortcut is enabled
    pub enabled: bool,
}

/// Shortcut categories for organization
#[derive(Debug, Clone, PartialEq)]
pub enum ShortcutCategory {
    /// File operations
    File,
    /// Edit operations
    Edit,
    /// View operations
    View,
    /// Navigation
    Navigation,
    /// Account operations
    Account,
    /// Transaction operations
    Transaction,
    /// Window operations
    Window,
    /// Help operations
    Help,
}

/// Keyboard shortcut action types
#[derive(Debug, Clone, PartialEq)]
pub enum ShortcutAction {
    /// Create new account
    CreateAccount,
    /// Save current state
    Save,
    /// Open file/account
    Open,
    /// Close current window/dialog
    Close,
    /// Navigate to dashboard
    GoToDashboard,
    /// Navigate to accounts
    GoToAccounts,
    /// Navigate to transactions
    GoToTransactions,
    /// Navigate to SurfPool
    GoToSurfPool,
    /// Navigate to settings
    GoToSettings,
    /// Navigate to analytics
    GoToAnalytics,
    /// Request airdrop
    RequestAirdrop,
    /// Toggle theme
    ToggleTheme,
    /// Show help
    ShowHelp,
    /// Show keyboard shortcuts
    ShowShortcuts,
    /// Focus search
    FocusSearch,
    /// Refresh data
    Refresh,
}

/// Keyboard shortcuts manager
#[derive(Clone)]
pub struct KeyboardManager {
    /// Registered shortcuts
    shortcuts: HashMap<String, Shortcut>,
    /// Action handlers
    handlers: HashMap<String, ShortcutAction>,
    /// Enabled state
    enabled: bool,
}

impl Default for KeyboardManager {
    fn default() -> Self {
        let mut manager = Self {
            shortcuts: HashMap::new(),
            handlers: HashMap::new(),
            enabled: true,
        };
        manager.register_default_shortcuts();
        manager
    }
}

impl KeyboardManager {
    /// Create new keyboard manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Register default keyboard shortcuts
    fn register_default_shortcuts(&mut self) {
        let default_shortcuts = vec![
            // File operations
            Shortcut {
                combination: "Ctrl+N".to_string(),
                description: "Create new account".to_string(),
                category: ShortcutCategory::File,
                enabled: true,
            },
            Shortcut {
                combination: "Ctrl+O".to_string(),
                description: "Open/import account".to_string(),
                category: ShortcutCategory::File,
                enabled: true,
            },
            Shortcut {
                combination: "Ctrl+S".to_string(),
                description: "Save current state".to_string(),
                category: ShortcutCategory::File,
                enabled: true,
            },
            Shortcut {
                combination: "Ctrl+W".to_string(),
                description: "Close current window".to_string(),
                category: ShortcutCategory::File,
                enabled: true,
            },
            Shortcut {
                combination: "Ctrl+Q".to_string(),
                description: "Quit application".to_string(),
                category: ShortcutCategory::File,
                enabled: true,
            },
            // Navigation
            Shortcut {
                combination: "Ctrl+1".to_string(),
                description: "Go to Dashboard".to_string(),
                category: ShortcutCategory::Navigation,
                enabled: true,
            },
            Shortcut {
                combination: "Ctrl+2".to_string(),
                description: "Go to Accounts".to_string(),
                category: ShortcutCategory::Navigation,
                enabled: true,
            },
            Shortcut {
                combination: "Ctrl+3".to_string(),
                description: "Go to Transactions".to_string(),
                category: ShortcutCategory::Navigation,
                enabled: true,
            },
            Shortcut {
                combination: "Ctrl+4".to_string(),
                description: "Go to SurfPool".to_string(),
                category: ShortcutCategory::Navigation,
                enabled: true,
            },
            Shortcut {
                combination: "Ctrl+5".to_string(),
                description: "Go to Analytics".to_string(),
                category: ShortcutCategory::Navigation,
                enabled: true,
            },
            Shortcut {
                combination: "Ctrl+,".to_string(),
                description: "Go to Settings".to_string(),
                category: ShortcutCategory::Navigation,
                enabled: true,
            },
            // Account operations
            Shortcut {
                combination: "Ctrl+Shift+A".to_string(),
                description: "Request airdrop".to_string(),
                category: ShortcutCategory::Account,
                enabled: true,
            },
            Shortcut {
                combination: "Ctrl+Shift+R".to_string(),
                description: "Refresh accounts".to_string(),
                category: ShortcutCategory::Account,
                enabled: true,
            },
            // View operations
            Shortcut {
                combination: "Ctrl+Shift+T".to_string(),
                description: "Toggle theme".to_string(),
                category: ShortcutCategory::View,
                enabled: true,
            },
            Shortcut {
                combination: "F11".to_string(),
                description: "Toggle fullscreen".to_string(),
                category: ShortcutCategory::View,
                enabled: true,
            },
            Shortcut {
                combination: "Ctrl+0".to_string(),
                description: "Reset zoom".to_string(),
                category: ShortcutCategory::View,
                enabled: true,
            },
            // Search and help
            Shortcut {
                combination: "Ctrl+F".to_string(),
                description: "Focus search".to_string(),
                category: ShortcutCategory::Help,
                enabled: true,
            },
            Shortcut {
                combination: "Ctrl+Shift+F".to_string(),
                description: "Global search".to_string(),
                category: ShortcutCategory::Help,
                enabled: true,
            },
            Shortcut {
                combination: "F1".to_string(),
                description: "Show help".to_string(),
                category: ShortcutCategory::Help,
                enabled: true,
            },
            Shortcut {
                combination: "Ctrl+?".to_string(),
                description: "Show keyboard shortcuts".to_string(),
                category: ShortcutCategory::Help,
                enabled: true,
            },
            // System operations
            Shortcut {
                combination: "Ctrl+R".to_string(),
                description: "Refresh data".to_string(),
                category: ShortcutCategory::Window,
                enabled: true,
            },
            Shortcut {
                combination: "Ctrl+Shift+R".to_string(),
                description: "Hard refresh".to_string(),
                category: ShortcutCategory::Window,
                enabled: true,
            },
            Shortcut {
                combination: "Escape".to_string(),
                description: "Cancel/Close dialog".to_string(),
                category: ShortcutCategory::Window,
                enabled: true,
            },
        ];

        for shortcut in default_shortcuts {
            let combination = shortcut.combination.clone();
            self.shortcuts.insert(combination.clone(), shortcut);

            // Map to actions
            let action = match combination.as_str() {
                "Ctrl+N" => ShortcutAction::CreateAccount,
                "Ctrl+O" => ShortcutAction::Open,
                "Ctrl+S" => ShortcutAction::Save,
                "Ctrl+W" => ShortcutAction::Close,
                "Ctrl+1" => ShortcutAction::GoToDashboard,
                "Ctrl+2" => ShortcutAction::GoToAccounts,
                "Ctrl+3" => ShortcutAction::GoToTransactions,
                "Ctrl+4" => ShortcutAction::GoToSurfPool,
                "Ctrl+5" => ShortcutAction::GoToAnalytics,
                "Ctrl+," => ShortcutAction::GoToSettings,
                "Ctrl+Shift+A" => ShortcutAction::RequestAirdrop,
                "Ctrl+Shift+T" => ShortcutAction::ToggleTheme,
                "Ctrl+F" => ShortcutAction::FocusSearch,
                "F1" => ShortcutAction::ShowHelp,
                "Ctrl+?" => ShortcutAction::ShowShortcuts,
                "Ctrl+R" => ShortcutAction::Refresh,
                "Escape" => ShortcutAction::Close,
                _ => continue,
            };

            self.handlers.insert(combination, action);
        }

        info!(
            "Registered {} default keyboard shortcuts",
            self.shortcuts.len()
        );
    }

    /// Register a new keyboard shortcut
    pub fn register_shortcut(&mut self, shortcut: Shortcut, action: ShortcutAction) {
        let combination = shortcut.combination.clone();
        self.shortcuts.insert(combination.clone(), shortcut);
        self.handlers.insert(combination.clone(), action);
        debug!("Registered keyboard shortcut: {}", combination);
    }

    /// Unregister a keyboard shortcut
    pub fn unregister_shortcut(&mut self, combination: &str) {
        self.shortcuts.remove(combination);
        self.handlers.remove(combination);
        debug!("Unregistered keyboard shortcut: {}", combination);
    }

    /// Enable or disable keyboard shortcuts
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        info!(
            "Keyboard shortcuts {}",
            if enabled { "enabled" } else { "disabled" }
        );
    }

    /// Check if keyboard shortcuts are enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Handle keyboard event
    pub fn handle_key_event(&self, event: &KeyboardEvent) -> Option<ShortcutAction> {
        if !self.enabled {
            return None;
        }

        let combination = self.key_event_to_combination(event);

        if let Some(action) = self.handlers.get(&combination) {
            debug!("Keyboard shortcut triggered: {}", combination);
            Some(action.clone())
        } else {
            None
        }
    }

    /// Convert keyboard event to combination string
    fn key_event_to_combination(&self, event: &dioxus::prelude::Event<KeyboardData>) -> String {
        let mut parts: Vec<String> = Vec::new();

        // Dioxus keyboard API - check modifiers
        // Note: Dioxus doesn't provide direct modifier key detection
        // For now, we'll handle simple key combinations
        let key_str = format!("{:?}", event.data.key());

        // Simplified key mapping for basic keys
        match key_str.as_str() {
            s if s.contains("Control") => parts.push("Ctrl".to_string()),
            s if s.contains("Shift") => parts.push("Shift".to_string()),
            s if s.contains("Alt") => parts.push("Alt".to_string()),
            s if s.contains("Meta") => parts.push("Meta".to_string()),
            _ => {}
        }

        // Main key handling
        let key = format!("{:?}", event.data.key());
        match key.as_str() {
            s if s.contains("Escape") => parts.push("Escape".to_string()),
            s if s.contains("Enter") => parts.push("Enter".to_string()),
            s if s.contains("Tab") => parts.push("Tab".to_string()),
            s if s.contains("Backspace") => parts.push("Backspace".to_string()),
            s if s.contains("Delete") => parts.push("Delete".to_string()),
            s if s.contains("Home") => parts.push("Home".to_string()),
            s if s.contains("End") => parts.push("End".to_string()),
            s if s.contains("PageUp") => parts.push("PageUp".to_string()),
            s if s.contains("PageDown") => parts.push("PageDown".to_string()),
            s if s.contains("ArrowUp") => parts.push("Up".to_string()),
            s if s.contains("ArrowDown") => parts.push("Down".to_string()),
            s if s.contains("ArrowLeft") => parts.push("Left".to_string()),
            s if s.contains("ArrowRight") => parts.push("Right".to_string()),
            s if s.contains("F1")
                | s.contains("F2")
                | s.contains("F3")
                | s.contains("F4")
                | s.contains("F5")
                | s.contains("F6")
                | s.contains("F7")
                | s.contains("F8")
                | s.contains("F9")
                | s.contains("F10")
                | s.contains("F11")
                | s.contains("F12") =>
            {
                parts.push(key.clone());
            }
            // Regular keys - extract the character if possible
            _ => {
                // Try to extract single character from the debug representation
                if let Some(c) = key.chars().next() {
                    if c.is_alphabetic() {
                        parts.push(c.to_uppercase().to_string());
                    } else if c.is_numeric() {
                        parts.push(c.to_string());
                    } else {
                        parts.push(key.clone());
                    }
                } else {
                    parts.push(key.clone());
                }
            }
        }

        parts.join("+")
    }

    /// Get all shortcuts
    pub fn get_all_shortcuts(&self) -> Vec<&Shortcut> {
        self.shortcuts.values().collect()
    }

    /// Get shortcuts by category
    pub fn get_shortcuts_by_category(&self, category: ShortcutCategory) -> Vec<&Shortcut> {
        self.shortcuts
            .values()
            .filter(|shortcut| shortcut.category == category)
            .collect()
    }

    /// Get shortcut by combination
    pub fn get_shortcut(&self, combination: &str) -> Option<&Shortcut> {
        self.shortcuts.get(combination)
    }

    /// Search shortcuts
    pub fn search_shortcuts(&self, query: &str) -> Vec<&Shortcut> {
        let query_lower = query.to_lowercase();
        self.shortcuts
            .values()
            .filter(|shortcut| {
                shortcut.combination.to_lowercase().contains(&query_lower)
                    || shortcut.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
}

/// Dioxus hook for keyboard shortcuts
pub fn use_keyboard_shortcuts() -> Signal<KeyboardManager> {
    use_signal(KeyboardManager::new)
}

/// Keyboard shortcuts provider component
#[component]
pub fn KeyboardShortcutsProvider(
    children: Element,
    on_action: EventHandler<ShortcutAction>,
) -> Element {
    let keyboard_manager = use_keyboard_shortcuts();

    // Global keyboard event listener
    use_coroutine(move |_: UnboundedReceiver<()>| {
        let _manager = keyboard_manager;
        let _action_handler = on_action;

        async move {
            // This would typically use a proper event listener
            // For now, we'll use a timer-based approach
            let mut interval = tokio::time::interval(std::time::Duration::from_millis(100));

            loop {
                interval.tick().await;
                // In a real implementation, this would listen for keyboard events
                // and call manager.handle_key_event(event)
            }
        }
    });

    rsx! {
        {children}
    }
}

/// Keyboard shortcuts display component
#[component]
pub fn KeyboardShortcutsDialog(
    show: Signal<bool>,
    keyboard_manager: Signal<KeyboardManager>,
) -> Element {
    let mut search_query = use_signal(|| String::new());
    let mut selected_category = use_signal(|| Option::<ShortcutCategory>::None);

    let categories = [
        ShortcutCategory::File,
        ShortcutCategory::Edit,
        ShortcutCategory::View,
        ShortcutCategory::Navigation,
        ShortcutCategory::Account,
        ShortcutCategory::Transaction,
        ShortcutCategory::Window,
        ShortcutCategory::Help,
    ];

    let mut filtered_shortcuts = use_signal(|| Vec::<Shortcut>::new());

    // Update filtered shortcuts when search or category changes
    use_effect(move || {
        let manager = keyboard_manager();
        let query = search_query();
        let category = selected_category();

        let shortcuts = if !query.is_empty() {
            manager.search_shortcuts(&query)
        } else if let Some(cat) = category {
            manager.get_shortcuts_by_category(cat)
        } else {
            manager.get_all_shortcuts()
        };

        filtered_shortcuts.set(shortcuts.into_iter().cloned().collect());
    });

    rsx! {
        if show() {
            div { class: "keyboard-shortcuts-dialog",

                // Dialog overlay
                div {
                    class: "dialog-overlay",
                    onclick: move |_| show.set(false)
                }

                // Dialog content
                div { class: "dialog-content",

                    // Header
                    div { class: "dialog-header",
                        h2 { "Keyboard Shortcuts" }
                        button {
                            class: "close-button",
                            onclick: move |_| show.set(false),
                            "×"
                        }
                    }

                    // Search and filter
                    div { class: "dialog-controls",
                        input {
                            class: "search-input",
                            placeholder: "Search shortcuts...",
                            value: "{search_query}",
                            oninput: move |evt| search_query.set(evt.value()),
                        }

                        select {
                            class: "category-filter",
                            value: selected_category().map_or(String::new(), |c| format!("{:?}", c)),
                            onchange: move |evt| {
                                let value = evt.value();
                                selected_category.set(if value.is_empty() {
                                    None
                                } else {
                                    // Parse category from string (simplified)
                                    match value.as_str() {
                                        "File" => Some(ShortcutCategory::File),
                                        "Navigation" => Some(ShortcutCategory::Navigation),
                                        "Account" => Some(ShortcutCategory::Account),
                                        "View" => Some(ShortcutCategory::View),
                                        "Help" => Some(ShortcutCategory::Help),
                                        _ => None,
                                    }
                                });
                            },
                            option { value: "", "All Categories" }
                            option { value: "File", "File" }
                            option { value: "Navigation", "Navigation" }
                            option { value: "Account", "Account" }
                            option { value: "View", "View" }
                            option { value: "Help", "Help" }
                        }
                    }

                    // Shortcuts list
                    div { class: "shortcuts-list",
                        for shortcut in filtered_shortcuts() {
                            div { class: "shortcut-item",
                                div { class: "shortcut-combination",
                                    "{shortcut.combination}"
                                }
                                div { class: "shortcut-description",
                                    "{shortcut.description}"
                                }
                            }
                        }
                    }

                    // Footer
                    div { class: "dialog-footer",
                        p { "Press Escape to close this dialog" }
                    }
                }
            }
        }
    }
}

/// Utility function to format shortcut combination for display
pub fn format_shortcut_combination(combination: &str) -> String {
    combination
        .replace("Ctrl", "⌃")
        .replace("Shift", "⇧")
        .replace("Alt", "⌥")
        .replace("Meta", "⌘")
        .replace("Enter", "↵")
        .replace("Escape", "Esc")
        .replace("ArrowUp", "↑")
        .replace("ArrowDown", "↓")
        .replace("ArrowLeft", "←")
        .replace("ArrowRight", "→")
}
