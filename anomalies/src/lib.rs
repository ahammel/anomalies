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
//! - **[`anomaly`]** — a [`std::error::Error`] with a [`category::Category`] and a [`status::Status`].
//!   Derive [`anomaly::Anomaly`] with a `#[category(...)]` attribute, or implement
//!   [`anomaly::HasCategory`] and [`anomaly::HasStatus`] by hand and add an empty
//!   `impl Anomaly for YourType {}`.
//!
//! # Usage
//!
//! Derive `Anomaly` and tag your type with the appropriate `#[category(...)]`:
//!
//! ```rust
//! use std::fmt;
//! use anomalies::anomaly::{Anomaly, HasStatus};
//! use anomalies::status::Status;
//!
//! // `not_found` has no default status — provide HasStatus explicitly.
//! #[derive(Anomaly, Debug)]
//! #[category(not_found)]
//! struct RecordMissing { id: u64 }
//!
//! impl fmt::Display for RecordMissing {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         write!(f, "record {} not found", self.id)
//!     }
//! }
//! impl std::error::Error for RecordMissing {}
//! impl HasStatus for RecordMissing {
//!     fn status(&self) -> Status { Status::Permanent }
//! }
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
