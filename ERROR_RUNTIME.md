➜  solana_account_studio git:(main) ✗ ./scripts/run-web.sh
=================================
🏄‍♂️ SurfDesk Web Application
=================================
[INFO] Checking dependencies...
[WARN] wasm-pack not found. Installing...
info: downloading wasm-pack
info: successfully installed wasm-pack to `/home/moo-tu/.cargo/bin/wasm-pack`
[INFO] Using web server: python3 -m http.server
=================================
🏗️ Building SurfDesk Web Application
=================================
[INFO] Building web application...
Error: crate-type must be cdylib to compile to wasm32-unknown-unknown. Add the following to your Cargo.toml file:

[lib]
crate-type = ["cdylib", "rlib"]
Caused by: crate-type must be cdylib to compile to wasm32-unknown-unknown. Add the following to your Cargo.toml file:

[lib]
crate-type = ["cdylib", "rlib"]
[WARN] wasm-pack build failed, trying cargo build...
   Compiling cfg-if v1.0.4
   Compiling once_cell v1.21.3
   Compiling unicode-ident v1.0.19
   Compiling memchr v2.7.6
   Compiling serde_core v1.0.228
   Compiling serde v1.0.228
   Compiling wasm-bindgen-shared v0.2.104
   Compiling typenum v1.19.0
   Compiling log v0.4.28
   Compiling generic-array v0.14.9
   Compiling smallvec v1.15.1
   Compiling wasm-bindgen v0.2.104
   Compiling num-traits v0.2.19
   Compiling zerocopy v0.8.27
   Compiling subtle v2.4.1
   Compiling parking_lot_core v0.9.12
   Compiling scopeguard v1.2.0
   Compiling itoa v1.0.15
   Compiling rustc-hash v1.1.0
   Compiling thiserror v1.0.69
   Compiling libc v0.2.177
   Compiling ryu v1.0.20
   Compiling lock_api v0.4.14
   Compiling getrandom v0.3.4
   Compiling serde_json v1.0.145
   Compiling ahash v0.8.12
   Compiling pin-project-lite v0.2.16
   Compiling crossbeam-utils v0.8.21
   Compiling zeroize v1.3.0
   Compiling lazy_static v1.5.0
error: The wasm32-unknown-unknown targets are not supported by default; you may need to enable the "wasm_js" configuration flag. Note that enabling the `wasm_js` feature flag alone is insufficient. For more information see: https://docs.rs/getrandom/0.3.4/#webassembly-support
   --> /home/moo-tu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends.rs:194:17
    |
194 | / ...   compile_error!(concat!(
195 | | ...       "The wasm32-unknown-unknown targets are not supported by default; \
196 | | ...       you may need to enable the \"wasm_js\" configuration flag. Note \
197 | | ...       that enabling the `wasm_js` feature flag alone is insufficient. \
198 | | ...       For more information see: \
199 | | ...       https://docs.rs/getrandom/", env!("CARGO_PKG_VERSION"), "/#webassembly-support"
200 | | ...   ));
    | |________^

   Compiling byteorder v1.5.0
error[E0425]: cannot find function `fill_inner` in module `backends`
  --> /home/moo-tu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/lib.rs:99:19
   |
99 |         backends::fill_inner(dest)?;
   |                   ^^^^^^^^^^ not found in `backends`

error[E0425]: cannot find function `inner_u32` in module `backends`
   --> /home/moo-tu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/lib.rs:123:15
    |
123 |     backends::inner_u32()
    |               ^^^^^^^^^ not found in `backends`
    |
help: consider importing this function
    |
 33 + use crate::util::inner_u32;
    |
help: if you import `inner_u32`, refer to it directly
    |
123 -     backends::inner_u32()
123 +     inner_u32()
    |

error[E0425]: cannot find function `inner_u64` in module `backends`
   --> /home/moo-tu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/lib.rs


➜  solana_account_studio git:(main) ✗ ./target/release/surfdesk-desktop
[2025-10-16T02:22:42Z INFO  surfdesk_desktop] Logging initialized at level: info
[2025-10-16T02:22:42Z INFO  surfdesk_desktop] Configuration loaded from: ./config/.env
[2025-10-16T02:22:42Z INFO  surfdesk_desktop] Starting SurfDesk Desktop application...
[2025-10-16T02:22:42Z INFO  surfdesk_desktop] Platform: Desktop
[2025-10-16T02:22:42Z INFO  surfdesk_desktop] Version: 0.1.0

thread 'main' panicked at /home/moo-tu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/env_logger-0.10.2/src/logger.rs:859:16:
env_logger::init should not be called after logger initialized: SetLoggerError(())
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[1]    235367 IOT instruction (core dumped)  ./target/release/surfdesk-desktop
➜  solana_account_studio git:(main) ✗ ./target/release/surfdesk-tui

[2025-10-16T02:22:51Z INFO  surfdesk_tui] Logging initialized at level: info
[2025-10-16T02:22:51Z INFO  surfdesk_tui] Configuration loaded from: ./config/.env
[2025-10-16T02:22:51Z INFO  surfdesk_tui] Starting SurfDesk Terminal UI application...
[2025-10-16T02:22:51Z INFO  surfdesk_tui] Platform: Desktop
[2025-10-16T02:22:51Z INFO  surfdesk_tui] Version: 0.1.0

thread 'main' panicked at /home/moo-tu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/env_logger-0.10.2/src/logger.rs:859:16:
env_logger::init should not be called after logger initialized: SetLoggerError(())
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[1]    235435 IOT instruction (core dumped)  ./target/release/surfdesk-tui


🧾 Summary of the Problem (English)

You are trying to build a Rust project for WebAssembly (WASM) using wasm-pack or cargo build --target wasm32-unknown-unknown, but the build fails due to several configuration issues:

❗ Main Issues Encountered:

1️⃣ Missing crate type configuration

The error says:

crate-type must be cdylib to compile to wasm32-unknown-unknown


Your Cargo.toml does not include:

[lib]
crate-type = ["cdylib", "rlib"]


2️⃣ getrandom crate not configured for WebAssembly

The build fails with:

The wasm32-unknown-unknown targets are not supported by default...
you may need to enable the "wasm_js" configuration flag


This means you must enable WebAssembly support:

getrandom = { version = "0.3", features = ["js"] }


3️⃣ Unexpected function errors (inner_u32, inner_u64)

Errors like:

cannot find function `inner_u64` in module `backends`


Indicate that your getrandom crate was accidentally modified or patched, breaking internal references.

🧭 Root Cause

Your Rust project is being compiled for WebAssembly without proper configuration for WASM, and the getrandom dependency was modified or corrupted in the Cargo registry.

🚑 Required Fix Steps (High-Level)
Step	Action
1	Add crate-type = ["cdylib", "rlib"] to Cargo.toml
2	Enable WebAssembly support in getrandom using features = ["js"]
3	Clean and restore Cargo registry to remove corrupted patches
4	Rebuild using wasm-pack or cargo build

If you'd like, I can generate a fixed Cargo.toml and shell script for you automatically.
