//! Main SurfDesk Application Component

use crate::components::AppShell;
use crate::platform::Platform;
use crate::solana_rpc::accounts::AccountManager;
use crate::solana_rpc::transactions::TransactionManager;
use crate::state::AppState;
use dioxus::prelude::*;

/// Main application component
#[component]
pub fn SurfDeskApp() -> Element {
    let state = use_signal(AppState::default);
    let _account_manager = use_signal(AccountManager::new);
    let _transaction_manager = use_signal(TransactionManager::new);

    rsx! {
        AppShell {
            state: state,
            platform: Platform::current(),
            theme: None,
        }
    }
}
