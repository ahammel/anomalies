# anomalies

Structured, categorized error handling for Rust.

## Motivation

Most error-handling advice boils down to "add context and forward." That approach produces errors that are richly described but semantically opaque: every error becomes a unique snowflake that callers must pattern-match on to decide what to do next.

This crate takes a different view, inspired by [Cognitect's anomalies](https://github.com/cognitect-labs/anomalies). An error type that implements one of this crate's `anomaly` traits signals its *category* — a stable, cross-cutting classification that callers can act on without knowing anything about the specific error type.

## Design

Three orthogonal concepts:

- **category** — *what kind of problem is this?* (e.g. the caller sent bad data, or the system is temporarily overloaded, or the record doesn't exist). Categories are zero-sized marker types so they carry no runtime overhead and can be used as generic type parameters.

- **status** — *should the caller retry?* `Status::Temporary` means the same request might succeed later; `Status::Permanent` means it won't.

- **anomaly** — a `std::error::Error` with a category and a status. Derive `Anomaly` with a `#[category(...)]` attribute to get a full implementation, or implement `HasCategory` and `HasStatus` by hand and add an empty `impl Anomaly for YourType {}`.

## Categories

**Fix** — an example of how a programmer or operator might resolve the problem.  
**Song** — the Hall & Oates song associated with this category, courtesy of [Cognitect's anomalies](https://github.com/cognitect-labs/anomalies).

| `#[category(...)]` | `status()` default | Fix | Song |
|---|---|---|---|
| `unavailable` | `Temporary` | make sure callee is healthy | Out of Touch |
| `interrupted` | — | stop interrupting | It Doesn't Matter Anymore |
| `busy` | `Temporary` | backoff and retry | Wait For Me |
| `incorrect` | `Permanent` | fix caller bug | You'll Never Learn |
| `forbidden` | `Permanent` | fix caller creds | I Can't Go For That |
| `unsupported` | `Permanent` | fix caller verb | Your Imagination |
| `not_found` | — | fix caller noun | She's Gone |
| `conflict` | `Permanent` | coordinate with callee | Give It Up |
| `fault` | `Permanent` | fix callee bug | Falling |

`Interrupted` and `NotFound` have no default status because the right answer depends on context. For all other categories the status is fixed and the impl block can be empty.

## Usage

Derive `Anomaly` and tag your type with the appropriate `#[category(...)]`. Categories with a
fixed default status generate a `HasStatus` impl automatically; `interrupted` and `not_found`
require you to provide one:

```rust
use std::fmt;
use anomalies::anomaly::{Anomaly, HasStatus};
use anomalies::status::Status;

// `fault` has a fixed default status — only Display and Error are needed.
#[derive(Anomaly, Debug)]
#[category(fault)]
struct DbConnectionFailed;

impl fmt::Display for DbConnectionFailed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "database connection failed")
    }
}
impl std::error::Error for DbConnectionFailed {}

// `not_found` has no default status — provide HasStatus explicitly.
#[derive(Anomaly, Debug)]
#[category(not_found)]
struct RecordMissing { id: u64 }

impl fmt::Display for RecordMissing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "record {} not found", self.id)
    }
}
impl std::error::Error for RecordMissing {}
impl HasStatus for RecordMissing {
    fn status(&self) -> Status { Status::Permanent }
}
```

A generic caller can branch on category or status without knowing the concrete type:

```rust
use anomalies::anomaly::{Anomaly, HasStatus};
use anomalies::status::Status;

fn should_retry(e: &dyn Anomaly) -> bool {
    e.status() == Status::Temporary
}
```

## Versioning

This crate uses a modified semantic versioning policy designed for long-term stability.

**Version 0.x (current):** the API is still being shaped.
- Patch bumps (`0.1.0 → 0.1.1`) are non-breaking.
- Minor bumps (`0.1 → 0.2`) may include breaking changes.

**Version 1.x (future):** the API is stable.
- Minor bumps (`1.0 → 1.1`) add new features without breaking existing code.
- Patch bumps (`1.0.0 → 1.0.1`) are bug fixes and other non-functional changes.
- Breaking changes will never be released. There will never be a version 2.

## Prior art

- [**Cognitect anomalies**](https://github.com/cognitect-labs/anomalies) — the original Clojure library this crate is modelled on. Defines the category vocabulary used here.
- [**"Stop Forwarding Errors, Start Designing Them"**](https://fast.github.io/blog/stop-forwarding-errors-start-designing-them/) — the essay that articulates why categorized, actionable errors are preferable to forwarded error chains.
- [**Xuanwo, "How I think about errors" (2022-46)**](https://xuanwo.io/en-us/reports/2022-46/) — the source of the `Status` vocabulary, including the three-way `Temporary` / `Persistent` / `Permanent` distinction.
