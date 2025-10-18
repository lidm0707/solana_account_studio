# 🚀 ALGORITHM - PRODUCTION FAST EXECUTION MODE

## 🔥 PRODUCTION COMPILE CYCLE
```
START → CHECK ERRORS → CATEGORIZE → BULK FIX → VALIDATE → PRODUCTION DEPLOY
```

## ⚡ PRODUCTION ERROR PATTERNS
```rust
// ❌ ANTI-PATTERN (Production)
fn Comp(state: Signal<AppState>) -> Element

// ✅ PRODUCTION CORRECT PATTERN  
#[derive(Props)]
struct Props { state: Signal<AppState> }
fn Comp(props: Props) -> Element

// ✅ SURFPOOL TERMINAL STRATEGY
use tokio::process::Command;
let output = Command::new("surfpool").arg("start").output().await?;
```

## 🎯 PRODUCTION PRIORITY MATRIX
```
[SURFPOOL INTEGRATION] → [TERMINAL STRATEGY] → [REAL SERVICES] → [UI UPDATES] → [VALIDATION] → [PRODUCTION]
          25%                 20%                15%           25%         10%           5%
```

## 📊 PRODUCTION METRICS TRACKING
```
ERRORS_START | ERRORS_CURRENT | REDUCTION% | SURFPOOL_STATUS | PRODUCTION_STATUS
     13      |       0        |   100%    | TERMINAL_READY  | ✅ PRODUCTION_READY
```

## 🔄 PRODUCTION WORKFLOW LOOP
```
1. cargo check --workspace
2. grep error | wc -l  
3. BULK APPLY PRODUCTION FIXES
4. VALIDATE SURFPOOL INTEGRATION
5. CHECK TERMINAL STRATEGY
6. GOTO 1 UNTIL 0 ERRORS
7. PRODUCTION DEPLOY
```

## 🎯 PRODUCTION PHASES
```
PHASE_1: DEPENDENCY RESOLVE
PHASE_2: SURFPOOL TERMINAL INTEGRATION  
PHASE_3: REAL SERVICE IMPLEMENTATIONS
PHASE_4: PRODUCTION UI UPDATES
PHASE_5: VALIDATION & TESTING
PHASE_6: PRODUCTION DEPLOYMENT
```

## ⚡ QUICK COMMANDS
```bash
# Core check
cargo check --lib 2>&1 | grep error | wc -l

# Platform checks  
cargo check --features desktop
cargo check --features web
cargo check --features tui

# Bulk build
cargo build --release --all-targets
```

## 🎯 SUCCESS MATRIX
```
0 ERRORS ✅ | ALL_PLATFORMS ✅ | 100% REDUCTION ✅
```

---
*FAST EXECUTION MODE - NO WORDS, PURE LOGIC*