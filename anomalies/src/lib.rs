//! Structured, categorized error handling for Rust.
//!
//! # Motivation
//!
//! Most error-handling advice boils down to "add context and forward." That approach
//! produces errors that are richly described but semantically opaque: every error becomes
//! a unique snowflake that callers must pattern-match on to decide what to do next.
//!
//! This crate takes a different view, inspired by
//! [Cognitect's anomalies](https://github.com/cognitect-labs/anomalies). An error type that
//! implements one of this crate's [`anomaly`] traits signals its *category* — a stable,
//! cross-cutting classification that callers can act on without knowing anything about the
//! specific error type.
//!
//! # Design
//!
//! Three orthogonal concepts:
//!
//! - **[`category`]** — *what kind of problem is this?* (e.g. the caller sent bad data, or
//!   the system is temporarily overloaded, or the record doesn't exist). Categories are
//!   zero-sized marker types so they carry no runtime overhead and can be used as generic
//!   type parameters.
//!
//! - **[`status`]** — *should the caller retry?* [`status::Status::Temporary`] means the same
//!   request might succeed later; [`status::Status::Permanent`] means it won't.
//!
//! - **[`anomaly`]** — a [`std::error::Error`] with a [`category::Category`] and a
//!   [`status::Status`]. Derive [`anomaly::Anomaly`] on a struct or enum with `#[category(...)]`
//!   (and optionally `#[status(...)]`) attributes, or implement [`anomaly::HasCategory`] and
//!   [`anomaly::HasStatus`] by hand and add an empty `impl Anomaly for YourType {}`.
//!
//! # Usage
//!
//! Derive `Anomaly` and tag each type (or variant) with `#[category(...)]`. Categories with a
//! fixed default status need nothing else; for `interrupted` and `not_found`, add
//! `#[status(...)]` to set a static retry status at the derive site:
//!
//! ```rust
//! use std::fmt;
//! use anomalies::anomaly::Anomaly;
//!
//! // `fault` has a fixed default status — only Display and Error are needed.
//! #[derive(Anomaly, Debug)]
//! #[category(fault)]
//! struct DbConnectionFailed;
//!
//! impl fmt::Display for DbConnectionFailed {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         write!(f, "database connection failed")
//!     }
//! }
//! impl std::error::Error for DbConnectionFailed {}
//!
//! // `not_found` has no default — set one with #[status(...)].
//! #[derive(Anomaly, Debug)]
//! #[category(not_found)]
//! #[status(permanent)]
//! struct RecordMissing { id: u64 }
//!
//! impl fmt::Display for RecordMissing {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         write!(f, "record {} not found", self.id)
//!     }
//! }
//! impl std::error::Error for RecordMissing {}
//! ```
//!
//! Enums are also supported — each variant gets its own `#[category(...)]` and
//! `#[status(...)]` where required. Variants that wrap another [`anomaly::Anomaly`] type
//! can use `#[anomaly(transparent)]` to delegate `category()` and `status()` to the inner
//! value:
//!
//! ```rust
//! use std::fmt;
//! use anomalies::anomaly::{Anomaly, HasCategory, HasStatus};
//! use anomalies::category::{Category, Fault};
//! use anomalies::status::Status;
//!
//! // An inner error type with runtime-varying status.
//! #[derive(Debug)]
//! struct DbError { retryable: bool }
//! impl fmt::Display for DbError {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "db error") }
//! }
//! impl std::error::Error for DbError {}
//! impl HasCategory for DbError {
//!     fn category(&self) -> Category { Fault }
//! }
//! impl HasStatus for DbError {
//!     fn status(&self) -> Status {
//!         if self.retryable { Status::Temporary } else { Status::Permanent }
//!     }
//! }
//! impl Anomaly for DbError {}
//!
//! #[derive(Anomaly, Debug)]
//! enum RepoError {
//!     #[category(not_found)]
//!     #[status(permanent)]
//!     Missing { id: u64 },
//!
//!     #[category(incorrect)]
//!     BadRequest(String),
//!
//!     // Delegates category() and status() to the inner DbError.
//!     #[anomaly(transparent)]
//!     Database(DbError),
//! }
//!
//! impl fmt::Display for RepoError {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         match self {
//!             Self::Missing { id } => write!(f, "record {id} not found"),
//!             Self::BadRequest(msg) => write!(f, "bad request: {msg}"),
//!             Self::Database(e) => write!(f, "database error: {e}"),
//!         }
//!     }
//! }
//! impl std::error::Error for RepoError {}
//! ```
//!
//! # Prior art
//!
//! - [**Cognitect anomalies**](https://github.com/cognitect-labs/anomalies) — the original
//!   Clojure library this crate is modelled on. Defines the category vocabulary used here.
//! - [**"Stop Forwarding Errors, Start Designing Them"**](https://fast.github.io/blog/stop-forwarding-errors-start-designing-them/) —
//!   the essay that articulates why categorized, actionable errors are preferable to
//!   forwarded error chains.
//! - [**Xuanwo, "How I think about errors" (2022-46)**](https://xuanwo.io/en-us/reports/2022-46/) —
//!   the source of the [`status::Status`] vocabulary, including the three-way
//!   `Temporary` / `Persistent` / `Permanent` distinction.

pub mod anomaly;
pub mod category;
pub mod status;

#[cfg(test)]
mod tests;
