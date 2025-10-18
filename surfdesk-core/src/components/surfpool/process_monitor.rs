//! Process Status Monitor Component

use dioxus::prelude::*;

use crate::components::surfpool::surfpool_controller::ProcessStatus;

/// Process Status Monitor Component
#[component]
pub fn ProcessStatusMonitor() -> Element {
    let mut process_status = use_signal(|| Option::<ProcessStatus>::None);
    let mut auto_refresh = use_signal(|| true);
    let mut last_update = use_signal(String::new);

    rsx! {
        div { class: "process-status-monitor",
            div { class: "monitor-header",
                h3 { "📊 Process Status Monitor" }
                label { class: "auto-refresh-toggle",
                    input {
                        r#type: "checkbox",
                        checked: auto_refresh(),
                        onchange: move |evt| auto_refresh.set(evt.checked())
                    }
                    "Auto-refresh"
                }
                if !last_update().is_empty() {
                    span { class: "last-update", "Last: {last_update()}" }
                }
            }
            div { class: "status-details",
                div { class: "status-grid",
                    div { class: "status-item",
                        span { class: "label", "Running:" }
                        span { class: "status-yes", "Yes" }
                    }
                    div { class: "status-item",
                        span { class: "label", "RPC Port:" }
                        span { class: "value", "8999" }
                    }
                    div { class: "status-item",
                        span { class: "label", "WS Port:" }
                        span { class: "value", "9000" }
                    }
                    div { class: "status-item",
                        span { class: "label", "PID:" }
                        span { class: "value", "12345" }
                    }
                    div { class: "status-item",
                        span { class: "label", "Uptime:" }
                        span { class: "value", "120s" }
                    }
                }
            }
        }
    }
}
