# AGENTS.md

This file provides guidance to AI coding agents (Claude Code, GitHub Copilot, etc.) when working with code in this repository.

## Project Overview

Unofficial Hatena Blog AtomPub API wrapper library for Rust (edition 2021, not published to crates.io).

## Build & Test Commands

```bash
cargo build                                    # Build (no default TLS feature; use --features native-tls or rustls-tls)
cargo test                                     # Run all tests
cargo test config::test::config_new            # Run a single test by name
cargo test -- --test-threads=1                 # Run tests single-threaded (needed for env var tests in config)
cargo run --features native-tls --example list_entries  # Run an example (requires env vars, see Config)
```

No linter or formatter is configured in CI. Use `cargo clippy` for linting.

**IMPORTANT: Always run `cargo +nightly fmt` after editing code to apply formatting.**

## Architecture

The library wraps Hatena Blog's AtomPub API with async HTTP calls and Atom XML parsing.

**Core flow**: `Client` (HTTP layer) → raw XML `String` → Response wrapper types (`MemberResponse`, `CollectionResponse`, etc.) → parsed Rust types via `TryFrom` conversions.

Key design decisions:
- **Response types are opaque wrappers around raw XML strings** (`MemberResponse`, `CollectionResponse`, `CategoryDocumentResponse`, `EmptyResponse`). Callers convert to domain types (`Entry`, `Vec<String>`, etc.) via `TryFrom`.
- **Two XML parsing strategies coexist**: `atom_syndication` crate for entry/feed parsing (`response.rs`), `quick-xml` for category document parsing (`response.rs:124-222`). The entry XML from single-entry responses is wrapped in `<feed>` tags before parsing with `atom_syndication`.
- **`EntryParams.into_xml()`** manually builds XML with a custom escape function rather than using a serialization library.
- **`Config`** supports both explicit construction and loading from environment variables (`HATENA_ID`, `HATENA_BLOG_ID`, `HATENA_API_KEY`, `HATENA_BLOG_BASE_URL`).
- **TLS is opt-in** via cargo features: `native-tls` or `rustls-tls` (passed through to reqwest).

## Module Responsibilities

- `client.rs` — `Client` struct, HTTP methods (CRUD + list), URL construction, basic auth
- `config.rs` — API credentials and base URL configuration
- `response.rs` — Response wrapper types, Atom/XML parsing, `TryFrom` conversions to domain types
- `entry.rs` — `Entry` domain struct
- `entry_params.rs` — `EntryParams` for create/update with XML serialization
- `entry_id.rs` — `EntryId` newtype wrapper
- `fixed_date_time.rs` — `FixedDateTime` newtype over `chrono::DateTime<FixedOffset>`

## Coding Conventions

- Newtype pattern for domain primitives (`EntryId`, `FixedDateTime`) with `FromStr`/`Display` implementations
- Error types use `thiserror` derive macros
- Tests are inline (`#[cfg(test)] mod test`) within each module, not in a separate `tests/` directory
- All public types are re-exported from `lib.rs`
