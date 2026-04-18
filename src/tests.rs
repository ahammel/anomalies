use std::fmt;

use crate::{
    anomaly::{self, Anomaly},
    category,
    status::Status,
};

// Minimal concrete error types for each category.

#[derive(Debug)]
struct ServiceDown;
impl fmt::Display for ServiceDown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("service down")
    }
}
impl std::error::Error for ServiceDown {}
impl anomaly::Unavailable for ServiceDown {}

#[derive(Debug)]
struct TimedOut;
impl fmt::Display for TimedOut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("timed out")
    }
}
impl std::error::Error for TimedOut {}
impl anomaly::Interrupted for TimedOut {
    fn status(&self) -> Status {
        Status::Temporary
    }
}

#[derive(Debug)]
struct RateLimited;
impl fmt::Display for RateLimited {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("rate limited")
    }
}
impl std::error::Error for RateLimited {}
impl anomaly::Busy for RateLimited {}

#[derive(Debug)]
struct BadInput;
impl fmt::Display for BadInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("bad input")
    }
}
impl std::error::Error for BadInput {}
impl anomaly::Incorrect for BadInput {}

#[derive(Debug)]
struct PermissionDenied;
impl fmt::Display for PermissionDenied {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("permission denied")
    }
}
impl std::error::Error for PermissionDenied {}
impl anomaly::Forbidden for PermissionDenied {}

#[derive(Debug)]
struct NotImplemented;
impl fmt::Display for NotImplemented {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("not implemented")
    }
}
impl std::error::Error for NotImplemented {}
impl anomaly::Unsupported for NotImplemented {}

#[derive(Debug)]
struct Missing;
impl fmt::Display for Missing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("missing")
    }
}
impl std::error::Error for Missing {}
impl anomaly::NotFound for Missing {
    fn status(&self) -> Status {
        Status::Permanent
    }
}

#[derive(Debug)]
struct AlreadyExists;
impl fmt::Display for AlreadyExists {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("already exists")
    }
}
impl std::error::Error for AlreadyExists {}
impl anomaly::Conflict for AlreadyExists {}

#[derive(Debug)]
struct InternalError;
impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("internal error")
    }
}
impl std::error::Error for InternalError {}
impl anomaly::Fault for InternalError {}

// Tests

#[test]
fn unavailable_is_temporary() {
    assert_eq!(
        Anomaly::<category::Unavailable>::status(&ServiceDown),
        Status::Temporary,
    );
}

#[test]
fn interrupted_delegates_status() {
    assert_eq!(
        Anomaly::<category::Interrupted>::status(&TimedOut),
        Status::Temporary,
    );
}

#[test]
fn busy_is_temporary() {
    assert_eq!(
        Anomaly::<category::Busy>::status(&RateLimited),
        Status::Temporary,
    );
}

#[test]
fn incorrect_is_permanent() {
    assert_eq!(
        Anomaly::<category::Incorrect>::status(&BadInput),
        Status::Permanent,
    );
}

#[test]
fn forbidden_is_permanent() {
    assert_eq!(
        Anomaly::<category::Forbidden>::status(&PermissionDenied),
        Status::Permanent,
    );
}

#[test]
fn unsupported_is_permanent() {
    assert_eq!(
        Anomaly::<category::Unsupported>::status(&NotImplemented),
        Status::Permanent,
    );
}

#[test]
fn not_found_delegates_status() {
    assert_eq!(
        Anomaly::<category::NotFound>::status(&Missing),
        Status::Permanent,
    );
}

#[test]
fn conflict_is_permanent() {
    assert_eq!(
        Anomaly::<category::Conflict>::status(&AlreadyExists),
        Status::Permanent,
    );
}

#[test]
fn fault_is_permanent() {
    assert_eq!(
        Anomaly::<category::Fault>::status(&InternalError),
        Status::Permanent,
    );
}
