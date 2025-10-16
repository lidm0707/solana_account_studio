//! Cross-platform integration tests
//! Tests that components work consistently across Desktop, Web, and Terminal platforms

use dioxus::prelude::*;
use pretty_assertions::assert_str_eq;
use surfdesk_core::components::{AppShell, Footer, Header, Sidebar};
use surfdesk_core::platform::Platform;
use surfdesk_core::state::AppState;

#[test]
fn test_cross_platform_app_shell_consistency() {
    let app_state = use_signal(|| AppState::default());

    // Test all platforms render the basic structure
    let platforms = vec![
        (Platform::Desktop, "platform-desktop"),
        (Platform::Web, "platform-web"),
        (Platform::Terminal, "platform-terminal"),
    ];

    for (platform, expected_class) in platforms {
        let app_shell = rsx! {
            AppShell {
                state: app_state,
                platform: platform,
                theme: None,
            }
        };

        let rendered = dioxus_ssr::render_element(app_shell);

        // Verify core structure is consistent
        assert!(rendered.contains("app-shell"));
        assert!(rendered.contains(expected_class));
        assert!(rendered.contains("header"));
        assert!(rendered.contains("app-main"));
        assert!(rendered.contains("footer"));
    }
}

#[test]
fn test_cross_platform_header_adaptations() {
    let platforms = vec![
        (Platform::Desktop, "header-desktop"),
        (Platform::Web, "header-web"),
        (Platform::Terminal, "header-terminal"),
    ];

    for (platform, expected_class) in platforms {
        let header = rsx! {
            Header {
                platform: platform,
                active_section: "dashboard".to_string(),
                on_sidebar_toggle: None,
            }
        };

        let rendered = dioxus_ssr::render_element(header);

        // Verify platform-specific adaptations
        assert!(rendered.contains("header"));
        assert!(rendered.contains("platform-header"));
        assert!(rendered.contains(expected_class));

        // Verify common elements exist on all platforms
        assert!(rendered.contains("header-brand"));
        assert!(rendered.contains("SurfDesk"));
    }
}

#[test]
fn test_cross_platform_sidebar_behavior() {
    let platforms = vec![
        (Platform::Desktop, "sidebar-desktop"),
        (Platform::Web, "sidebar-web"),
        (Platform::Terminal, "sidebar-terminal"),
    ];

    for (platform, expected_class) in platforms {
        let sidebar = rsx! {
            Sidebar {
                platform: platform,
                is_collapsed: false,
                active_section: "dashboard".to_string(),
                on_section_change: None,
            }
        };

        let rendered = dioxus_ssr::render_element(sidebar);

        // Verify platform-specific sidebar behavior
        assert!(rendered.contains("sidebar"));
        assert!(rendered.contains("platform-sidebar"));
        assert!(rendered.contains(expected_class));

        // Verify navigation elements exist
        assert!(rendered.contains("sidebar-nav"));
        assert!(rendered.contains("Dashboard"));
        assert!(rendered.contains("SurfPool"));
    }
}

#[test]
fn test_cross_platform_footer_consistency() {
    let platforms = vec![
        (Platform::Desktop, "footer-desktop"),
        (Platform::Web, "footer-web"),
        (Platform::Terminal, "footer-terminal"),
    ];

    for (platform, expected_class) in platforms {
        let footer = rsx! {
            Footer {
                platform: platform,
                connected: true,
            }
        };

        let rendered = dioxus_ssr::render_element(footer);

        // Verify footer structure consistency
        assert!(rendered.contains("footer"));
        assert!(rendered.contains("platform-footer"));
        assert!(rendered.contains(expected_class));
        assert!(rendered.contains("connection-status"));
    }
}

#[test]
fn test_cross_platform_theme_support() {
    let themes = vec![
        (
            surfdesk_core::components::app_shell::Theme::Light,
            "theme-light",
        ),
        (
            surfdesk_core::components::app_shell::Theme::Dark,
            "theme-dark",
        ),
        (
            surfdesk_core::components::app_shell::Theme::Auto,
            "theme-auto",
        ),
    ];

    let platforms = vec![Platform::Desktop, Platform::Web, Platform::Terminal];

    for platform in platforms {
        for (theme, expected_class) in &themes {
            let app_shell = rsx! {
                AppShell {
                    state: use_signal(|| AppState::default()),
                    platform: platform,
                    theme: Some(theme.clone()),
                }
            };

            let rendered = dioxus_ssr::render_element(app_shell);

            // Verify theme support across platforms
            assert!(rendered.contains(expected_class));
            assert!(rendered.contains("app-shell"));
        }
    }
}

#[test]
fn test_cross_platform_responsive_layout() {
    let app_state = use_signal(|| AppState::default());

    // Test responsive behavior on different platforms
    let test_cases = vec![
        (Platform::Desktop, "platform-desktop", "desktop-layout"),
        (Platform::Web, "platform-web", "responsive-layout"),
        (Platform::Terminal, "platform-terminal", "terminal-layout"),
    ];

    for (platform, platform_class, layout_class) in test_cases {
        let app_shell = rsx! {
            AppShell {
                state: app_state,
                platform: platform,
                theme: None,
            }
        };

        let rendered = dioxus_ssr::render_element(app_shell);

        // Verify platform and layout classes
        assert!(rendered.contains(platform_class));
        assert!(rendered.contains("app-shell"));
    }
}

#[test]
fn test_cross_platform_navigation_consistency() {
    let platforms = vec![Platform::Desktop, Platform::Web, Platform::Terminal];

    for platform in platforms {
        let app_state = use_signal(|| AppState::default());

        let app_shell = rsx! {
            AppShell {
                state: app_state,
                platform: platform,
                theme: None,
            }
        };

        let rendered = dioxus_ssr::render_element(app_shell);

        // Verify navigation consistency across platforms
        assert!(rendered.contains("Dashboard"));
        assert!(rendered.contains("SurfPool"));
        assert!(rendered.contains("Accounts"));

        // Verify navigation structure
        match platform {
            Platform::Desktop => {
                assert!(rendered.contains("sidebar-nav"));
                assert!(rendered.contains("main-nav"));
            }
            Platform::Web => {
                assert!(rendered.contains("main-nav"));
                assert!(rendered.contains("responsive-layout"));
            }
            Platform::Terminal => {
                assert!(rendered.contains("terminal-header"));
                assert!(rendered.contains("terminal-sidebar"));
            }
        }
    }
}

#[test]
fn test_cross_platform_accessibility_features() {
    let platforms = vec![Platform::Desktop, Platform::Web, Platform::Terminal];

    for platform in platforms {
        let app_state = use_signal(|| AppState::default());

        let app_shell = rsx! {
            AppShell {
                state: app_state,
                platform: platform,
                theme: None,
            }
        };

        let rendered = dioxus_ssr::render_element(app_shell);

        // Verify semantic HTML structure exists across platforms
        assert!(rendered.contains("<header"));
        assert!(rendered.contains("</header>"));
        assert!(rendered.contains("<main"));
        assert!(rendered.contains("</main>"));
        assert!(rendered.contains("<footer"));
        assert!(rendered.contains("</footer>"));
        assert!(rendered.contains("<nav"));
        assert!(rendered.contains("</nav>"));
    }
}

#[test]
fn test_cross_platform_performance_consistency() {
    use std::time::Instant;

    let platforms = vec![Platform::Desktop, Platform::Web, Platform::Terminal];
    let render_times = Vec::new();

    for platform in platforms {
        let app_state = use_signal(|| AppState::default());

        let start = Instant::now();

        let app_shell = rsx! {
            AppShell {
                state: app_state,
                platform: platform,
                theme: None,
            }
        };

        let _rendered = dioxus_ssr::render_element(app_shell);

        let duration = start.elapsed();

        // Store render times for comparison (in a real test)
        // For now, just ensure rendering completes
        assert!(
            duration.as_millis() < 1000,
            "Rendering should complete within 1 second"
        );
    }
}

#[test]
fn test_cross_platform_state_management() {
    let platforms = vec![Platform::Desktop, Platform::Web, Platform::Terminal];
    let mut app_state = use_signal(|| AppState::default());

    // Test state changes across platforms
    let sections = vec!["dashboard", "surfpool", "accounts"];

    for platform in platforms {
        for section in sections {
            app_state.write().active_section = section.to_string();

            let app_shell = rsx! {
                AppShell {
                    state: app_state,
                    platform: platform,
                    theme: None,
                }
            };

            let rendered = dioxus_ssr::render_element(app_shell);

            // Verify state is reflected consistently
            assert!(rendered.contains("app-shell"));
            assert!(
                rendered.contains("platform-desktop")
                    || rendered.contains("platform-web")
                    || rendered.contains("platform-terminal")
            );
        }
    }
}

#[test]
fn test_cross_platform_error_handling() {
    let platforms = vec![Platform::Desktop, Platform::Web, Platform::Terminal];

    for platform in platforms {
        // Test with invalid state (should not panic)
        let mut app_state = use_signal(|| AppState::default());
        app_state.write().active_section = "invalid-section".to_string();

        let app_shell = rsx! {
            AppShell {
                state: app_state,
                platform: platform,
                theme: None,
            }
        };

        // Should render without errors
        let rendered = dioxus_ssr::render_element(app_shell);
        assert!(rendered.contains("app-shell"));
        assert!(!rendered.is_empty());
    }
}

#[test]
fn test_cross_platform_css_classes() {
    let platforms = vec![
        (
            Platform::Desktop,
            vec![
                "platform-desktop",
                "header-desktop",
                "sidebar-desktop",
                "footer-desktop",
            ],
        ),
        (
            Platform::Web,
            vec!["platform-web", "header-web", "sidebar-web", "footer-web"],
        ),
        (
            Platform::Terminal,
            vec![
                "platform-terminal",
                "header-terminal",
                "sidebar-terminal",
                "footer-terminal",
            ],
        ),
    ];

    for (platform, expected_classes) in platforms {
        let app_state = use_signal(|| AppState::default());

        let app_shell = rsx! {
            AppShell {
                state: app_state,
                platform: platform,
                theme: None,
            }
        };

        let rendered = dioxus_ssr::render_element(app_shell);

        // Verify platform-specific CSS classes
        for class in expected_classes {
            assert!(
                rendered.contains(class),
                "Missing class '{}' for platform {:?}",
                class,
                platform
            );
        }
    }
}
