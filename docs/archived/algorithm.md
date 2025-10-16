# 🚀 ALGORITHM - FAST EXECUTION MODE

## 🔥 COMPILE CYCLE
```
START → CHECK ERRORS → CATEGORIZE → BULK FIX → VALIDATE → LOOP
```

## ⚡ ERROR PATTERNS
```rust
// ❌ ANTI-PATTERN
fn Comp(state: Signal<AppState>) -> Element

// ✅ CORRECT PATTERN  
#[derive(Props)]
struct Props { state: Signal<AppState> }
fn Comp(props: Props) -> Element
```

## 🎯 PRIORITY MATRIX
```
[DEPENDENCY] → [FEATURES] → [TRAITS] → [PROPS] → [MOVES] → [CLEANUP]
     5%        20%        15%      35%      20%       5%
```

## 📊 METRICS TRACKING
```
ERRORS_START | ERRORS_CURRENT | REDUCTION% | STATUS
     13      |       0        |   100%    | ✅ DONE
```

## 🔄 WORKFLOW LOOP
```
1. cargo check
2. grep error | wc -l  
3. BULK APPLY FIXES
4. VALIDATE REDUCTION
5. GOTO 1 UNTIL 0 ERRORS
```

## 🎯 PHASES
```
PHASE_1: DEPENDENCY RESOLVE
PHASE_2: FEATURE GATE FIXES  
PHASE_3: TRAIT IMPLEMENTATIONS
PHASE_4: COMPONENT PROPS
PHASE_5: MOVE FIXES
PHASE_6: CLEANUP
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