//! Traits for structured, categorized errors.
//!
//! [`Anomaly<C>`] is the base trait. The category-specific sub-traits
//! ([`Unavailable`], [`Busy`], [`Incorrect`], etc.) each have a blanket impl that
//! automatically satisfies `Anomaly<C>`, so implementors only need to declare one
//! `impl` block — and for categories with unambiguous status, that block can be empty.
//!
//! ## Choosing a trait
//!
//! **Fix** — an example of how a programmer or operator might resolve the problem.
//! **Song** — the Hall & Oates song associated with this category, courtesy of
//! [Cognitect's anomalies](https://github.com/cognitect-labs/anomalies).
//!
//! | Trait | `status()` default | Fix | Song |
//! |---|---|---|---|
//! | [`Unavailable`] | `Temporary` | make sure callee is healthy | Out of Touch |
//! | [`Interrupted`] | — | stop interrupting | It Doesn't Matter Anymore |
//! | [`Busy`] | `Temporary` | backoff and retry | Wait For Me |
//! | [`Incorrect`] | `Permanent` | fix caller bug | You'll Never Learn |
//! | [`Forbidden`] | `Permanent` | fix caller creds | I Can't Go For That |
//! | [`Unsupported`] | `Permanent` | fix caller verb | Your Imagination |
//! | [`NotFound`] | — | fix caller noun | She's Gone |
//! | [`Conflict`] | `Permanent` | coordinate with callee | Give It Up |
//! | [`Fault`] | `Permanent` | fix callee bug | Falling |

use std::error::Error;

use crate::{
    category::{self, Category},
    status::Status,
};

/// A structured error that carries a [`Category`] and a [`Status`].
///
/// `Anomaly<C>` is the base trait for generic callers. You do not implement this
/// directly; implement one of the category-specific sub-traits instead and the
/// blanket impl will satisfy `Anomaly<C>` automatically.
///
/// Generic callers that need to handle *any* category should bound on
/// `Anomaly<impl Category>`. Callers that only handle a specific category should
/// bound on the corresponding sub-trait (e.g. [`NotFound`]).
pub trait Anomaly<C: Category>: Error {
    /// Returns the category of this anomaly.
    fn category(&self) -> C;

    /// Returns whether retrying the same request is safe and likely to succeed.
    fn status(&self) -> Status;
}

/// A resource or service that was expected to be reachable is not.
///
/// An empty `impl Unavailable for MyError {}` is valid.
pub trait Unavailable: Error {}

impl<T: Unavailable> Anomaly<category::Unavailable> for T {
    fn category(&self) -> category::Unavailable {
        category::Unavailable
    }
    fn status(&self) -> Status {
        Status::Temporary
    }
}

/// An operation that was cut short before completing.
///
/// There is no default `status()` because whether a retry is safe depends on
/// whether partial work was committed. Implementors must provide it.
pub trait Interrupted: Error {
    /// Returns whether retrying is safe given the partial-work context.
    fn status(&self) -> Status;
}

impl<T: Interrupted> Anomaly<category::Interrupted> for T {
    fn category(&self) -> category::Interrupted {
        category::Interrupted
    }
    fn status(&self) -> Status {
        <T as Interrupted>::status(self)
    }
}

/// A reachable system that is temporarily too busy to accept new work.
///
/// An empty `impl Busy for MyError {}` is valid.
pub trait Busy: Error {}

impl<T: Busy> Anomaly<category::Busy> for T {
    fn category(&self) -> category::Busy {
        category::Busy
    }
    fn status(&self) -> Status {
        Status::Temporary
    }
}

/// A request that is malformed or violates a domain invariant.
///
/// An empty `impl Incorrect for MyError {}` is valid.
pub trait Incorrect: Error {}

impl<T: Incorrect> Anomaly<category::Incorrect> for T {
    fn category(&self) -> category::Incorrect {
        category::Incorrect
    }
    fn status(&self) -> Status {
        Status::Permanent
    }
}

/// An operation that was denied due to insufficient permissions.
///
/// An empty `impl Forbidden for MyError {}` is valid.
pub trait Forbidden: Error {}

impl<T: Forbidden> Anomaly<category::Forbidden> for T {
    fn category(&self) -> category::Forbidden {
        category::Forbidden
    }
    fn status(&self) -> Status {
        Status::Permanent
    }
}

/// An operation that the implementation does not support.
///
/// An empty `impl Unsupported for MyError {}` is valid.
pub trait Unsupported: Error {}

impl<T: Unsupported> Anomaly<category::Unsupported> for T {
    fn category(&self) -> category::Unsupported {
        category::Unsupported
    }
    fn status(&self) -> Status {
        Status::Permanent
    }
}

/// A resource that does not exist at the given identifier.
///
/// There is no default `status()` because permanence depends on context: a
/// hard-deleted record is permanently gone, but a resource that hasn't been created
/// yet might appear later. Implementors must provide it.
pub trait NotFound: Error {
    /// Returns whether this absence is permanent or transient.
    fn status(&self) -> Status;
}

impl<T: NotFound> Anomaly<category::NotFound> for T {
    fn category(&self) -> category::NotFound {
        category::NotFound
    }
    fn status(&self) -> Status {
        <T as NotFound>::status(self)
    }
}

/// An operation that cannot be applied due to a conflict with existing state.
///
/// An empty `impl Conflict for MyError {}` is valid.
pub trait Conflict: Error {}

impl<T: Conflict> Anomaly<category::Conflict> for T {
    fn category(&self) -> category::Conflict {
        category::Conflict
    }
    fn status(&self) -> Status {
        Status::Permanent
    }
}

/// An internal error that is the system's fault, not the caller's.
///
/// An empty `impl Fault for MyError {}` is valid.
pub trait Fault: Error {}

impl<T: Fault> Anomaly<category::Fault> for T {
    fn category(&self) -> category::Fault {
        category::Fault
    }
    fn status(&self) -> Status {
        Status::Permanent
    }
}
