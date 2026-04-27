//! Traits for structured, categorized errors.
//!
//! [`Anomaly`] is the base trait. Derive it with `#[derive(Anomaly)]` on a struct or enum,
//! tagging each type or variant with `#[category(...)]`. For `interrupted` and `not_found` —
//! the two categories without a fixed default status — add `#[status(...)]` to set a static
//! retry status at the derive site, or implement [`HasStatus`] by hand if the status needs to
//! vary at runtime. For all other categories the `HasStatus` impl is generated automatically.
//!
//! Callers can accept `impl Anomaly` and inspect the category or status without knowing the
//! concrete type.
//!
//! ## Choosing a category
//!
//! **Fix** — an example of how a programmer or operator might resolve the problem.
//! **Song** — the Hall & Oates song associated with this category, courtesy of
//! [Cognitect's anomalies](https://github.com/cognitect-labs/anomalies).
//!
//! | `#[category(...)]` | `status()` default | Fix | Song |
//! |---|---|---|---|
//! | `unavailable` | `Temporary` | make sure callee is healthy | Out of Touch |
//! | `interrupted` | context dependent | stop interrupting | It Doesn't Matter Anymore |
//! | `busy` | `Temporary` | backoff and retry | Wait For Me |
//! | `incorrect` | `Permanent` | fix caller bug | You'll Never Learn |
//! | `forbidden` | `Permanent` | fix caller creds | I Can't Go For That |
//! | `unsupported` | `Permanent` | fix caller verb | Your Imagination |
//! | `not_found` | — use `#[status(...)]` | fix caller noun | She's Gone |
//! | `conflict` | `Permanent` | coordinate with callee | Give It Up |
//! | `fault` | `Permanent` | fix callee bug | Falling |

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
