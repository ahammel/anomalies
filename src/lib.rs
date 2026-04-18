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
//! [`anomaly::Anomaly`] is the base trait; each category has a convenience sub-trait
//! (e.g. [`anomaly::NotFound`]) with sensible defaults so implementors only override what they need to.
//!
//! # Usage
//!
//! Implement one of the category-specific sub-traits on your error type:
//!
//! ```rust
//! use std::fmt;
//! use anomalies::anomaly;
//! use anomalies::status::Status;
//!
//! #[derive(Debug)]
//! struct RecordMissing { id: u64 }
//!
//! impl fmt::Display for RecordMissing {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         write!(f, "record {} not found", self.id)
//!     }
//! }
//!
//! impl std::error::Error for RecordMissing {}
//!
//! impl anomaly::NotFound for RecordMissing {
//!     fn status(&self) -> Status { Status::Permanent }
//! }
//! ```
//!
//! For categories with unambiguous status (e.g. [`anomaly::Incorrect`]), the impl
//! block can be completely empty:
//!
//! ```rust
//! # use std::fmt;
//! # #[derive(Debug)] struct BadInput;
//! # impl fmt::Display for BadInput {
//! #     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "bad input") }
//! # }
//! # impl std::error::Error for BadInput {}
//! use anomalies::anomaly;
//!
//! impl anomaly::Incorrect for BadInput {}
//! ```
//!
//! A generic caller can then branch on category or status without knowing the concrete type:
//!
//! ```rust
//! # use anomalies::anomaly::Anomaly;
//! # use anomalies::category::Category;
//! # use anomalies::status::Status;
//! fn should_retry(e: &dyn Anomaly<impl Category>) -> bool {
//!     e.status() == Status::Temporary
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
