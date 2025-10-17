# 🌊 DIOXUS.md - Dioxus 0.6+ Framework Guide for SurfDesk

## 📋 OVERVIEW

Dioxus is a modern, portable GUI framework for Rust that enables cross-platform application development. SurfDesk leverages Dioxus 0.6+ to create a unified experience across Desktop, Web, and Terminal platforms.

## 🏗️ ARCHITECTURE

### **Multi-Platform Strategy**
```
🖥️ Desktop: Native TAO-based applications
🌐 Web: WASM-compiled browser applications  
💻 Terminal: TUI-based console applications
📱 Mobile: Future support (planned)
```

### **Core Components**
```
🎨 UI Components: Reusable RSX-based components
🔄 State Management: Signals and hooks for reactive state
🌐 Routing: Client-side navigation with dioxus-router
🎭 Props System: Type-safe component communication
🔧 Services: Async service integration
📦 Workspace: Multi-package cargo workspace
```

## 🚀 GETTING STARTED

### **Installation**
```bash
# Install Rust with rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Dioxus CLI
cargo install dioxus-cli

# Verify installation
dx --version
```

### **Project Structure**
```
surfdesk/
├── surfdesk-core/          # Shared core library
├── surfdesk-desktop/       # Desktop application
├── surfdesk-web/          # Web application
├── surfdesk-tui/          # Terminal application
├── surfdesk-cli/          # Command-line interface
└── Cargo.toml             # Workspace configuration
```

### **Development Commands**
```bash
# Desktop development
dx serve --platform desktop

# Web development  
dx serve --platform web

# Terminal development
cargo run --package surfdesk-tui

# Build all platforms
dx build --platform desktop,web
```

## 🎨 COMPONENT DEVELOPMENT

### **Basic Component Structure**
```rust
use dioxus::prelude::*;

#[component]
pub fn MyComponent(
    #[props(default = "Default Title".to_string())] title: String,
    #[props(default = 42)] count: i32,
) -> Element {
    let mut state = use_signal(|| count);

    rsx! {
        div { class: "my-component",
            h1 { "{title}" }
            p { "Count: {state}" }
            button { 
                onclick: move |_| state += 1,
                "Increment"
            }
        }
    }
}
```

### **Props System**
```rust
#[derive(Clone, PartialEq, Props)]
pub struct ButtonProps {
    #[props(!optional)]
    pub onclick: EventHandler<MouseEvent>,
    #[props(default = "primary".to_string())]
    pub variant: String,
    #[props(default = false)]
    pub disabled: bool,
    pub children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: "btn-{props.variant}",
            onclick: props.onclick,
            disabled: props.disabled,
            {props.children}
        }
    }
}
```

### **State Management with Signals**
```rust
#[component]
pub fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| "Hello".to_string());

    rsx! {
        div {
            h1 { "Count: {count}" }
            input {
                value: "{text}",
                oninput: move |e| text.set(e.value()),
            }
            button {
                onclick: move |_| count += 1,
                "Increment"
            }
        }
    }
}
```

## 🔄 ASYNC PATTERNS

### **Coroutines for Async Operations**
```rust
use dioxus::prelude::*;

#[component]
pub fn AsyncComponent() -> Element {
    let mut data = use_signal(|| Vec::<String>::new());
    let mut loading = use_signal(|| false);

    // Use coroutine for async operations
    let fetch_data = use_coroutine(|_| {
        let mut data = data.clone();
        let mut loading = loading.clone();
        
        async move {
            loading.set(true);
            // Simulate API call
            tokio::time::sleep(Duration::from_secs(1)).await;
            
            let result = vec!["Item 1".to_string(), "Item 2".to_string()];
            data.set(result);
            loading.set(false);
        }
    });

    rsx! {
        div {
            if *loading.read() {
                "Loading..."
            } else {
                ul {
                    for item in data.read().iter() {
                        li { "{item}" }
                    }
                }
            }
            button {
                onclick: move |_| fetch_data.send(()),
                "Fetch Data"
            }
        }
    }
}
```

### **Service Integration**
```rust
use crate::services::database::DatabaseService;

#[component]
pub fn DatabaseComponent() -> Element {
    let mut projects = use_signal(|| Vec::<Project>::new());
    let mut error = use_signal(|| None::<String>);

    let load_projects = use_coroutine(|_| {
        let mut projects = projects.clone();
        let mut error = error.clone();
        
        async move {
            match DatabaseService::get_projects().await {
                Ok(data) => projects.set(data),
                Err(e) => error.set(Some(e.to_string())),
            }
        }
    });

    rsx! {
        div {
            if let Some(err) = error.read() {
                div { class: "error", "Error: {err}" }
            }
            
            button {
                onclick: move |_| load_projects.send(()),
                "Load Projects"
            }
            
            ul {
                for project in projects.read().iter() {
                    li { "{project.name}" }
                }
            }
        }
    }
}
```

## 🌐 ROUTING

### **Router Configuration**
```rust
use dioxus_router::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {
            div {
                h1 { "SurfDesk" }
                Nav {}
                Route::to404(Nav404 {}),
                Route {
                    to: Route::Home {},
                    Home {}
                }
                Route {
                    to: Route::Projects {},
                    Projects {}
                }
                Route {
                    to: Route::Settings {},
                    Settings {}
                }
            }
        }
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/projects")]
    Projects {},
    #[route("/settings")]
    Settings {},
    #[route("/:..route")]
    Nav404 { route: Vec<String> },
}
```

### **Navigation Component**
```rust
#[component]
pub fn Nav() -> Element {
    rsx! {
        nav {
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Projects {},
                "Projects"
            }
            Link {
                to: Route::Settings {},
                "Settings"
            }
        }
    }
}
```

## 🎨 STYLING

### **CSS Classes and Styles**
```rust
#[component]
pub fn StyledComponent() -> Element {
    rsx! {
        div { class: "container",
            style { "
                display: flex;
                flex-direction: column;
                gap: 1rem;
                padding: 2rem;
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                border-radius: 8px;
            " },
            
            h1 { 
                class: "title",
                style { "color: white; margin-bottom: 1rem;" }
            }
            
            button {
                class: "btn btn-primary",
                style { "
                    padding: 0.75rem 1.5rem;
                    background: #007bff;
                    color: white;
                    border: none;
                    border-radius: 4px;
                    cursor: pointer;
                    
                    &:hover {
                        background: #0056b3;
                        transform: translateY(-2px);
                        transition: all 0.2s ease;
                    }
                ",
                "Click me"
            }
        }
    }
}
```

### **Dynamic Styling**
```rust
#[component]
pub fn DynamicStyle() -> Element {
    let mut theme = use_signal(|| "light".to_string());
    let is_dark = *theme.read() == "dark";

    rsx! {
        div {
            style {
                "
                background: {if is_dark { '#1a1a1a' } else { '#ffffff' }};
                color: {if is_dark { '#ffffff' } else { '#000000' }};
                padding: 1rem;
                border-radius: 8px;
                transition: all 0.3s ease;
                "
            }
            
            h1 { "Theme: {theme}" }
            
            button {
                onclick: move |_| {
                    theme.set(if is_dark { "light" } else { "dark" }.to_string());
                },
                "Toggle Theme"
            }
        }
    }
}
```

## 🔧 PLATFORM-SPECIFIC FEATURES

### **Desktop Features**
```rust
#[cfg(feature = "desktop")]
use dioxus::desktop::{Config, WindowBuilder};

#[component]
pub fn DesktopApp() -> Element {
    // Desktop-specific code
    rsx! {
        div {
            h1 { "Desktop Application" }
            button {
                onclick: move |_| {
                    // Desktop-specific actions
                    if let Err(e) = dioxus::desktop::window::show_minimize() {
                        eprintln!("Failed to minimize: {e}");
                    }
                },
                "Minimize Window"
            }
        }
    }
}
```

### **Web Features**
```rust
#[cfg(feature = "web")]
use web_sys::window;

#[component]
pub fn WebApp() -> Element {
    rsx! {
        div {
            h1 { "Web Application" }
            button {
                onclick: move |_| {
                    // Web-specific actions
                    if let Some(win) = window() {
                        let _ = win.alert_with_message("Hello from Web!");
                    }
                },
                "Show Alert"
            }
        }
    }
}
```

## 📦 CUSTOM HOOKS

### **Use Local Storage Hook**
```rust
pub fn use_local_storage<T: serde::Serialize + serde::de::DeserializeOwned>(
    key: &str,
    initial_value: T,
) -> Signal<T> {
    let mut signal = use_signal(|| {
        // Try to load from localStorage
        if let Ok(Some(value)) = web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|storage| storage.get_item(key).ok().flatten())
        {
            serde_json::from_str(&value).unwrap_or(initial_value)
        } else {
            initial_value
        }
    });

    // Save to localStorage when value changes
    use_effect(move || {
        if let Ok(Some(storage)) = web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
        {
            if let Ok(json) = serde_json::to_string(&*signal.read()) {
                let _ = storage.set_item(key, &json);
            }
        }
    });

    signal
}
```

### **Use Debounce Hook**
```rust
pub fn use_debounce<T: Clone + 'static>(
    value: Signal<T>,
    delay_ms: u64,
) -> Signal<T> {
    let mut debounced = use_signal(|| value.read().clone());
    let mut timer = use_signal(|| None::<tokio::task::JoinHandle<()>>);

    use_effect(move || {
        // Cancel previous timer
        if let Some(handle) = timer.take() {
            handle.abort();
        }

        let current_value = value.read().clone();
        let debounced_clone = debounced.clone();

        // Set new timer
        let handle = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(delay_ms)).await;
            debounced_clone.set(current_value);
        });

        timer.set(Some(handle));
    });

    debounced
}
```

## 🧪 TESTING

### **Component Testing**
```rust
#[cfg(test)]
mod tests {
    use dioxus::prelude::*;
    use dioxus_ssr::render_lazy;

    #[test]
    fn test_button_render() {
        let mut vdom = VirtualDom::new(|| {
            rsx! {
                Button {
                    onclick: |_| {},
                    variant: "primary",
                    "Test Button"
                }
            }
        });

        let _ = vdom.rebuild();
        let html = render_lazy(&vdom);
        
        assert!(html.contains("Test Button"));
        assert!(html.contains("btn-primary"));
    }

    #[test]
    fn test_counter_interaction() {
        let mut vdom = VirtualDom::new(|| rsx! { Counter {} });
        let _ = vdom.rebuild();

        // Simulate button click
        let button = vdom
            .base_scope()
            .query_selector(&vdom, "button")
            .next()
            .unwrap();

        button.click(&mut vdom);
        
        // Check if count was incremented
        let count_text = vdom
            .base_scope()
            .query_selector(&vdom, "h1")
            .next()
            .unwrap()
            .inner_text(&vdom);

        assert!(count_text.contains("1"));
    }
}
```

## 🚀 PERFORMANCE OPTIMIZATION

### **Memoization**
```rust
#[component]
pub fn OptimizedList(items: Vec<String>) -> Element {
    // Memoize expensive computation
    let processed_items = use_memo(move || {
        items
            .iter()
            .map(|item| format!("Processed: {}", item))
            .collect::<Vec<_>>()
    });

    rsx! {
        ul {
            for item in processed_items.read().iter() {
                li { "{item}" }
            }
        }
    }
}
```

### **Lazy Loading**
```rust
#[component]
pub fn LazyComponent() -> Element {
    let mut should_load = use_signal(|| false);

    rsx! {
        div {
            button {
                onclick: move |_| should_load.set(true),
                "Load Component"
            }
            
            if *should_load.read() {
                rsx! {
                    HeavyComponent {}
                }
            }
        }
    }
}

#[component]
fn HeavyComponent() -> Element {
    // Expensive component that should only render when needed
    let data = use_resource(|| async {
        // Expensive computation or API call
        tokio::time::sleep(Duration::from_secs(2)).await;
        vec!["Heavy Data 1", "Heavy Data 2"]
    });

    rsx! {
        div {
            h2 { "Heavy Component" }
            match &*data.read() {
                Some(data) => rsx! {
                    for item in data {
                        p { "{item}" }
                    }
                },
                None => rsx! { "Loading..." },
            }
        }
    }
}
```

## 🔧 DEBUGGING

### **DevTools Integration**
```rust
#[cfg(debug_assertions)]
use dioxus::logger;

#[component]
pub fn DebugComponent() -> Element {
    // Enable logging in debug mode
    #[cfg(debug_assertions)]
    {
        logger::init(Default::default()).expect("failed to init logger");
    }

    let debug_info = use_signal(|| "Debug info will appear here".to_string());

    rsx! {
        div {
            h2 { "Debug Information" }
            pre {
                "{debug_info}"
            }
            
            button {
                onclick: move |_| {
                    let info = format!(
                        "Current time: {:?}\nPlatform: {:?}\nState: {:?}",
                        std::time::SystemTime::now(),
                        crate::current_platform(),
                        debug_info
                    );
                    debug_info.set(info);
                },
                "Update Debug Info"
            }
        }
    }
}
```

## 📚 BEST PRACTICES

### **Component Design**
1. **Single Responsibility**: Each component should have one clear purpose
2. **Props Interface**: Keep props simple and well-documented
3. **State Management**: Use signals for local state, services for global state
4. **Error Handling**: Always handle async errors gracefully
5. **Performance**: Use memoization for expensive computations

### **Code Organization**
```
src/
├── components/          # Reusable UI components
│   ├── buttons/        # Button variants
│   ├── forms/          # Form components
│   └── layout/         # Layout components
├── pages/              # Route-specific components
├── services/           # Business logic and API calls
├── styles/             # CSS and styling utilities
├── hooks/              # Custom hooks
├── utils/              # Utility functions
└── main.rs            # Application entry point
```

### **Naming Conventions**
- Components: PascalCase (e.g., `UserProfile`)
- Props: PascalCase with `Props` suffix (e.g., `UserProfileProps`)
- Signals: snake_case with descriptive names (e.g., `user_data`)
- Functions: snake_case (e.g., `fetch_user_data`)

## 🚀 DEPLOYMENT

### **Desktop Deployment**
```bash
# Build desktop application
dx build --platform desktop --release

# Create installer (platform-specific)
# Windows: Creates MSI installer
# macOS: Creates DMG package
# Linux: Creates AppImage or DEB package
```

### **Web Deployment**
```bash
# Build web application
dx build --platform web --release

# Deploy to any static hosting service
# The output will be in dist/
```

### **Terminal Deployment**
```bash
# Build terminal application
cargo build --release --package surfdesk-tui

# Binary will be in target/release/
```

## 🔗 RESOURCES

### **Official Documentation**
- [Dioxus Documentation](https://dioxuslabs.com/)
- [Dioxus Book](https://dioxuslabs.com/learn/0.6/)
- [Dioxus Examples](https://github.com/DioxusLabs/example-projects)

### **Community**
- [Discord Server](https://discord.gg/XgGxMSkvUM)
- [GitHub Discussions](https://github.com/DioxusLabs/dioxus/discussions)
- [Reddit](https://reddit.com/r/dioxus)

### **Related Tools**
- [Dioxus CLI](https://github.com/DioxusLabs/dioxus-cli)
- [Dioxus Router](https://github.com/DioxusLabs/dioxus-router)
- [Dioxus Signals](https://github.com/DioxusLabs/dioxus-signals)

---

**This guide covers the essential aspects of using Dioxus 0.6+ in SurfDesk. For more detailed information, refer to the official Dioxus documentation and the examples in this repository.**

🌊 **Happy coding with Dioxus!**