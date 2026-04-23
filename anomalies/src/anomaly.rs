//! Traits for structured, categorized errors.
//!
//! [`Anomaly`] is the base trait. Implement [`HasCategory`] and [`HasStatus`] on your
//! error type, then add an empty `impl Anomaly for YourType {}` to opt in to the trait
//! bound. Callers can then accept `impl Anomaly` and inspect the category or status
//! without knowing the concrete type.
//!
//! ## Choosing a trait
//!
//! **Fix** — an example of how a programmer or operator might resolve the problem.
//! **Song** — the Hall & Oates song associated with this category, courtesy of
//! [Cognitect's anomalies](https://github.com/cognitect-labs/anomalies).
//!
//! | Trait | `status()` convention | Fix | Song |
//! |---|---|---|---|
//! | [`Unavailable`] | `Temporary` | make sure callee is healthy | Out of Touch |
//! | [`Interrupted`] | caller decides | stop interrupting | It Doesn't Matter Anymore |
//! | [`Busy`] | `Temporary` | backoff and retry | Wait For Me |
//! | [`Incorrect`] | `Permanent` | fix caller bug | You'll Never Learn |
//! | [`Forbidden`] | `Permanent` | fix caller creds | I Can't Go For That |
//! | [`Unsupported`] | `Permanent` | fix caller verb | Your Imagination |
//! | [`NotFound`] | caller decides | fix caller noun | She's Gone |
//! | [`Conflict`] | `Permanent` | coordinate with callee | Give It Up |
//! | [`Fault`] | `Permanent` | fix callee bug | Falling |

use std::error::Error;

use crate::{category::Category, status::Status};

pub use anomalies_derive::Anomaly;

/// A structured error that carries a [`Category`] and a retry [`Status`].
///
/// Implementing by hand:
///
/// ```rust
/// use std::fmt;
/// use anomalies::{
///   anomaly::{Anomaly, HasCategory, HasStatus},
///   category::{Category, Unavailable},
///   status::Status};
///
/// #[derive(Debug)]
/// struct ServiceDown;
///
/// impl fmt::Display for ServiceDown {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "service down") }
/// }
/// impl std::error::Error for ServiceDown {}
///
/// impl HasCategory for ServiceDown {
///     fn category(&self) -> Category { Unavailable }
/// }
///
/// impl HasStatus for ServiceDown {
///     fn status(&self) -> Status { Status::Temporary }
/// }
///
/// impl Anomaly for ServiceDown { }
/// ```
pub trait Anomaly: Error + HasCategory + HasStatus {}

/// Returns the [`Category`] that classifies this anomaly.
pub trait HasCategory {
    /// Returns the category that classifies this anomaly.
    fn category(&self) -> Category;
}

/// Returns the retry [`Status`] for this anomaly.
pub trait HasStatus {
    /// Returns whether the caller should retry.
    fn status(&self) -> Status;
}
