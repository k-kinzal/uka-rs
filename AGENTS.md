# Agent Handbook

## Mission & Scope
- Build and maintain the ukagaka (伺か) Rust workspace across protocol, runtime, and utility crates.
- Deliver changes that respect Materia-era specifications while incorporating SSP/UKADOC extensions when needed.
- Keep the workspace healthy by enforcing lint, test, and documentation standards before code review.

## Environment Setup
- Install the latest stable Rust toolchain and ensure `cargo` commands are available in PATH.
- Recommended components: `rustfmt`, `clippy`, `rust-src`, and a Tokio-aware IDE or LSP.
- Platform targets: develop primarily on Unix-like systems; validate Windows-specific HGLOBAL behaviour within Windows runners or VMs.
- Optional tooling: `cargo-nextest` for faster test loops, and `just`/shell scripts for task shortcuts when available.

## Daily Workflow Checklist
1. Pull the latest changes and review open PRs for context.
2. Review the protocol priorities in the Decision-Making Hierarchy below before coding.
3. Sketch the change: identify affected crates (`uka_sstp`, `uka_shiori`, `uka_macro`, `uka_util`, `examples/ghost`).
4. Update or add tests alongside the implementation.
5. Run the command suite before pushing:
   ```bash
   cargo fmt
   cargo clippy --all-targets --all-features
   cargo test
   cargo doc --workspace --no-deps
   ```
6. Capture platform notes (especially Windows vs Unix expectations) in commit or PR descriptions.

## Quick Command Reference
```bash
# Build the entire workspace
cargo build

# Build the example ghost DLL
cargo build -p ghost --release

# Run workspace tests or focus on a crate
cargo test
cargo test -p uka_sstp
cargo test -p uka_shiori

# Narrow scope further when debugging
cargo test test_name
cargo test --package uka_sstp header::tests
cargo test -- --nocapture
```

## Workspace Map
- `uka_sstp`: Sakura Script Transfer Protocol parsing, request/response builders, status handling, multi-version and charset support; integration tests in `uka_sstp/tests`.
- `uka_shiori`: SHIORI/3.0 runtime built on Tokio with DLL adapters, service traits, and FFI bridge; contract tests in `uka_shiori/tests`.
- `uka_util`: Cross-cutting helpers such as pointer abstractions, allocators, and encoding conversions.
- `uka_macro`: Compile-time utilities used by the other crates.
- `examples/ghost`: Minimal ghost package for end-to-end validation and release DLL builds.

## Architecture & Patterns
- **Layered design:** SSTP (protocol), SHIORI (runtime), UTIL (infrastructure) with clear ownership boundaries.
- **Builders for protocol safety:** Use request/response builders to enforce required headers and charset agreements at compile time.
- **Service traits:** SHIORI handlers implement `Service<C, Request>` with async futures to keep the runtime modular.
- **FFI safety:** Distinguish `OwnedPtr` vs `RawPtr`; hide platform-specific details behind `#[cfg]` blocks; release HGLOBAL allocations correctly on Windows.
- **Error handling:** Favor `thiserror` for descriptive errors and map protocol or FFI failures to actionable diagnostics.

## Coding Standards
- Four-space indentation; `snake_case` items, `UpperCamelCase` types, `SCREAMING_SNAKE_CASE` constants.
- Order imports: standard library → external crates → local modules; avoid glob imports outside tests.
- Document public items with concise explanations and doctest-friendly examples using realistic SSTP or SHIORI messages.
- Keep comments purposeful: highlight non-obvious decisions, specification nuances, or platform workarounds.

## Testing Strategy
- Write unit tests alongside source modules; use crate-level `tests/` for black-box, regression, and FFI contract coverage.
- Encode regressions with meaningful names, e.g., `test_windows_hglobal_issue147`.
- Add doctests for public APIs and ensure they compile via `cargo doc --no-deps`.
- Gate Windows-specific tests with `#[cfg(windows)]` and provide mocks or fallbacks for cross-platform runs.

## Platform Guidance
- Primary target is Windows for DLL interoperability; ensure pointer lifetimes and HGLOBAL allocations obey Win32 rules.
- Secondary target is Unix-like systems for day-to-day development; keep abstractions portable and guard platform-specific code.
- Charset handling must support Shift_JIS and UTF-8 across SSTP messages; add regression tests when modifying encoding logic.

## Collaboration & Delivery
- Use imperative, present-tense commit titles under 72 characters; include body context when touching multiple crates or platform-specific paths.
- Pull requests should outline scope, validation commands run, and any platform assumptions.
- Prefer textual reproduction steps; attach logs or screenshots only when essential.
- Coordinate with other contributors when altering shared abstractions or public APIs to avoid breaking dependents.

## Decision-Making Hierarchy
1. Materia-era specifications (SSTP/1.x, SHIORI/1.x–3.0, Shell/SERIKO/MAYUNA, INSTALL flows).
2. SSP/UKADOC interpretations for gaps and clarifications.
3. Project-defined behaviour with documented rationale; guard behind configuration toggles when practical.

## Additional Resources
- `uka_shiori/src/runtime/shiori.rs` for async orchestration patterns.
- `uka_sstp/src/parse.rs` for cursor-based parsing strategies.
- `uka_util/src/ptr/` and `uka_util/src/alloc.rs` for memory management abstractions.
- Reach out to maintainers via repository discussions or issue tracker for cross-cutting decisions.
