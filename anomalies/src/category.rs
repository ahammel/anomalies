#![allow(non_upper_case_globals)]
//! Zero-sized marker types that classify anomalies by kind.
//!
//! Each constant in this module answers the question *what kind of problem is this?*
//! Return one from [`crate::anomaly::HasCategory::category`] to classify your error type.
use std::fmt;

/// Marker trait for anomaly categories.
///
/// A category answers the question *what kind of problem is this?* independently
/// of the specific error type. Categories are zero-sized marker types — they exist
/// only to carry type-level information and have no runtime cost.
///
/// You generally do not implement this trait directly; use one of the provided
/// concrete categories instead. The trait exists as a bound on [`crate::anomaly::Anomaly`]
/// so that generic code can accept any category.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Category(&'static str);

impl Category {
    /// Creates a new category with the given name.
    ///
    /// Use this to define custom categories outside the crate:
    ///
    /// ```rust
    /// use anomalies::category::Category;
    /// pub static RateLimited: Category = Category::new("rate-limited");
    /// ```
    pub const fn new(name: &'static str) -> Self {
        Category(name)
    }
}

impl fmt::Debug for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The requested resource or service is not currently reachable.
///
/// Unlike [`Busy`], `Unavailable` implies the dependency itself is absent or
/// down — not merely overloaded. The request may succeed later if the dependency
/// recovers.
///
/// Default status: [`Status::Temporary`](crate::status::Status::Temporary).
pub const Unavailable: Category = Category("unavailable");

/// The operation was cut short before it could complete.
///
/// The system was reachable and processing the request, but something (a timeout,
/// a cancellation signal, a network reset) stopped it mid-flight. Whether the
/// caller should retry depends on whether the partial work was committed; there is
/// no universal default status for this category.
pub const Interrupted: Category = Category("interrupted");

/// The system is reachable but overloaded and cannot accept more work right now.
///
/// Unlike [`Unavailable`], the dependency is up and healthy — it just has more
/// demand than capacity at this moment. Callers should back off and retry.
///
/// Default status: [`Status::Temporary`](crate::status::Status::Temporary).
pub const Busy: Category = Category("busy");

/// The request itself is malformed or invalid.
///
/// The problem is with what was asked, not with the system's current state.
/// Retrying the same request unchanged will not help.
///
/// Default status: [`Status::Permanent`](crate::status::Status::Permanent).
pub const Incorrect: Category = Category("incorrect");

/// The caller does not have permission to perform the operation.
///
/// The request was understood but authorization was denied. Retrying with the same
/// credentials will not help; the caller needs elevated privileges or a different
/// identity.
///
/// Default status: [`Status::Permanent`](crate::status::Status::Permanent).
pub const Forbidden: Category = Category("forbidden");

/// The operation is not supported by this implementation.
///
/// The system understood the request but has no capability to fulfill it. This is
/// a permanent condition: the feature simply does not exist.
///
/// Default status: [`Status::Permanent`](crate::status::Status::Permanent).
pub const Unsupported: Category = Category("unsupported");

/// The requested resource does not exist.
///
/// The identifier or path is valid but points to nothing. Whether this is
/// permanent depends on context (e.g. a deleted record vs. a race with a
/// concurrent create), so there is no universal default status for this category.
pub const NotFound: Category = Category("not-found");

/// The operation cannot be applied because it conflicts with existing state.
///
/// Typically a uniqueness violation, an optimistic-lock mismatch, or a
/// precondition failure. Retrying the same request unchanged will not resolve
/// the conflict; the caller must reconcile the state difference first.
///
/// Default status: [`Status::Permanent`](crate::status::Status::Permanent).
pub const Conflict: Category = Category("conflict");

/// An internal error that is the system's fault, not the caller's.
///
/// The request was valid but something went wrong internally — a bug, a failed
/// invariant, an unexpected state. The caller cannot fix this by changing their
/// request.
///
/// Default status: [`Status::Permanent`](crate::status::Status::Permanent).
pub const Fault: Category = Category("fault");
