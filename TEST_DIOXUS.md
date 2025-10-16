# SurfDesk Testing Guide

## 📁 Test Directory Structure

SurfDesk uses a comprehensive test organization strategy with tests separated into dedicated directories:

```
tests/
├── integration/           # Cross-component integration tests
│   ├── app_shell_tests.rs
│   ├── navigation_tests.rs
│   ├── surfpool_integration.rs
│   └── cross_platform_tests.rs
├── unit/                  # Isolated unit tests
│   ├── components/       # Component-specific tests
│   │   ├── header_tests.rs
│   │   ├── sidebar_tests.rs
│   │   ├── footer_tests.rs
│   │   └── modal_tests.rs
│   ├── services/         # Service layer tests
│   │   ├── database_tests.rs
│   │   ├── events_tests.rs
│   │   ├── config_tests.rs
│   │   └── logger_tests.rs
│   └── database/         # Database-specific tests
│       ├── schema_tests.rs
│       ├── migration_tests.rs
│       └── query_tests.rs
└── common/               # Shared test utilities
    ├── mod.rs
    ├── test_helpers.rs
    └── mock_data.rs
```

## 🧪 Testing with Dioxus

When building applications or libraries with Dioxus, you may want to include tests to check the behavior of parts of your application. This guide will teach you how to test different parts of your Dioxus application.

### Component Testing
You can use a combination of pretty-assertions and dioxus-ssr to check that two snippets of rsx are equal:

tests/unit/components/component_test.rs

use futures::FutureExt;
use std::{cell::RefCell, sync::Arc};

use dioxus::prelude::*;

#[test]
fn test() {
    assert_rsx_eq(
        rsx! {
            div { "Hello world" }
            div { "Hello world" }
        },
        rsx! {
            for _ in 0..2 {
                div { "Hello world" }
            }
        },
    )
}

fn assert_rsx_eq(first: Element, second: Element) {
    let first = dioxus_ssr::render_element(first);
    let second = dioxus_ssr::render_element(second);
    pretty_assertions::assert_str_eq!(first, second);
}
### Hook Testing
When creating libraries around Dioxus, it can be helpful to make tests for your custom hooks.

Dioxus does not currently have a full hook testing library, but you can build a bespoke testing framework by manually driving the virtual dom.

tests/unit/components/hook_test.rs

use futures::FutureExt;
use std::{cell::RefCell, rc::Rc, sync::Arc, thread::Scope};

use dioxus::{dioxus_core::NoOpMutations, prelude::*};

#[test]
fn test() {
    test_hook(
        || use_signal(|| 0),
        |mut value, mut proxy| match proxy.generation {
            0 => {
                value.set(1);
            }
            1 => {
                assert_eq!(*value.read(), 1);
                value.set(2);
            }
            2 => {
                proxy.rerun();
            }
            3 => {}
            _ => todo!(),
        },
        |proxy| assert_eq!(proxy.generation, 4),
    );
}

fn test_hook<V: 'static>(
    initialize: impl FnMut() -> V + 'static,
    check: impl FnMut(V, MockProxy) + 'static,
    mut final_check: impl FnMut(MockProxy) + 'static,
) {
    #[derive(Props)]
    struct MockAppComponent<I: 'static, C: 'static> {
        hook: Rc<RefCell<I>>,
        check: Rc<RefCell<C>>,
    }

    impl<I, C> PartialEq for MockAppComponent<I, C> {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }

    impl<I, C> Clone for MockAppComponent<I, C> {
        fn clone(&self) -> Self {
            Self {
                hook: self.hook.clone(),
                check: self.check.clone(),
            }
        }
    }

    fn mock_app<I: FnMut() -> V, C: FnMut(V, MockProxy), V>(
        props: MockAppComponent<I, C>,
    ) -> Element {
        let value = props.hook.borrow_mut()();

        props.check.borrow_mut()(value, MockProxy::new());

        rsx! { div {} }
    }

    let mut vdom = VirtualDom::new_with_props(
        mock_app,
        MockAppComponent {
            hook: Rc::new(RefCell::new(initialize)),
            check: Rc::new(RefCell::new(check)),
        },
    );

    vdom.rebuild_in_place();

    while vdom.wait_for_work().now_or_never().is_some() {
        vdom.render_immediate(&mut NoOpMutations);
    }

    vdom.in_runtime(|| {
        ScopeId::ROOT.in_runtime(|| {
            final_check(MockProxy::new());
        })
    })
}

struct MockProxy {
    rerender: Arc<dyn Fn()>,
    pub generation: usize,
}

impl MockProxy {
    fn new() -> Self {
        let generation = generation();
        let rerender = schedule_update();

        Self {
            rerender,
            generation,
        }
    }

    pub fn rerun(&mut self) {
        (self.rerender)();
    }
}
### End to End Testing
You can use Playwright to create end to end tests for your dioxus application.

In your playwright.config.js, you will need to run cargo run or dx serve instead of the default build command. Here is a snippet from the end to end web example:

## 🚀 Running Tests

### Unit Tests
```bash
# Run all unit tests
cargo test --test unit

# Run specific component tests
cargo test --test unit -- components::header_tests

# Run service layer tests
cargo test --test unit -- services::database_tests
```

### Integration Tests
```bash
# Run all integration tests
cargo test --test integration

# Run specific integration tests
cargo test --test integration -- app_shell_tests

# Run cross-platform tests
cargo test --test integration -- cross_platform_tests
```

### Full Test Suite
```bash
# Run all tests
cargo test --workspace

# Run tests with coverage
cargo test --workspace --coverage

# Run tests in release mode
cargo test --workspace --release
```

## 📋 Test Categories

### 🧩 Unit Tests
- **Component Tests**: Individual component behavior and rendering
- **Service Tests**: Service layer functionality and error handling
- **Database Tests**: Schema validation and query operations

### 🔗 Integration Tests
- **App Shell Tests**: Layout and navigation integration
- **SurfPool Integration**: Validator management workflows
- **Cross-Platform Tests**: Multi-platform consistency validation

### 🎭 End-to-End Tests
- **User Workflows**: Complete user journey testing
- **Browser Tests**: Web application functionality
- **Desktop Tests**: Native application behavior

## 🛠️ Test Utilities

### Common Test Helpers
```rust
// tests/common/test_helpers.rs
pub fn setup_test_app() -> VirtualDom {
    // Setup test application state
}

pub fn create_test_props() -> ComponentProps {
    // Create test component props
}

pub fn assert_component_render(component: Element, expected: &str) {
    // Assert component renders correctly
}
```

### Mock Data
```rust
// tests/common/mock_data.rs
pub fn mock_solana_account() -> Account {
    // Create mock Solana account for testing
}

pub fn mock_surfpool_state() -> SurfPoolState {
    // Create mock SurfPool state for testing
}
```

## 📊 Coverage Requirements

SurfDesk maintains >90% test coverage across all components:
- Core components: 95% coverage required
- Service layer: 90% coverage required  
- Database operations: 95% coverage required
- Integration tests: 80% coverage required

## 🔄 Continuous Integration

All tests run automatically on:
- Pull requests
- Main branch commits
- Release candidates

Tests must pass before any code can be merged to main branch.


//...
webServer: [
    {
        cwd: path.join(process.cwd(), 'playwright-tests', 'web'),
        command: 'dx serve',
        port: 8080,
        timeout: 10 * 60 * 1000,
        reuseExistingServer: !process.env.CI,
        stdout: "pipe",
    },
],
