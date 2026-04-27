# anomalies

Structured, categorized error handling for Rust.

## Motivation

Most error-handling advice boils down to "add context and forward." That approach produces errors that are richly described but semantically opaque: every error becomes a unique snowflake that callers must pattern-match on to decide what to do next.

This crate takes a different view, inspired by [Cognitect's anomalies](https://github.com/cognitect-labs/anomalies). An error type that implements one of this crate's `anomaly` traits signals its *category* — a stable, cross-cutting classification that callers can act on without knowing anything about the specific error type.

## Design

Three orthogonal concepts:

- **category** — *what kind of problem is this?* (e.g. the caller sent bad data, or the system is temporarily overloaded, or the record doesn't exist). Categories are zero-sized marker types so they carry no runtime overhead and can be used as generic type parameters.

- **status** — *should the caller retry?* `Status::Temporary` means the same request might succeed later; `Status::Permanent` means it won't.

- **anomaly** — a `std::error::Error` with a category and a status. Derive `Anomaly` on a struct or enum with `#[category(...)]` (and optionally `#[status(...)]`) attributes to get a full implementation, or implement `HasCategory` and `HasStatus` by hand and add an empty `impl Anomaly for YourType {}`.

## Categories

**Fix** — an example of how a programmer or operator might resolve the problem.  
**Song** — the Hall & Oates song associated with this category, courtesy of [Cognitect's anomalies](https://github.com/cognitect-labs/anomalies).

| `#[category(...)]` | `status()` default | Fix | Song |
|---|---|---|---|
| `unavailable` | `Temporary` | make sure callee is healthy | Out of Touch |
| `interrupted` | context dependent | stop interrupting | It Doesn't Matter Anymore |
| `busy` | `Temporary` | backoff and retry | Wait For Me |
| `incorrect` | `Permanent` | fix caller bug | You'll Never Learn |
| `forbidden` | `Permanent` | fix caller creds | I Can't Go For That |
| `unsupported` | `Permanent` | fix caller verb | Your Imagination |
| `not_found` | context dependent | fix caller noun | She's Gone |
| `conflict` | `Permanent` | coordinate with callee | Give It Up |
| `fault` | `Permanent` | fix callee bug | Falling |

`interrupted` and `not_found` have no default status because the right answer depends on context.
For all other categories the status is fixed.

## Usage

### Structs

Derive `Anomaly` and tag your struct with `#[category(...)]`. Categories with a fixed default
status need nothing else; for `interrupted` and `not_found`, add `#[status(...)]` to set a
static status at the derive site:

```rust
use std::fmt;
use anomalies::anomaly::Anomaly;

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

// `not_found` has no default — set one with #[status(...)].
#[derive(Anomaly, Debug)]
#[category(not_found)]
#[status(permanent)]
struct RecordMissing { id: u64 }

impl fmt::Display for RecordMissing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "record {} not found", self.id)
    }
}
impl std::error::Error for RecordMissing {}
```

If the status needs to vary at runtime (e.g. `interrupted` where retry depends on whether the
work was committed), implement `HasStatus` by hand instead of using `#[status(...)]`.

### Enums

Each variant carries its own `#[category(...)]` and optional `#[status(...)]`:

```rust
use std::fmt;
use anomalies::anomaly::Anomaly;

#[derive(Anomaly, Debug)]
enum RepoError {
    #[category(not_found)]
    #[status(permanent)]
    Missing { id: u64 },

    #[category(fault)]
    Unexpected(String),
}

impl fmt::Display for RepoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing { id } => write!(f, "record {id} not found"),
            Self::Unexpected(msg) => write!(f, "unexpected error: {msg}"),
        }
    }
}
impl std::error::Error for RepoError {}
```

`thiserror`'s `#[error(...)]` attribute coexists with `#[category(...)]` / `#[status(...)]`
on the same variants without conflict.

For variants that wrap another `Anomaly` implementation, use `#[anomaly(transparent)]` to
delegate `category()` and `status()` to the inner value:

```rust
#[derive(Anomaly, Debug)]
enum ServiceError {
    #[category(incorrect)]
    BadRequest(String),

    #[anomaly(transparent)]
    Database(DbError),  // category() and status() come from DbError
}
```

The inner type must implement `Anomaly`. `#[anomaly(transparent)]` cannot be combined with
`#[category(...)]` or `#[status(...)]` on the same variant.

### Generic callers

A caller can branch on category or status without knowing the concrete type:

```rust
use anomalies::anomaly::Anomaly;
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
