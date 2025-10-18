# Production SurfDesk Test Suite

This directory contains the comprehensive production test suite for the SurfDesk application, organized by test type and functionality with zero-compilation-error architecture and enterprise-grade quality assurance.

## 📁 Directory Structure

```
tests/
├── README.md                    # This file
├── integration/                 # Production integration tests
│   ├── app_shell_tests.rs      # AppShell component integration
│   ├── navigation_tests.rs     # Navigation flow tests
│   ├── surfpool_integration.rs # SurfPool MCP service integration
│   └── cross_platform_tests.rs # Multi-platform consistency
├── unit/                       # Production unit tests
│   ├── components/            # Production component-specific tests
│   │   ├── header_tests.rs   # Header component tests
│   │   ├── sidebar_tests.rs  # Sidebar component tests
│   │   ├── footer_tests.rs   # Footer component tests
│   │   └── modal_tests.rs    # Modal component tests
│   ├── services/              # Production service layer tests
│   │   ├── database_tests.rs # Database service tests
│   │   ├── events_tests.rs   # Events service tests
│   │   ├── config_tests.rs   # Config service tests
│   │   ├── logger_tests.rs   # Logger service tests
│   │   └── surfpool_tests.rs # SurfPool MCP service tests
│   └── database/              # Production database-specific tests
│       ├── schema_tests.rs   # Database schema tests
│       ├── migration_tests.rs # Migration tests
│       └── query_tests.rs    # Query operation tests
└── common/                    # Production shared test utilities
    ├── mod.rs                # Common module exports
    ├── test_helpers.rs       # Test helper functions
    └── production_test_data.rs # Real production test data (no mocks)
```

## 🚀 Running Tests

### Unit Tests
```bash
# Run all unit tests
cargo test --test unit

# Run specific component tests
cargo test --test unit -- components::header_tests
cargo test --test unit -- components::sidebar_tests

# Run service layer tests
cargo test --test unit -- services::database_tests
cargo test --test unit -- services::events_tests
```

### Integration Tests
```bash
# Run all integration tests
cargo test --test integration

# Run specific integration tests
cargo test --test integration -- app_shell_tests
cargo test --test integration -- surfpool_integration

# Run cross-platform tests
cargo test --test integration -- cross_platform_tests
```

### Full Test Suite
```bash
# Run all tests (unit + integration + inline)
cargo test --workspace

# Run tests with detailed output
cargo test --workspace -- --nocapture

# Run tests in release mode
cargo test --workspace --release

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --workspace --out Html
```

## 📋 Test Categories

### 🧩 Unit Tests
- **Purpose**: Test individual components and functions in isolation
- **Location**: `tests/unit/`
- **Coverage Target**: 95% for core components, 90% for services
- **Examples**:
  - Component rendering and props handling
  - Service method functionality
  - Database schema validation

### 🔗 Integration Tests
- **Purpose**: Test component interactions and end-to-end workflows
- **Location**: `tests/integration/`
- **Coverage Target**: 80% for critical user flows
- **Examples**:
  - AppShell layout and navigation
  - SurfPool validator management
  - Cross-platform consistency

### 🎭 Inline Tests
- **Purpose**: Test module-specific functionality alongside implementation
- **Location**: Within source files under `#[cfg(test)]`
- **Coverage Target**: Supplement unit and integration tests
- **Examples**:
  - Internal helper functions
  - Data structure validation
  - Error handling scenarios

## 🛠️ Test Utilities

### Common Helpers (`tests/common/test_helpers.rs`)
- `create_test_vdom()`: Create a test VirtualDom instance
- `render_component()`: Render component to string for assertions
- `assert_components_eq()`: Compare two component renderings
- `PlatformTestHelper`: Platform-specific testing utilities
- `ComponentAssertions`: Component assertion helpers

### Production Test Data (`tests/common/production_test_data.rs`)
- `ProductionAccount`: Generate real test Solana accounts
- `ProductionTransaction`: Create real test transactions
- `ProductionEnvironment`: Generate test environments
- `ProductionSurfPool`: Create SurfPool MCP state fixtures
- `ProductionCollections`: Bulk real data generation

### Test Configuration (`tests/common/mod.rs`)
- Test constants and configuration
- Common setup functions
- Assertion utilities
- Error handling helpers

## 📊 Coverage Requirements

SurfDesk maintains strict production test coverage requirements:

| Component Type | Minimum Coverage | Target Coverage |
|----------------|------------------|------------------|
| Core Components | 90% | 95% |
| Service Layer | 85% | 90% |
| Database Operations | 90% | 95% |
| Integration Tests | 70% | 80% |
| Overall Coverage | 85% | 90% |

## 🔧 Test Configuration

### Production Environment Setup
Tests use the following environment variables:
- `SURFDESK_TEST_MODE`: Set to `true` during production test runs
- `SURFDESK_TEST_DB`: Path to test database (SQLite in-memory)
- `SURFDESK_SURFPOOL_URL`: SurfPool MCP URL for tests (localhost:8899)
- `SURFDESK_PRODUCTION_WALLET`: Real wallet path for production tests

### Production Test Dependencies
Key production testing dependencies:
- `pretty-assertions`: Enhanced assertion messages
- `dioxus-ssr`: Server-side rendering for component tests
- `env_logger`: Production logging during test execution
- `tokio-test`: Async testing utilities
- `surfpool-mcp-test`: SurfPool MCP testing utilities

## 🚨 Production Test Best Practices

### Writing Production Tests
1. **Descriptive Names**: Use clear, descriptive production test function names
2. **Arrange-Act-Assert**: Structure tests with clear setup, action, and verification phases
3. **Real Data**: Use provided production test data generators for consistency
4. **Isolation**: Ensure tests don't depend on each other's state
5. **Platform Testing**: Test components across all platforms with SurfPool MCP
6. **Zero Mock Policy**: Use real SurfPool MCP service instead of mocks

### Component Tests
```rust
#[test]
fn test_component_renders_correctly() {
    // Arrange
    let component = rsx! {
        Component {
            prop: "test_value".to_string(),
        }
    };
    
    // Act
    let rendered = render_component(component);
    
    // Assert
    assert_contains(&rendered, "test_value");
    assert_has_class(&rendered, "component-class");
}
```

### Production Service Tests
```rust
#[tokio::test]
async fn test_production_service_method() {
    // Arrange
    let service = Service::new(test_config()).await;
    
    // Act
    let result = service.do_something().await;
    
    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap().expected_field, "expected_value");
}
```

## 🔄 Continuous Integration

All production tests run automatically in CI/CD pipelines:
- **Pull Requests**: Full production test suite must pass
- **Main Branch**: Production tests run on every commit
- **Release**: Comprehensive production testing including performance benchmarks
- **Coverage**: Production coverage reports generated and must meet minimum requirements
- **SurfPool Integration**: All tests require running SurfPool MCP service

## 🐛 Debugging Production Tests

### Running Individual Production Tests
```bash
# Run a specific production test
cargo test test_name -- --exact

# Run production tests with output
cargo test test_name -- --nocapture

# Run production tests with backtrace
cargo test test_name -- --backtrace

# Run tests with SurfPool MCP
SURFDESK_SURFPOOL_URL=http://localhost:8899 cargo test test_name
```

### Common Production Issues
- **Signal Borrowing**: Use proper `mut` and clone patterns for Dioxus signals
- **Async Tests**: Use `#[tokio::test]` for async test functions
- **SurfPool MCP Connection**: Ensure SurfPool is running on localhost:8899
- **Database Tests**: Use in-memory SQLite for isolated database testing
- **Real Data**: Use production test data instead of mocks

## 📈 Production Performance Testing

Some production tests include performance validation:
- Component render time benchmarks with SurfPool MCP
- Service method performance limits
- Memory usage validation
- Database query performance
- SurfPool MCP response time validation

Performance tests use the `ProductionPerformanceTestHelper` utility for consistent measurement.

## 🔮 Production Future Enhancements

Planned production test improvements:
- **Visual Regression Testing**: Component screenshot comparison
- **Load Testing**: High-volume transaction and account testing with SurfPool
- **Fuzz Testing**: Randomized input testing for robustness
- **Property-Based Testing**: Automated test case generation
- **Browser Automation**: End-to-end testing with Playwright and SurfPool MCP
- **SurfPool MCP Stress Testing**: High-load MCP connection testing

---

For more information on production testing with Dioxus, see [TEST_DIOXUS.md](../TEST_DIOXUS.md).