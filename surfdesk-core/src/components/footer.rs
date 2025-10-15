//! # Footer Component
//!
//! Footer component for displaying application information,
//  links, and status across all platforms.

use crate::components::{combine_classes, CommonProps};
use dioxus::prelude::*;

/// Footer component properties
#[derive(Debug, Clone, PartialEq, Props)]
pub struct FooterProps {
    /// Common component properties
    #[props(optional)]
    pub common: Option<CommonProps>,

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
    let common = props.common.unwrap_or_default();
    let show_status = props.show_status.unwrap_or(true);
    let show_links = props.show_links.unwrap_or(false);
    let links = props.links.unwrap_or_default();
    let version = props.version.unwrap_or_else(|| crate::VERSION.to_string());
    let text = props
        .text
        .unwrap_or_else(|| "SurfDesk - Solana Account Studio".to_string());

    let mut classes = vec!["footer"];
    if let Some(class) = &common.class {
        classes.push(class);
    }

    rsx! {
        footer {
            class: combine_classes(&classes),
            id: common.id,

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
                        "Built with Dioxus {dioxus::version::DIOXUS_VERSION}"
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
                &class.unwrap_or_default()
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
        let props = FooterProps {
            common: None,
            text: None,
            version: None,
            show_status: None,
            status: None,
            show_links: None,
            links: None,
        };

        assert!(props.show_status.unwrap_or(true));
        assert!(!props.show_links.unwrap_or(false));
        assert!(props.links.unwrap_or_default().is_empty());
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
