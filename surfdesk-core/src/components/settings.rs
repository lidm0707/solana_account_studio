//! # Settings Component
//!
//! Settings component for managing application preferences,
//! configuration, and user settings across all platforms.

use crate::components::{
    combine_classes, Button, Card, CommonProps, Form, FormField, Size, Variant,
};
use dioxus::prelude::*;

/// Settings component properties
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SettingsProps {
    /// Common component properties
    #[props(optional)]
    pub common: Option<CommonProps>,

    /// Current theme
    #[props(optional)]
    pub theme: Option<String>,

    /// Current language
    #[props(optional)]
    pub language: Option<String>,

    /// Auto-save setting
    #[props(optional)]
    pub auto_save: Option<bool>,

    /// Notification settings
    #[props(optional)]
    pub notifications_enabled: Option<bool>,

    /// Settings change handler
    #[props(optional)]
    pub on_settings_change: Option<EventHandler<SettingsData>>,
}

/// Settings data structure
#[derive(Debug, Clone)]
pub struct SettingsData {
    pub theme: String,
    pub language: String,
    pub auto_save: bool,
    pub notifications_enabled: bool,
}

/// Settings component
#[component]
pub fn Settings(props: SettingsProps) -> Element {
    let common = props.common.unwrap_or_default();
    let theme = props.theme.unwrap_or_else(|| "dark".to_string());
    let language = props.language.unwrap_or_else(|| "en".to_string());
    let auto_save = props.auto_save.unwrap_or(true);
    let notifications_enabled = props.notifications_enabled.unwrap_or(true);

    let mut settings = use_signal(|| SettingsData {
        theme: theme.clone(),
        language: language.clone(),
        auto_save,
        notifications_enabled,
    });

    let mut classes = vec!["settings"];
    if let Some(class) = &common.class {
        classes.push(class);
    }

    let handle_submit = move |data: crate::components::FormSubmitData| {
        let new_settings = SettingsData {
            theme: data.values.get("theme").cloned().unwrap_or_default(),
            language: data.values.get("language").cloned().unwrap_or_default(),
            auto_save: data
                .values
                .get("auto_save")
                .map(|v| v == "true")
                .unwrap_or(false),
            notifications_enabled: data
                .values
                .get("notifications_enabled")
                .map(|v| v == "true")
                .unwrap_or(false),
        };

        settings.set(new_settings.clone());

        if let Some(on_settings_change) = props.on_settings_change {
            on_settings_change(new_settings);
        }
    };

    rsx! {
        div {
            class: combine_classes(&classes),
            id: common.id,

            // Settings header
            div { class: "settings-header",
                h1 { class: "settings-title", "Settings" }
                p { class: "settings-description", "Manage your application preferences" }
            }

            // Settings sections
            div { class: "settings-content",
                // Appearance settings
                Card {
                    title: Some("Appearance".to_string()),
                    variant: Variant::Default,
                    Form {
                        title: Some("Appearance Settings".to_string()),
                        fields: Some(vec![
                            FormField {
                                name: "theme".to_string(),
                                label: "Theme".to_string(),
                                field_type: crate::components::FormFieldType::Select(vec![
                                    crate::components::SelectOption {
                                        value: "light".to_string(),
                                        label: "Light".to_string(),
                                        disabled: false,
                                    },
                                    crate::components::SelectOption {
                                        value: "dark".to_string(),
                                        label: "Dark".to_string(),
                                        disabled: false,
                                    },
                                    crate::components::SelectOption {
                                        value: "auto".to_string(),
                                        label: "Auto".to_string(),
                                        disabled: false,
                                    },
                                ]),
                                value: theme,
                                required: false,
                                placeholder: None,
                                validation: None,
                                help_text: Some("Choose your preferred theme".to_string()),
                                disabled: false,
                            },
                            FormField {
                                name: "language".to_string(),
                                label: "Language".to_string(),
                                field_type: crate::components::FormFieldType::Select(vec![
                                    crate::components::SelectOption {
                                        value: "en".to_string(),
                                        label: "English".to_string(),
                                        disabled: false,
                                    },
                                    crate::components::SelectOption {
                                        value: "es".to_string(),
                                        label: "Español".to_string(),
                                        disabled: false,
                                    },
                                    crate::components::SelectOption {
                                        value: "fr".to_string(),
                                        label: "Français".to_string(),
                                        disabled: false,
                                    },
                                ]),
                                value: language,
                                required: false,
                                placeholder: None,
                                validation: None,
                                help_text: Some("Select your preferred language".to_string()),
                                disabled: false,
                            },
                        ]),
                        on_submit: Some(handle_submit),
                        submit_text: Some("Save Settings".to_string()),
                        children: rsx! { "" },
                    }
                }

                // Behavior settings
                Card {
                    title: Some("Behavior".to_string()),
                    variant: Variant::Default,
                    Form {
                        fields: Some(vec![
                            FormField {
                                name: "auto_save".to_string(),
                                label: "Auto-save".to_string(),
                                field_type: crate::components::FormFieldType::Checkbox,
                                value: auto_save.to_string(),
                                required: false,
                                placeholder: None,
                                validation: None,
                                help_text: Some("Automatically save your work".to_string()),
                                disabled: false,
                            },
                            FormField {
                                name: "notifications_enabled".to_string(),
                                label: "Enable Notifications".to_string(),
                                field_type: crate::components::FormFieldType::Checkbox,
                                value: notifications_enabled.to_string(),
                                required: false,
                                placeholder: None,
                                validation: None,
                                help_text: Some("Receive notifications about important events".to_string()),
                                disabled: false,
                            },
                        ]),
                        on_submit: Some(handle_submit),
                        submit_text: Some("Save Settings".to_string()),
                        children: rsx! { "" },
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_default_props() {
        let props = SettingsProps {
            common: None,
            theme: None,
            language: None,
            auto_save: None,
            notifications_enabled: None,
            on_settings_change: None,
        };

        assert_eq!(props.theme.unwrap_or_else(|| "dark".to_string()), "dark");
        assert_eq!(props.language.unwrap_or_else(|| "en".to_string()), "en");
        assert!(props.auto_save.unwrap_or(true));
        assert!(props.notifications_enabled.unwrap_or(true));
    }

    #[test]
    fn test_settings_data() {
        let data = SettingsData {
            theme: "light".to_string(),
            language: "es".to_string(),
            auto_save: false,
            notifications_enabled: true,
        };

        assert_eq!(data.theme, "light");
        assert_eq!(data.language, "es");
        assert!(!data.auto_save);
        assert!(data.notifications_enabled);
    }
}
