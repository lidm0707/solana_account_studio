//! Style constants for SurfDesk Web Application
//! Provides reusable styling strings for Dioxus components

/// Common layout styles
pub mod layout {
    pub const CONTAINER: &str = "min-height: 100vh; background-color: #f9fafb;";
    pub const CARD: &str = "background-color: white; border-radius: 0.5rem; padding: 1.5rem; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);";
    pub const SECTION: &str =
        "background-color: white; box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);";
    pub const FLEX_ROW: &str = "display: flex; align-items: center;";
    pub const FLEX_COL: &str = "display: flex; flex-direction: column;";
    pub const CENTER: &str = "display: flex; justify-content: center; align-items: center;";
    pub const SPACE_BETWEEN: &str = "justify-content: space-between;";
    pub const GAP: &str = "gap: 1rem;";
    pub const PADDING: &str = "padding: 1.5rem 2rem;";
    pub const PADDING_SM: &str = "padding: 0.75rem 2rem;";
    pub const MARGIN: &str = "margin: 1rem;";
}

/// Typography styles
pub mod typography {
    pub const HEADING_1: &str = "font-size: 1.875rem; font-weight: bold; color: #111827;";
    pub const HEADING_2: &str = "font-size: 1.5rem; font-weight: 600; color: #111827;";
    pub const HEADING_3: &str = "font-size: 1.25rem; font-weight: 600; color: #111827;";
    pub const BODY: &str = "font-size: 0.875rem; color: #374151;";
    pub const BODY_SM: &str = "font-size: 0.75rem; color: #6b7280;";
    pub const LABEL: &str = "font-size: 0.875rem; font-weight: 500; color: #374151;";
}

/// Button styles
pub mod buttons {
    pub const PRIMARY: &str = "background-color: #4f46e5; color: white; padding: 0.5rem 1rem; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 500; border: none; cursor: pointer; transition: background-color 0.2s;";
    pub const PRIMARY_HOVER: &str = "background-color: #4338ca;";
    pub const SECONDARY: &str = "background-color: #059669; color: white; padding: 0.5rem 1rem; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 500; border: none; cursor: pointer; transition: background-color 0.2s;";
    pub const SECONDARY_HOVER: &str = "background-color: #047857;";
    pub const TERTIARY: &str = "background-color: #4b5563; color: white; padding: 0.5rem 1rem; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 500; border: none; cursor: pointer; transition: background-color 0.2s;";
    pub const TERTIARY_HOVER: &str = "background-color: #374151;";
    pub const CANCEL: &str = "background-color: #d1d5db; color: #374151; padding: 0.5rem 1rem; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 500; border: none; cursor: pointer; transition: background-color 0.2s;";
    pub const CANCEL_HOVER: &str = "background-color: #9ca3af;";
    pub const LINK: &str =
        "color: #4f46e5; text-decoration: none; font-size: 0.875rem; font-weight: 500;";
    pub const LINK_HOVER: &str = "color: #4338ca; text-decoration: underline;";
}

/// Form styles
pub mod forms {
    pub const INPUT: &str = "width: 100%; padding: 0.5rem 0.75rem; font-size: 0.875rem; border: 1px solid #d1d5db; border-radius: 0.375rem; background-color: white; outline: none;";
    pub const INPUT_FOCUS: &str =
        "border-color: #4f46e5; box-shadow: 0 0 0 3px rgba(79, 70, 229, 0.1);";
    pub const SELECT: &str = "width: 8rem; padding: 0.5rem 2.5rem 0.5rem 0.75rem; font-size: 0.875rem; border: 1px solid #d1d5db; border-radius: 0.375rem; background-color: white; outline: none;";
    pub const TEXTAREA: &str = "width: 100%; padding: 0.5rem 0.75rem; font-size: 0.875rem; border: 1px solid #d1d5db; border-radius: 0.375rem; background-color: white; outline: none; resize: vertical;";
    pub const LABEL: &str = "display: block; font-size: 0.875rem; font-weight: 500; color: #374151; margin-bottom: 0.5rem;";
}

/// Message styles
pub mod messages {
    pub const ERROR: &str =
        "background-color: #fef2f2; border-left: 4px solid #ef4444; padding: 1rem; margin: 1rem;";
    pub const ERROR_TEXT: &str = "font-size: 0.875rem; color: #991b1b;";
    pub const SUCCESS: &str =
        "background-color: #f0fdf4; border-left: 4px solid #22c55e; padding: 1rem; margin: 1rem;";
    pub const SUCCESS_TEXT: &str = "font-size: 0.875rem; color: #166534;";
    pub const WARNING: &str =
        "background-color: #fffbeb; border-left: 4px solid #f59e0b; padding: 1rem; margin: 1rem;";
    pub const WARNING_TEXT: &str = "font-size: 0.875rem; color: #92400e;";
}

/// Modal styles
pub mod modal {
    pub const OVERLAY: &str = "position: fixed; inset: 0; background-color: rgba(0, 0, 0, 0.5); z-index: 50; display: flex; align-items: center; justify-content: center;";
    pub const CONTENT: &str = "background-color: white; border-radius: 0.5rem; padding: 1.5rem; box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1); max-width: 24rem; width: 90%; max-height: 90vh; overflow-y: auto;";
    pub const HEADER: &str =
        "font-size: 1.125rem; font-weight: bold; color: #111827; margin-bottom: 1rem;";
    pub const FOOTER: &str =
        "display: flex; justify-content: flex-end; gap: 0.75rem; margin-top: 1.5rem;";
}

/// Grid styles
pub mod grid {
    pub const AUTO_FIT: &str = "display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1.5rem; margin: 1.5rem 0;";
    pub const TWO_COL: &str = "display: grid; grid-template-columns: repeat(2, 1fr); gap: 1.5rem;";
    pub const THREE_COL: &str =
        "display: grid; grid-template-columns: repeat(3, 1fr); gap: 1.5rem;";
}

/// Loading styles
pub mod loading {
    pub const SPINNER: &str = "width: 3rem; height: 3rem; border: 4px solid #e5e7eb; border-top: 4px solid #4f46e5; border-radius: 50%; animation: spin 1s linear infinite;";
    pub const CONTAINER: &str =
        "display: flex; justify-content: center; align-items: center; padding: 3rem;";
}

/// Color palette
pub mod colors {
    pub const PRIMARY: &str = "#4f46e5";
    pub const PRIMARY_HOVER: &str = "#4338ca";
    pub const SECONDARY: &str = "#059669";
    pub const SECONDARY_HOVER: &str = "#047857";
    pub const TERTIARY: &str = "#4b5563";
    pub const TERTIARY_HOVER: &str = "#374151";
    pub const SUCCESS: &str = "#22c55e";
    pub const ERROR: &str = "#ef4444";
    pub const WARNING: &str = "#f59e0b";
    pub const GRAY_50: &str = "#f9fafb";
    pub const GRAY_100: &str = "#f3f4f6";
    pub const GRAY_200: &str = "#e5e7eb";
    pub const GRAY_300: &str = "#d1d5db";
    pub const GRAY_400: &str = "#9ca3af";
    pub const GRAY_500: &str = "#6b7280";
    pub const GRAY_600: &str = "#4b5563";
    pub const GRAY_700: &str = "#374151";
    pub const GRAY_800: &str = "#1f2937";
    pub const GRAY_900: &str = "#111827";
}

/// Component-specific styles
pub mod components {
    use super::*;

    pub const ACCOUNT_CARD: &str = "background-color: white; border-radius: 0.5rem; padding: 1.5rem; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); border: 1px solid #e5e7eb; transition: transform 0.2s, box-shadow 0.2s;";
    pub const ACCOUNT_CARD_HOVER: &str =
        "transform: translateY(-2px); box-shadow: 0 8px 12px rgba(0, 0, 0, 0.15);";
    pub const BALANCE_DISPLAY: &str = "font-size: 1.125rem; font-weight: bold; color: #4f46e5;";
    pub const PUBKEY_DISPLAY: &str =
        "font-family: monospace; font-size: 0.75rem; color: #6b7280; word-break: break-all;";
    pub const AVATAR: &str = "width: 2.5rem; height: 2.5rem; border-radius: 50%; background-color: #ede9fe; display: flex; align-items: center; justify-content: center; font-size: 1rem; color: #4f46e5; font-weight: 500;";

    pub const NAV_CONTAINER: &str =
        "background-color: white; border-bottom: 1px solid #e5e7eb; padding: 1rem 2rem;";
    pub const NAV_ITEM: &str = "color: #374151; text-decoration: none; padding: 0.5rem 1rem; border-radius: 0.375rem; transition: background-color 0.2s;";
    pub const NAV_ITEM_ACTIVE: &str = "background-color: #ede9fe; color: #4f46e5;";
    pub const NAV_ITEM_HOVER: &str = "background-color: #f3f4f6;";

    pub const TABLE_CONTAINER: &str = "background-color: white; border-radius: 0.5rem; overflow: hidden; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);";
    pub const TABLE_HEADER: &str = "background-color: #f9fafb; border-bottom: 1px solid #e5e7eb; padding: 1rem; font-weight: 600; color: #374151;";
    pub const TABLE_ROW: &str =
        "border-bottom: 1px solid #e5e7eb; padding: 1rem; transition: background-color 0.2s;";
    pub const TABLE_ROW_HOVER: &str = "background-color: #f9fafb;";
}

/// Animation keyframes
pub mod animations {
    pub const SPIN: &str = r#"
        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }
    "#;

    pub const FADE_IN: &str = r#"
        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(10px); }
            to { opacity: 1; transform: translateY(0); }
        }
    "#;

    pub const SLIDE_UP: &str = r#"
        @keyframes slideUp {
            from { opacity: 0; transform: translateY(20px); }
            to { opacity: 1; transform: translateY(0); }
        }
    "#;
}

/// Props for styled components
#[derive(Clone, Debug, PartialEq)]
pub struct StyleProps {
    pub variant: StyleVariant,
    pub size: Size,
    pub disabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StyleVariant {
    Primary,
    Secondary,
    Tertiary,
    Success,
    Error,
    Warning,
    Ghost,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl Default for StyleProps {
    fn default() -> Self {
        Self {
            variant: StyleVariant::Primary,
            size: Size::Medium,
            disabled: false,
        }
    }
}

/// Style builder helpers
impl StyleProps {
    pub fn primary() -> Self {
        Self {
            variant: StyleVariant::Primary,
            ..Default::default()
        }
    }

    pub fn secondary() -> Self {
        Self {
            variant: StyleVariant::Secondary,
            ..Default::default()
        }
    }

    pub fn tertiary() -> Self {
        Self {
            variant: StyleVariant::Tertiary,
            ..Default::default()
        }
    }

    pub fn success() -> Self {
        Self {
            variant: StyleVariant::Success,
            ..Default::default()
        }
    }

    pub fn error() -> Self {
        Self {
            variant: StyleVariant::Error,
            ..Default::default()
        }
    }

    pub fn warning() -> Self {
        Self {
            variant: StyleVariant::Warning,
            ..Default::default()
        }
    }

    pub fn small(mut self) -> Self {
        self.size = Size::Small;
        self
    }

    pub fn medium(mut self) -> Self {
        self.size = Size::Medium;
        self
    }

    pub fn large(mut self) -> Self {
        self.size = Size::Large;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Get button style based on props
pub fn get_button_style(props: &StyleProps) -> &'static str {
    use buttons::*;

    if props.disabled {
        return "background-color: #d1d5db; color: #9ca3af; padding: 0.5rem 1rem; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 500; border: none; cursor: not-allowed;";
    }

    match (&props.variant, &props.size) {
        (StyleVariant::Primary, Size::Small) => "background-color: #4f46e5; color: white; padding: 0.25rem 0.75rem; border-radius: 0.375rem; font-size: 0.75rem; font-weight: 500; border: none; cursor: pointer;",
        (StyleVariant::Primary, Size::Medium) => PRIMARY,
        (StyleVariant::Primary, Size::Large) => "background-color: #4f46e5; color: white; padding: 0.75rem 1.5rem; border-radius: 0.375rem; font-size: 1rem; font-weight: 500; border: none; cursor: pointer;",

        (StyleVariant::Secondary, Size::Small) => "background-color: #059669; color: white; padding: 0.25rem 0.75rem; border-radius: 0.375rem; font-size: 0.75rem; font-weight: 500; border: none; cursor: pointer;",
        (StyleVariant::Secondary, Size::Medium) => SECONDARY,
        (StyleVariant::Secondary, Size::Large) => "background-color: #059669; color: white; padding: 0.75rem 1.5rem; border-radius: 0.375rem; font-size: 1rem; font-weight: 500; border: none; cursor: pointer;",

        (StyleVariant::Tertiary, Size::Small) => "background-color: #4b5563; color: white; padding: 0.25rem 0.75rem; border-radius: 0.375rem; font-size: 0.75rem; font-weight: 500; border: none; cursor: pointer;",
        (StyleVariant::Tertiary, Size::Medium) => TERTIARY,
        (StyleVariant::Tertiary, Size::Large) => "background-color: #4b5563; color: white; padding: 0.75rem 1.5rem; border-radius: 0.375rem; font-size: 1rem; font-weight: 500; border: none; cursor: pointer;",

        (StyleVariant::Success, _) => "background-color: #22c55e; color: white; padding: 0.5rem 1rem; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 500; border: none; cursor: pointer;",
        (StyleVariant::Error, _) => "background-color: #ef4444; color: white; padding: 0.5rem 1rem; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 500; border: none; cursor: pointer;",
        (StyleVariant::Warning, _) => "background-color: #f59e0b; color: white; padding: 0.5rem 1rem; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 500; border: none; cursor: pointer;",
        (StyleVariant::Ghost, _) => "background-color: transparent; color: #4f46e5; padding: 0.5rem 1rem; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 500; border: 1px solid #d1d5db; cursor: pointer;",
    }
}
