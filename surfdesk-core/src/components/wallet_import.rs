//! Wallet Import Component
//!
//! Provides UI for importing wallets from various sources including:
//! - File path selection
//! - Base58 secret key input
//! - Directory batch import
//! - Drag and drop support (future)

use crate::error::{Result, SurfDeskError};
use crate::solana_rpc::{
    accounts::{Account, AccountManager},
    SolanaNetwork,
};
use crate::wallet::{WalletFormat, WalletImportService};
use dioxus::prelude::*;
use std::path::PathBuf;

/// Wallet import component props
#[derive(Debug, PartialEq, Clone, Props)]
pub struct WalletImportProps {
    /// Account manager to add imported accounts to
    pub account_manager: AccountManager,
    /// Network to import accounts to
    pub network: SolanaNetwork,
    /// Callback when import is successful
    pub on_import_success: EventHandler<Vec<Account>>,
    /// Callback when import fails
    pub on_import_error: EventHandler<String>,
    /// Custom CSS classes
    pub class: Option<String>,
}

/// Wallet import component
#[component]
pub fn WalletImport(props: WalletImportProps) -> Element {
    let mut import_method = use_signal(|| ImportMethod::File);
    let mut file_path = use_signal(|| String::new());
    let mut secret_key = use_signal(|| String::new());
    let mut label = use_signal(|| String::new());
    let mut is_importing = use_signal(|| false);
    let mut import_results = use_signal(|| Vec::<ImportResult>::new());

    let handle_file_import = move |_| {
        let file_path_str = file_path.read().clone();
        let label_str = label.read().clone();
        let network = props.network;
        let account_manager = props.account_manager.clone();
        let mut is_importing = is_importing.clone();
        let mut import_results = import_results.clone();
        let on_success = props.on_import_success.clone();
        let on_error = props.on_import_error.clone();

        spawn(async move {
            is_importing.set(true);
            import_results.set(Vec::<ImportResult>::new());

            let path = PathBuf::from(&file_path_str);

            match WalletImportService::import_from_path(
                &path,
                if label_str.is_empty() {
                    None
                } else {
                    Some(label_str)
                },
                network,
            )
            .await
            {
                Ok(account) => {
                    let result = ImportResult {
                        success: true,
                        account: Some(account.clone()),
                        error: None,
                    };
                    import_results.with_mut(|results| results.push(result));
                    on_success.call(vec![account]);
                }
                Err(e) => {
                    let result = ImportResult {
                        success: false,
                        account: None,
                        error: Some(e.to_string()),
                    };
                    import_results.with_mut(|results| results.push(result));
                    on_error.call(e.to_string());
                }
            }

            is_importing.set(false);
        });
    };

    let handle_secret_key_import = move |_| {
        let secret_key_str = secret_key.read().clone();
        let label_str = label.read().clone();
        let network = props.network;
        let mut is_importing = is_importing.clone();
        let mut import_results = import_results.clone();
        let on_success = props.on_import_success.clone();
        let on_error = props.on_import_error.clone();

        spawn(async move {
            is_importing.set(true);
            import_results.set(Vec::<ImportResult>::new());

            match WalletImportService::import_from_base58(
                &secret_key_str,
                if label_str.is_empty() {
                    None
                } else {
                    Some(label_str)
                },
                network,
            ) {
                Ok(account) => {
                    let result = ImportResult {
                        success: true,
                        account: Some(account.clone()),
                        error: None,
                    };
                    import_results.with_mut(|results| results.push(result));
                    on_success.call(vec![account]);
                }
                Err(e) => {
                    let result = ImportResult {
                        success: false,
                        account: None,
                        error: Some(e.to_string()),
                    };
                    import_results.with_mut(|results| results.push(result));
                    on_error.call(e.to_string());
                }
            }

            is_importing.set(false);
        });
    };

    let handle_directory_import = move |_| {
        let file_path_str = file_path.read().clone();
        let network = props.network;
        let mut is_importing = is_importing.clone();
        let mut import_results = import_results.clone();
        let on_success = props.on_import_success.clone();
        let on_error = props.on_import_error.clone();

        spawn(async move {
            is_importing.set(true);
            import_results.set(Vec::<ImportResult>::new());

            let path = PathBuf::from(&file_path_str);

            match WalletImportService::import_from_directory(&path, network).await {
                Ok(accounts) => {
                    for account in &accounts {
                        let result = ImportResult {
                            success: true,
                            account: Some(account.clone()),
                            error: None,
                        };
                        import_results.with_mut(|results| results.push(result));
                    }
                    on_success.call(accounts);
                }
                Err(e) => {
                    let result = ImportResult {
                        success: false,
                        account: None,
                        error: Some(e.to_string()),
                    };
                    import_results.with_mut(|results| results.push(result));
                    on_error.call(e.to_string());
                }
            }

            is_importing.set(false);
        });
    };

    rsx! {
        div { class: format!("wallet-import-container {}", props.class.as_deref().unwrap_or("")),

            // Header
            div { class: "wallet-import-header",
                h3 { class: "wallet-import-title", "Import Wallet" }
                p { class: "wallet-import-description",
                    "Import wallets from files, Base58 keys, or directories"
                }
            }

            // Import method selection
            div { class: "import-method-selector",
                div { class: "method-tabs",
                    button {
                        class: format!("method-tab {} {}",
                            if import_method() == ImportMethod::File { "active" } else { "" },
                            if is_importing() { "disabled" } else { "" }
                        ),
                        onclick: move |_| if !is_importing() { import_method.set(ImportMethod::File) },
                        disabled: is_importing(),
                        "File Path"
                    }
                    button {
                        class: format!("method-tab {} {}",
                            if import_method() == ImportMethod::SecretKey { "active" } else { "" },
                            if is_importing() { "disabled" } else { "" }
                        ),
                        onclick: move |_| if !is_importing() { import_method.set(ImportMethod::SecretKey) },
                        disabled: is_importing(),
                        "Secret Key"
                    }
                    button {
                        class: format!("method-tab {} {}",
                            if import_method() == ImportMethod::Directory { "active" } else { "" },
                            if is_importing() { "disabled" } else { "" }
                        ),
                        onclick: move |_| if !is_importing() { import_method.set(ImportMethod::Directory) },
                        disabled: is_importing(),
                        "Directory"
                    }
                }
            }

            // Import form based on selected method
            div { class: "import-form",
                match import_method() {
                    ImportMethod::File => rsx! {
                        div { class: "file-import-form",
                            div { class: "form-group",
                                label { class: "form-label", "Wallet File Path:" }
                                div { class: "input-group",
                                    input {
                                        class: "form-input",
                                        r#type: "text",
                                        placeholder: "Enter path to wallet file...",
                                        value: "{file_path}",
                                        oninput: move |e| file_path.set(e.value()),
                                        disabled: is_importing(),
                                    }
                                    button {
                                        class: "browse-button",
                                        r#type: "button",
                                        disabled: is_importing(),
                                        onclick: move |_| {
                                            // TODO: Implement file browser dialog
                                            // For now, just show a placeholder
                                        },
                                        "Browse"
                                    }
                                }
                                p { class: "form-help",
                                    "Supported formats: JSON keypair, Base58 (.txt, .json, .key, .sk)"
                                }
                            }

                            div { class: "form-group",
                                label { class: "form-label", "Label (optional):" }
                                input {
                                    class: "form-input",
                                    r#type: "text",
                                    placeholder: "Enter wallet label...",
                                    value: "{label}",
                                    oninput: move |e| label.set(e.value()),
                                    disabled: is_importing(),
                                }
                            }

                            button {
                                class: format!("import-button {} {}",
                                    if file_path().is_empty() || is_importing() { "disabled" } else { "" },
                                    if is_importing() { "loading" } else { "" }
                                ),
                                r#type: "button",
                                onclick: handle_file_import,
                                disabled: file_path().is_empty() || is_importing(),
                                if is_importing() { "Importing..." } else { "Import Wallet" }
                            }
                        }
                    },
                    ImportMethod::SecretKey => rsx! {
                        div { class: "secret-key-import-form",
                            div { class: "form-group",
                                label { class: "form-label", "Base58 Secret Key:" }
                                textarea {
                                    class: "form-textarea",
                                    placeholder: "Enter Base58 encoded secret key...",
                                    value: "{secret_key}",
                                    oninput: move |e| secret_key.set(e.value()),
                                    disabled: is_importing(),
                                    rows: 3,
                                }
                                p { class: "form-help",
                                    "Enter a 44-88 character Base58 encoded secret key"
                                }
                            }

                            div { class: "form-group",
                                label { class: "form-label", "Label (optional):" }
                                input {
                                    class: "form-input",
                                    r#type: "text",
                                    placeholder: "Enter wallet label...",
                                    value: "{label}",
                                    oninput: move |e| label.set(e.value()),
                                    disabled: is_importing(),
                                }
                            }

                            button {
                                class: format!("import-button {} {}",
                                    if secret_key().is_empty() || is_importing() { "disabled" } else { "" },
                                    if is_importing() { "loading" } else { "" }
                                ),
                                r#type: "button",
                                onclick: handle_secret_key_import,
                                disabled: secret_key().is_empty() || is_importing(),
                                if is_importing() { "Importing..." } else { "Import Wallet" }
                            }
                        }
                    },
                    ImportMethod::Directory => rsx! {
                        div { class: "directory-import-form",
                            div { class: "form-group",
                                label { class: "form-label", "Directory Path:" }
                                div { class: "input-group",
                                    input {
                                        class: "form-input",
                                        r#type: "text",
                                        placeholder: "Enter path to directory containing wallet files...",
                                        value: "{file_path}",
                                        oninput: move |e| file_path.set(e.value()),
                                        disabled: is_importing(),
                                    }
                                    button {
                                        class: "browse-button",
                                        r#type: "button",
                                        disabled: is_importing(),
                                        onclick: move |_| {
                                            // TODO: Implement directory browser dialog
                                        },
                                        "Browse"
                                    }
                                }
                                p { class: "form-help",
                                    "All supported wallet files in the directory will be imported"
                                }
                            }

                            button {
                                class: format!("import-button {} {}",
                                    if file_path().is_empty() || is_importing() { "disabled" } else { "" },
                                    if is_importing() { "loading" } else { "" }
                                ),
                                r#type: "button",
                                onclick: handle_directory_import,
                                disabled: file_path().is_empty() || is_importing(),
                                if is_importing() { "Importing..." } else { "Import All Wallets" }
                            }
                        }
                    },
                }
            }

            // Import results
            if !import_results().is_empty() {
                div { class: "import-results",
                    h4 { class: "results-title", "Import Results" }
                    div { class: "results-list",
                        {import_results().iter().enumerate().map(|(index, result)| {
                            rsx! {
                                div { class: format!("result-item {} {}",
                                    if result.success { "success" } else { "error" },
                                    "result-item"
                                ),
                                    div { class: "result-icon",
                                        if result.success { "✓" } else { "✗" }
                                    }
                                    div { class: "result-content",
                                        if let Some(account) = &result.account {
                                            div { class: "result-title",
                                                "{account.display_name()}"
                                            }
                                            div { class: "result-details",
                                                "Public key: {account.pubkey}"
                                            }
                                        } else {
                                            div { class: "result-title", "Import Failed" }
                                        }
                                        if let Some(error) = &result.error {
                                            div { class: "result-error",
                                                "Error: {error}"
                                            }
                                        }
                                    }
                                }
                            }
                        })}
                    }
                }
            }

            // Loading overlay
            if is_importing() {
                div { class: "import-loading-overlay",
                    div { class: "loading-spinner" }
                    p { class: "loading-text", "Importing wallet..." }
                }
            }
        }
    }
}

/// Import method types
#[derive(Debug, Clone, PartialEq)]
enum ImportMethod {
    File,
    SecretKey,
    Directory,
}

/// Import result
#[derive(Debug, Clone)]
struct ImportResult {
    success: bool,
    account: Option<Account>,
    error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_method_equality() {
        assert_eq!(ImportMethod::File, ImportMethod::File);
        assert_ne!(ImportMethod::File, ImportMethod::SecretKey);
    }

    #[test]
    fn test_import_result_creation() {
        let success_result = ImportResult {
            success: true,
            account: None,
            error: None,
        };
        assert!(success_result.success);

        let error_result = ImportResult {
            success: false,
            account: None,
            error: Some("Test error".to_string()),
        };
        assert!(!error_result.success);
        assert_eq!(error_result.error, Some("Test error".to_string()));
    }
}
