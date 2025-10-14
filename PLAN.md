PLANNING.md — SurfDesk (Dioxus + Surfpool integration)
1. Purpose & Vision

SurfDesk is a desktop application built with Dioxus 0.6+ designed to provide a rich, developer-focused interface around Surfpool (a Solana mainnet fork). It enables:

Easy setup and control of a local mainnet-fork environment

UI for deploying programs, creating transactions, managing state

An MCP (management / control panel) layer for AI-assisted test generation, scenario execution, fuzz testing

Exportable runbooks / headless execution for CI pipelines

Goal: make local Solana development & testing more visual, interactive, and robust — bridging UI + AI + fork-based testing.

1.1. Context Documentation Strategy

Each major module/library will include a `(lib)_CONTEXT.md` file that provides:

- Module purpose and responsibilities
- Key interfaces and data structures
- Usage patterns and examples
- Dependencies and integration points
- Testing approaches and guidelines
- Performance considerations

Context files serve as developer onboarding documentation and AI reference material for understanding codebase architecture.

2. Key Components & Modules
Module	Responsibility	Notes / Interfaces	Context Documentation
surfpool_controller	Start, stop, monitor the Surfpool process or embed via crate	Expose logs, status, version, flags	surfpool_controller_CONTEXT.md
rpc_client	Interact via Solana JSON-RPC (and control / cheatcode RPCs)	Use solana-client + custom JSON-RPC methods	rpc_client_CONTEXT.md
program_manager	Build & deploy programs (Anchor, non-Anchor)	Detect Anchor projects, manage artifacts	program_manager_CONTEXT.md
transaction_builder	UI + logic to build instructions & transactions	Use IDL for forms or raw JSON fallback	transaction_builder_CONTEXT.md
account_inspector	Explore, override, reset accounts & state	Integrate cheatcodes for setting state	account_inspector_CONTEXT.md
time_control	Advance / rewind / freeze time/slots	Expose UI to control chain time	time_control_CONTEXT.md
mcp_ai	Prompt + generate test plans; scenario runner	Structured plan schema, runner, result analyzer	mcp_ai_CONTEXT.md
ui	Dioxus components	Views: Dashboard, Program, Tx, Inspector, MCP, Logs	ui_CONTEXT.md
cli_headless	Headless runner mode	for CI / automation using same core logic	cli_headless_CONTEXT.md
3. Architectural Decisions

Integration Mode

Support both external Surfpool process and embedded via crate (if stable).

Default to external for simplicity; allow embedding when possible.

Process Management

Use tokio::process::Command to spawn Surfpool; capture stdout/stderr; monitor exit.

Expose lifecycle operations (start, stop, restart, reset fork).

RPC & Control Surface

Use Solana RPC for account queries, simulate/send transactions.

Use custom JSON-RPC methods (cheatcodes/control methods) for state override, time travel, etc.

Abstract these into a ControlClient layer so UI & MCP don’t care about raw RPC strings.

IDL / Anchor Integration

Automatically detect Anchor projects (looking for Anchor.toml, programs/, target/idl/)

Parse IDLs to build UI forms for instructions.

For non-Anchor programs, use fallback JSON editor.

Test Plan Schema

Represent test scenarios as structured JSON (not raw tx bytes).

Steps include instruction, override_account, time_travel, expect_assert, etc.

Runner executes steps, collects RPC & diff logs, flags failures.

AI Layer (MCP)

LLM prompts generate structured plans given IDL + test goal.

Use rules / schema to validate AI output before executing.

Provide user override/edits before run.

Headless / CI Mode

Run the same test plans in CI using surfpool start + runner logic

Return exit code on failures

Output JSON / HTML report logs

4. Feature Roadmap & Milestones
Phase	Features	Objectives / Deliverables
Phase 0	Project scaffold	Dioxus skeleton, Surfpool start/stop, log viewer
Phase 1	RPC client & basic transaction flow	Manual JSON tx builder, simulate & send
Phase 2	IDL-driven UI + program deploy	Anchor detection, deploy, instruction form UI
Phase 3	State control features	Account override, time travel, reset fork via UI
Phase 4	MCP v1	Prompt-based test generation, execution, results
Phase 5	Headless / CI export & runner	CLI mode, runbooks, short CI integration
Phase 6	Polish, UX, error handling, versioning	Stable release, error messages, update logic
5. Open Questions & Risks

Embedding stability: Is using the Surfpool crate (if available) stable across versions? Might tie your app to its internals.

Control RPC surface: What exact cheatcode or admin JSON-RPC methods does Surfpool expose? (Need to inspect its source).

LLM safety: Generated plans must be verified and sandboxed (simulate before send).

Version compatibility: If Surfpool upgrades, your UI or control logic might break. Need version detection and compatibility layer.

Resource constraints: Running many transactions / fuzzing could stress memory/CPU. Must limit concurrency.

Anchor & custom IDL coverage: Non-Anchor programs might not have clean IDL; your fallback editor must be robust.

User identity & keys: How to manage keypairs safely in the desktop app (encrypted storage, import/export, ephemeral use).

6. Context Documentation Standards

6.1. (lib)_CONTEXT.md Template Structure

```markdown
# [Module Name] Context Documentation

## Purpose
- Clear description of module's primary responsibility
- Problems it solves within the SurfDesk ecosystem

## Key Interfaces
- Public APIs and their signatures
- Data structures and enums
- Event/message types

## Usage Patterns
- Common usage examples
- Integration patterns with other modules
- Best practices and gotchas

## Dependencies
- External crates and their versions
- Internal module dependencies
- Required configuration or setup

## Testing Strategy
- Unit test approaches
- Integration test scenarios
- Mock requirements

## Performance Notes
- Bottlenecks and optimization opportunities
- Resource usage patterns
- Scaling considerations

## Future Enhancements
- Planned improvements
- Extension points for developers
```

6.2. Documentation Maintenance

- Context files updated alongside code changes
- Examples verified in CI/CD pipeline
- Cross-references kept in sync with actual implementation
- Review during architecture changes

7. Next Actions (short-term)

Investigate Surfpool's repo

List available RPC methods, control methods, embed APIs

Understand how auto-deploy works internally

Create context documentation template

Define (lib)_CONTEXT.md standards and examples

Build skeleton repo

Dioxus "Hello world" + surfpool spawn/stop logic

Basic UI to show logs and state

Define test plan JSON schema

Draft schema (steps, actions, assertions)

Build runner stub that reads the schema

Integrate Solana RPC client

Connect to localhost RPC, query accounts, simulate tx

Wire simple transaction UI

JSON editor + simulate + send

Bridge to MCP AI

Simple prompt ➝ plan draft (hardcode or minimal LLM)

Execute plan stub, show results

Implement agent guidelines and AI working rules

Create agent.md with pre/post work validation
