//! Main SurfDesk Application Component

use crate::accounts::AccountManager;
use crate::components::AppShell;
use crate::platform::Platform;
use crate::state::AppState;
use crate::transactions::TransactionManager;
use dioxus::prelude::*;

/// Main application component
#[component]
pub fn SurfDeskApp() -> Element {
    let state = use_signal(AppState::default);
    let account_manager = use_signal(AccountManager::new);
    let transaction_manager = use_signal(TransactionManager::new);

    rsx! {
        AppShell {
            state: state,
            platform: Platform::current(),
            theme: None,
        }
    }
}
