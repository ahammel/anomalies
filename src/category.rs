//! Zero-sized marker types that classify anomalies by kind.
//!
//! Each type in this module answers the question *what kind of problem is this?*
//! They are used as the type parameter `C` on [`crate::anomaly::Anomaly<C>`] and
//! as supertrait bounds on the convenience traits in [`crate::anomaly`].

/// Marker trait for anomaly categories.
///
/// A category answers the question *what kind of problem is this?* independently
/// of the specific error type. Categories are zero-sized marker types — they exist
/// only to carry type-level information and have no runtime cost.
///
/// You generally do not implement this trait directly; use one of the provided
/// concrete categories instead. The trait exists as a bound on [`crate::anomaly::Anomaly`]
/// so that generic code can accept any category.
pub trait Category {}

/// The requested resource or service is not currently reachable.
///
/// Unlike [`Busy`], `Unavailable` implies the dependency itself is absent or
/// down — not merely overloaded. The request may succeed later if the dependency
/// recovers.
///
/// Default status: [`Status::Temporary`](crate::status::Status::Temporary).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unavailable;
impl Category for Unavailable {}

/// The operation was cut short before it could complete.
///
/// The system was reachable and processing the request, but something (a timeout,
/// a cancellation signal, a network reset) stopped it mid-flight. Whether the
/// caller should retry depends on whether the partial work was committed; there is
/// no universal default status for this category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interrupted;
impl Category for Interrupted {}

/// The system is reachable but overloaded and cannot accept more work right now.
///
/// Unlike [`Unavailable`], the dependency is up and healthy — it just has more
/// demand than capacity at this moment. Callers should back off and retry.
///
/// Default status: [`Status::Temporary`](crate::status::Status::Temporary).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Busy;
impl Category for Busy {}

/// The request itself is malformed or invalid.
///
/// The problem is with what was asked, not with the system's current state.
/// Retrying the same request unchanged will not help.
///
/// Default status: [`Status::Permanent`](crate::status::Status::Permanent).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Incorrect;
impl Category for Incorrect {}

/// The caller does not have permission to perform the operation.
///
/// The request was understood but authorization was denied. Retrying with the same
/// credentials will not help; the caller needs elevated privileges or a different
/// identity.
///
/// Default status: [`Status::Permanent`](crate::status::Status::Permanent).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Forbidden;
impl Category for Forbidden {}

/// The operation is not supported by this implementation.
///
/// The system understood the request but has no capability to fulfill it. This is
/// a permanent condition: the feature simply does not exist.
///
/// Default status: [`Status::Permanent`](crate::status::Status::Permanent).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unsupported;
impl Category for Unsupported {}

/// The requested resource does not exist.
///
/// The identifier or path is valid but points to nothing. Whether this is
/// permanent depends on context (e.g. a deleted record vs. a race with a
/// concurrent create), so there is no universal default status for this category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotFound;
impl Category for NotFound {}

/// The operation cannot be applied because it conflicts with existing state.
///
/// Typically a uniqueness violation, an optimistic-lock mismatch, or a
/// precondition failure. Retrying the same request unchanged will not resolve
/// the conflict; the caller must reconcile the state difference first.
///
/// Default status: [`Status::Permanent`](crate::status::Status::Permanent).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Conflict;
impl Category for Conflict {}

/// An internal error that is the system's fault, not the caller's.
///
/// The request was valid but something went wrong internally — a bug, a failed
/// invariant, an unexpected state. The caller cannot fix this by changing their
/// request.
///
/// Default status: [`Status::Permanent`](crate::status::Status::Permanent).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fault;
impl Category for Fault {}
