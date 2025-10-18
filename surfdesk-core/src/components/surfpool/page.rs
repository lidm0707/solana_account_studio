// //! # SurfPool Page Component
// //!
// //! Enhanced SurfPool integration with real-time controls, status monitoring,
// //! live metrics, log viewing, and configuration management for local Solana validator.

// use crate::components::surfpool::ui_manager::{SurfPoolManager, SurfPoolMetrics, SurfPoolStatus};
// use crate::components::{Button, Card, Size, Variant};
// use crate::services::surfpool::SurfPoolConfig;
// use crate::solana_rpc::pubkey_key::{RpcCommitment, SolanaRpcClient};
// use dioxus::prelude::*;
// use log::{error, info, warn};
// use std::time::Duration;

// // Re-export spawn from tokio for async tasks
// use dioxus::prelude::spawn;

// /// Enhanced SurfPool control page component
// #[component]
// pub fn SurfPoolPage() -> Element {
//     let mut config = use_signal(|| SurfPoolConfig::default());
//     let mut show_config_modal = use_signal(|| false);
//     let mut error_message = use_signal(|| Option::<String>::None);
//     let mut current_status = use_signal(|| Option::<SurfPoolStatus>::None);
//     let mut current_metrics = use_signal(|| Option::<SurfPoolMetrics>::None);
//     let mut is_refreshing = use_signal(|| false);

//     // Enhanced SurfPool manager with real service integration
//     let surfpool_manager = use_signal(|| {
//         let cfg = config.read().clone();
//         SurfPoolManager::new(cfg)
//     });

//     // Real RPC client for health checks and validation
//     let rpc_client = use_signal(|| {
//         SolanaRpcClient::new_with_url(
//             "http://localhost:8899", // Default SurfPool port from testing
//             RpcCommitment::Confirmed,
//         )
//     });

//     // Enhanced periodic status and metrics update
//     use_coroutine(move |_: dioxus::prelude::UnboundedReceiver<()>| {
//         let mut error_signal = error_message;
//         let mut status_signal = current_status;
//         let mut metrics_signal = current_metrics;
//         let mut refreshing_signal = is_refreshing;
//         let manager = surfpool_manager.clone();
//         let rpc = rpc_client.clone();

//         async move {
//             loop {
//                 // Set refreshing state
//                 refreshing_signal.set(true);

//                 // Get SurfPool status
//                 if let Ok(manager_ref) = manager.try_read() {
//                     let status = manager_ref.get_status().await;
//                     status_signal.set(Some(status));
//                     error_signal.set(None);

//                     // Refresh metrics
//                     let metrics = manager_ref.get_metrics().await;
//                     metrics_signal.set(metrics);
//                 }

//                 // Periodic RPC health check
//                 if let Ok(healthy) = rpc.read().test_connection().await {
//                     if healthy {
//                         info!("RPC health check passed");
//                     } else {
//                         warn!("RPC health check failed");
//                         error_signal.set(Some("Unable to connect to RPC".to_string()));
//                     }
//                 }

//                 refreshing_signal.set(false);
//                 tokio::time::sleep(Duration::from_secs(3)).await;
//             }
//         }
//     });

//     // Handle configuration change
//     let handle_config_change = move |_| {
//         show_config_modal.set(true);
//     };

//     // Handle manual refresh
//     let handle_refresh = move |_| {
//         let manager = surfpool_manager.clone();
//         let mut status_signal = current_status;
//         let mut metrics_signal = current_metrics;
//         let mut error_signal = error_message;

//         spawn(async move {
//             is_refreshing.set(true);

//             if let Ok(manager_ref) = manager.try_read() {
//                 let status = manager_ref.get_status().await;
//                 status_signal.set(Some(status));
//                 error_signal.set(None);

//                 let metrics = manager_ref.get_metrics().await;
//                 metrics_signal.set(metrics);
//             }

//             is_refreshing.set(false);
//         });
//     };

//     // Handle start SurfPool
//     let handle_start_surfpool = move |_| {
//         let manager = surfpool_manager.clone();
//         let mut error_signal = error_message;

//         spawn(async move {
//             if let Ok(mut manager_ref) = manager.try_write() {
//                 if let Err(e) = manager_ref.start().await {
//                     error_signal.set(Some(format!("Failed to start SurfPool: {}", e)));
//                 } else {
//                     info!("SurfPool started successfully");
//                 }
//             }
//         });
//     };

//     // Handle stop SurfPool
//     let handle_stop_surfpool = move |_| {
//         let manager = surfpool_manager.clone();
//         let mut error_signal = error_message;

//         spawn(async move {
//             if let Ok(mut manager_ref) = manager.try_write() {
//                 if let Err(e) = manager_ref.stop().await {
//                     error_signal.set(Some(format!("Failed to stop SurfPool: {}", e)));
//                 } else {
//                     info!("SurfPool stopped successfully");
//                 }
//             }
//         });
//     };

//     rsx! {
//         div { class: "surfpool-page",
//             // Header with title and actions
//             div { class: "page-header",
//                 div { class: "page-title-section",
//                     h1 { class: "page-title", "🌊 SurfPool Control" }
//                     p { class: "page-subtitle",
//                         "Manage your local Solana validator with real-time monitoring and control" }
//                 }
//                 div { class: "page-actions",
//                     Button {
//                         variant: Variant::Secondary,
//                         size: Size::Medium,
//                         onclick: handle_config_change,
//                         children: rsx! {
//                             "⚙️ Configuration"
//                         }
//                     }
//                     Button {
//                         variant: Variant::Secondary,
//                         size: Size::Medium,
//                         onclick: move |_| info!("Export logs clicked"),
//                         children: rsx! {
//                             "📥 Export Logs"
//                         }
//                     }
//                 }
//             }

//             // Error display
//             if let Some(error) = error_message.read().as_ref() {
//                 Card {
//                     variant: Variant::Error,
//                     elevated: false,
//                     children: rsx! {
//                         div { class: "flex items-center gap-3",
//                             span { class: "text-xl", "⚠️" }
//                             div { class: "flex-1",
//                                 h3 { class: "font-semibold text-error", "Connection Error" }
//                                 p { class: "text-sm text-error/80", "{error}" }
//                             }
//                             Button {
//                                 variant: Variant::Ghost,
//                                 size: Size::Small,
//                                 onclick: move |_| error_message.set(None),
//                                 children: rsx! {
//                                     "✕"
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }

//             // Status Display
//             Card {
//                 elevated: true,
//                 children: rsx! {
//                     div { class: "flex justify-between items-center",
//                         h2 { class: "text-xl font-semibold mb-4", "🔧 Validator Status" }
//                         Button {
//                             variant: Variant::Ghost,
//                             size: Size::Small,
//                             onclick: handle_refresh,
//                             disabled: is_refreshing(),
//                             children: rsx! {
//                                 "🔄 Refresh"
//                             }
//                         }
//                     }

//                     if let Some(status) = current_status.read().as_ref() {
//                         div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
//                             // Service Status
//                             div { class: "status-card",
//                                 div { class: "status-indicator" }
//                                 div { class: "status-details",
//                                     h3 { "Service Status" }
//                                     p { "{:?}", status.service_status }
//                                 }
//                             }

//                             // Current Slot
//                             if let Some(slot) = status.current_slot {
//                                 div { class: "metric-card",
//                                     h3 { "Current Slot" }
//                                     p { "{slot}" }
//                                 }
//                             }

//                             // Block Height
//                             if let Some(block_height) = status.block_height {
//                                 div { class: "metric-card",
//                                     h3 { "Block Height" }
//                                     p { "{block_height}" }
//                                 }
//                             }
//                         }
//                     } else {
//                         div { class: "text-center py-8",
//                             div { class: "loading-spinner mx-auto mb-4" }
//                             p { "Loading SurfPool status..." }
//                         }
//                     }

//                     // Control Buttons
//                     div { class: "flex gap-3 mt-6",
//                         Button {
//                             variant: Variant::Primary,
//                             size: Size::Medium,
//                             onclick: handle_start_surfpool,
//                             children: rsx! {
//                                 "🚀 Start Validator"
//                             }
//                         }
//                         Button {
//                             variant: Variant::Secondary,
//                             size: Size::Medium,
//                             onclick: handle_stop_surfpool,
//                             children: rsx! {
//                                 "⏹️ Stop Validator"
//                             }
//                         }
//                     }
//                 }
//             }

//             // Metrics Display
//             if let Some(metrics) = current_metrics.read().as_ref() {
//                 Card {
//                     elevated: true,
//                     children: rsx! {
//                         h2 { class: "text-xl font-semibold mb-4", "📊 Performance Metrics" }

//                         div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
//                             div { class: "metric-item",
//                                 span { class: "metric-label", "TPS" }
//                                 span { class: "metric-value", "{metrics.tps}" }
//                             }
//                             div { class: "metric-item",
//                                 span { class: "metric-label", "Validators" }
//                                 span { class: "metric-value", "{metrics.validators}" }
//                             }
//                             div { class: "metric-item",
//                                 span { class: "metric-label", "Memory" }
//                                 span { class: "metric-value", "{metrics.memory_mb}MB" }
//                             }
//                             div { class: "metric-item",
//                                 span { class: "metric-label", "CPU" }
//                                 span { class: "metric-value", "{metrics.cpu_percent}%" }
//                             }
//                         }
//                     }
//                 }
//             }

//             // RPC Connection Test
//             Card {
//                 elevated: true,
//                 children: rsx! {
//                     h2 { class: "text-xl font-semibold mb-4", "🔌 RPC Connection" }

//                     div { class: "flex gap-3",
//                         Button {
//                             variant: Variant::Outline,
//                             size: Size::Medium,
//                             onclick: move |_| {
//                                 let rpc = rpc_client.clone();
//                                 let mut error_signal = error_message;

//                                 spawn(async move {
//                                     match rpc.read().test_connection().await {
//                                         Ok(healthy) => {
//                                             if healthy {
//                                                 info!("RPC connection test successful");
//                                             } else {
//                                                 warn!("RPC connection test failed - unhealthy");
//                                                 error_signal.set(Some("RPC connection unhealthy".to_string()));
//                                             }
//                                         }
//                                         Err(e) => {
//                                             error!("RPC connection test failed: {}", e);
//                                             error_signal.set(Some(format!("RPC connection test failed: {}", e)));
//                                         }
//                                     }
//                                 });
//                             },
//                             children: rsx! {
//                                 "🔍 Test Connection"
//                             }
//                         }
//                     }
//                 }
//             }
//         }

//         // Configuration Modal
//         if show_config_modal() {
//             ConfigModal {
//                 config: config.clone(),
//                 on_save: move |new_config| {
//                     config.set(new_config.clone());
//                     show_config_modal.set(false);

//                     // Update SurfPool manager with new config
//                     let manager = surfpool_manager.clone();
//                     let mut error_signal = error_message;

//                     spawn(async move {
//                         if let Ok(mut manager_ref) = manager.try_write() {
//                             if let Err(e) = manager_ref.update_config(new_config).await {
//                                 error_signal.set(Some(format!("Failed to update config: {}", e)));
//                             } else {
//                                 info!("SurfPool configuration updated successfully");
//                             }
//                         }
//                     });
//                 },
//                 on_cancel: move |_| {
//                     show_config_modal.set(false);
//                 }
//             }
//         }
//     }
// }

// /// Configuration modal component
// #[component]
// fn ConfigModal(
//     config: Signal<SurfPoolConfig>,
//     on_save: EventHandler<SurfPoolConfig>,
//     on_cancel: EventHandler<()>,
// ) -> Element {
//     let mut rpc_port = use_signal(|| config.read().rpc_port);
//     let mut ws_port = use_signal(|| config.read().ws_port);
//     let mut fork_url = use_signal(|| config.read().fork_url.clone());
//     let mut enable_mcp = use_signal(|| config.read().enable_mcp);

//     let handle_save = move |_| {
//         let new_config = SurfPoolConfig {
//             rpc_port: rpc_port(),
//             ws_port: ws_port(),
//             fork_url: fork_url.read().clone(),
//             enable_mcp: enable_mcp(),
//         };
//         on_save.call(new_config);
//     };

//     rsx! {
//         div { class: "modal-overlay",
//             div { class: "modal-content",
//                 div { class: "modal-header",
//                     h2 { "⚙️ SurfPool Configuration" }
//                     button {
//                         class: "modal-close",
//                         onclick: move |_| on_cancel.call(()),
//                         "✕"
//                     }
//                 }

//                 div { class: "modal-body",
//                     div { class: "form-group",
//                         label { "RPC Port" }
//                         input {
//                             value: "{rpc_port}",
//                             r#type: "number",
//                             oninput: move |e| {
//                                 if let Ok(port) = e.value().parse() {
//                                     rpc_port.set(port);
//                                 }
//                             }
//                         }
//                     }

//                     div { class: "form-group",
//                         label { "WebSocket Port" }
//                         input {
//                             value: "{ws_port}",
//                             r#type: "number",
//                             oninput: move |e| {
//                                 if let Ok(port) = e.value().parse() {
//                                     ws_port.set(port);
//                                 }
//                             }
//                         }
//                     }

//                     div { class: "form-group",
//                         label { "Fork URL (optional)" }
//                         input {
//                             value: "{fork_url.read().as_deref().unwrap_or("")}",
//                             oninput: move |e| {
//                                 let url = e.value();
//                                 if url.trim().is_empty() {
//                                     fork_url.set(None);
//                                 } else {
//                                     fork_url.set(Some(url));
//                                 }
//                             }
//                         }
//                     }

//                     div { class: "form-group",
//                         label { class: "checkbox-label",
//                             input {
//                                 r#type: "checkbox",
//                                 checked: enable_mcp(),
//                                 onchange: move |e| enable_mcp.set(e.checked())
//                             }
//                             "Enable MCP (Model Context Protocol)"
//                         }
//                     }
//                 }

//                 div { class: "modal-footer",
//                     Button {
//                         variant: Variant::Secondary,
//                         size: Size::Medium,
//                         onclick: move |_| on_cancel.call(()),
//                         children: rsx! { "Cancel" }
//                     }
//                     Button {
//                         variant: Variant::Primary,
//                         size: Size::Medium,
//                         onclick: handle_save,
//                         children: rsx! { "💾 Save Configuration" }
//                     }
//                 }
//             }
//         }
//     }
// }
