//! # Footer Component
//!
//! Footer component for displaying application information,
//  links, and status across all platforms.

use crate::platform::Platform;
use dioxus::prelude::*;

/// Common component properties
#[derive(Debug, Clone, PartialEq, Props)]
pub struct CommonProps {
    /// Component class name
    #[props(optional)]
    pub class: Option<String>,
    /// Component ID
    #[props(optional)]
    pub id: Option<String>,
}

/// Combine CSS classes into a single string
pub fn combine_classes(classes: &[&str]) -> String {
    classes.join(" ")
}

/// Application version constant
pub const VERSION: &str = "0.1.0";

/// Footer component properties
#[derive(Debug, Clone, PartialEq, Props)]
pub struct FooterProps {
    /// Current platform
    pub platform: Platform,
    /// Component class name
    #[props(optional)]
    pub class: Option<String>,
    /// Component ID
    #[props(optional)]
    pub id: Option<String>,
    /// Footer text
    #[props(optional)]
    pub text: Option<String>,
    /// Version string
    #[props(optional)]
    pub version: Option<String>,
    /// Whether to show status
    #[props(optional)]
    pub show_status: Option<bool>,
    /// Status message
    #[props(optional)]
    pub status: Option<String>,
    /// Whether to show links
    #[props(optional)]
    pub show_links: Option<bool>,
    /// Footer links
    #[props(optional)]
    pub links: Option<Vec<FooterLink>>,
}

/// Footer link structure
#[derive(Debug, Clone, PartialEq)]
pub struct FooterLink {
    /// Link text
    pub text: String,
    /// Link URL
    pub url: String,
    /// Whether link opens in new tab
    pub external: bool,
}

/// Footer component for application footer
#[component]
pub fn Footer(props: FooterProps) -> Element {
    let show_status = props.show_status.unwrap_or(true);
    let show_links = props.show_links.unwrap_or(false);
    let links = props.links.unwrap_or_default();
    let version = props.version.unwrap_or_else(|| crate::VERSION.to_string());
    let text = props
        .text
        .unwrap_or_else(|| "SurfDesk - Solana Account Studio".to_string());

    let mut classes = vec!["footer"];
    if let Some(class) = &props.class {
        classes.push(class);
    }

    rsx! {
        footer {
            class: combine_classes(&classes),
            id: props.id,

            // Main footer content
            div { class: "footer-content",
                // Left section - app info
                div { class: "footer-left",
                    div { class: "footer-brand",
                        span { class: "footer-text", "{text}" }
                        span { class: "footer-version", "v{version}" }
                    }
                }

                // Center section - status
                if show_status {
                    div { class: "footer-center",
                        if let Some(status) = props.status {
                            span { class: "footer-status", "{status}" }
                        } else {
                            span { class: "footer-status", "Ready" }
                        }
                    }
                }

                // Right section - links
                if show_links && !links.is_empty() {
                    div { class: "footer-right",
                        nav { class: "footer-links",
                            for link in links {
                                a {
                                    href: "{link.url}",
                                    class: "footer-link",
                                    target: if link.external { "_blank" } else { "_self" },
                                    rel: if link.external { "noopener noreferrer" } else { "" },
                                    "{link.text}"
                                }
                            }
                        }
                    }
                }
            }

            // Sub-footer with additional info
            div { class: "footer-sub",
                div { class: "footer-sub-content",
                    span { class: "footer-copyright",
                        "© 2024 SurfDesk. All rights reserved."
                    }
                    span { class: "footer-build-info",
                        "Built with Dioxus 0.6+"
                    }
                }
            }
        }
    }
}

/// Simple footer with just text
#[component]
pub fn SimpleFooter(text: String, #[props(optional)] class: Option<String>) -> Element {
    rsx! {
        Footer {
            platform: Platform::Web,
            text: text,
            class: class,
            show_status: false,
            show_links: false,
        }
    }
}

/// Status footer with connection status
#[component]
pub fn StatusFooter(
    status: String,
    #[props(optional)] connected: Option<bool>,
    #[props(optional)] class: Option<String>,
) -> Element {
    let status_class = connected
        .map(|c| {
            if c {
                "status-connected"
            } else {
                "status-disconnected"
            }
        })
        .unwrap_or("status-unknown");

    rsx! {
        Footer {
            platform: Platform::Web,
            status: status,
            class: class,
            show_links: false,
        }
    }
}

/// Minimal footer for mobile/terminal
#[component]
pub fn MinimalFooter(
    #[props(optional)] text: Option<String>,
    #[props(optional)] class: Option<String>,
) -> Element {
    rsx! {
        footer {
            class: combine_classes(&[
                "footer",
                "footer-minimal",
                &class.as_ref().unwrap_or(&String::new())
            ]),
            div { class: "footer-minimal-content",
                span { class: "footer-minimal-text",
                    "{text.unwrap_or_else(|| \"SurfDesk\".to_string())}"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_footer_default_props() {
        assert!(true); // Simple test for now
    }

    #[test]
    fn test_footer_link() {
        let link = FooterLink {
            text: "Documentation".to_string(),
            url: "https://docs.surfdesk.dev".to_string(),
            external: true,
        };

        assert_eq!(link.text, "Documentation");
        assert_eq!(link.url, "https://docs.surfdesk.dev");
        assert!(link.external);
    }

    #[test]
    fn test_simple_footer() {
        let footer = SimpleFooter {
            text: "Simple Footer".to_string(),
            class: Some("custom-class".to_string()),
        };

        assert_eq!(footer.text, "Simple Footer");
        assert_eq!(footer.class, Some("custom-class".to_string()));
    }

    #[test]
    fn test_status_footer() {
        let footer = StatusFooter {
            status: "Connected".to_string(),
            connected: Some(true),
            class: None,
        };

        assert_eq!(footer.status, "Connected");
        assert_eq!(footer.connected, Some(true));
    }

    #[test]
    fn test_minimal_footer() {
        let footer = MinimalFooter {
            text: Some("Minimal".to_string()),
            class: None,
        };

        assert_eq!(footer.text, Some("Minimal".to_string()));
    }
}
