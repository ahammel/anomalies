use std::fmt;

use crate::{
    anomaly::{Anomaly, HasCategory, HasStatus},
    category::{self},
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

impl HasStatus for ServiceDown {
    fn status(&self) -> Status {
        Status::Temporary
    }
}

impl HasCategory for ServiceDown {
    fn category(&self) -> category::Category {
        category::Unavailable
    }
}

impl Anomaly for ServiceDown {}

#[derive(Debug)]
struct TimedOut;
impl fmt::Display for TimedOut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("timed out")
    }
}
impl std::error::Error for TimedOut {}

impl HasStatus for TimedOut {
    fn status(&self) -> Status {
        Status::Temporary
    }
}

impl HasCategory for TimedOut {
    fn category(&self) -> category::Category {
        category::Interrupted
    }
}

impl Anomaly for TimedOut {}

#[derive(Debug)]
struct RateLimited;
impl fmt::Display for RateLimited {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("rate limited")
    }
}
impl std::error::Error for RateLimited {}

impl HasStatus for RateLimited {
    fn status(&self) -> Status {
        Status::Temporary
    }
}

impl HasCategory for RateLimited {
    fn category(&self) -> category::Category {
        category::Busy
    }
}

impl Anomaly for RateLimited {}

#[derive(Debug)]
struct BadInput;
impl fmt::Display for BadInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("bad input")
    }
}
impl std::error::Error for BadInput {}

impl HasStatus for BadInput {
    fn status(&self) -> Status {
        Status::Permanent
    }
}

impl HasCategory for BadInput {
    fn category(&self) -> category::Category {
        category::Incorrect
    }
}

impl Anomaly for BadInput {}

#[derive(Debug)]
struct PermissionDenied;
impl fmt::Display for PermissionDenied {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("permission denied")
    }
}
impl std::error::Error for PermissionDenied {}

impl HasStatus for PermissionDenied {
    fn status(&self) -> Status {
        Status::Permanent
    }
}

impl HasCategory for PermissionDenied {
    fn category(&self) -> category::Category {
        category::Forbidden
    }
}

impl Anomaly for PermissionDenied {}

#[derive(Debug)]
struct NotImplemented;
impl fmt::Display for NotImplemented {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("not implemented")
    }
}
impl std::error::Error for NotImplemented {}

impl HasStatus for NotImplemented {
    fn status(&self) -> Status {
        Status::Permanent
    }
}

impl HasCategory for NotImplemented {
    fn category(&self) -> category::Category {
        category::Unsupported
    }
}

impl Anomaly for NotImplemented {}

#[derive(Debug)]
struct Missing;
impl fmt::Display for Missing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("missing")
    }
}
impl std::error::Error for Missing {}

impl HasStatus for Missing {
    fn status(&self) -> Status {
        Status::Permanent
    }
}

impl HasCategory for Missing {
    fn category(&self) -> category::Category {
        category::NotFound
    }
}

impl Anomaly for Missing {}

#[derive(Debug)]
struct AlreadyExists;
impl fmt::Display for AlreadyExists {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("already exists")
    }
}

impl std::error::Error for AlreadyExists {}

impl HasStatus for AlreadyExists {
    fn status(&self) -> Status {
        Status::Permanent
    }
}

impl HasCategory for AlreadyExists {
    fn category(&self) -> category::Category {
        category::Conflict
    }
}

impl Anomaly for AlreadyExists {}

#[derive(Debug)]
struct InternalError;
impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("internal error")
    }
}
impl std::error::Error for InternalError {}

impl HasStatus for InternalError {
    fn status(&self) -> Status {
        Status::Permanent
    }
}

impl HasCategory for InternalError {
    fn category(&self) -> category::Category {
        category::Fault
    }
}

impl Anomaly for InternalError {}

// Tests

#[test]
fn unavailable_is_temporary() {
    assert_eq!(
        (ServiceDown.status(), ServiceDown.category()),
        (Status::Temporary, category::Unavailable),
    );
}

#[test]
fn interrupted_delegates_status() {
    assert_eq!(
        (TimedOut.status(), TimedOut.category()),
        (Status::Temporary, category::Interrupted),
    );
}

#[test]
fn busy_is_temporary() {
    assert_eq!(
        (RateLimited.status(), RateLimited.category()),
        (Status::Temporary, category::Busy),
    );
}

#[test]
fn incorrect_is_permanent() {
    assert_eq!(
        (BadInput.status(), BadInput.category()),
        (Status::Permanent, category::Incorrect),
    );
}

#[test]
fn forbidden_is_permanent() {
    assert_eq!(
        (PermissionDenied.status(), PermissionDenied.category()),
        (Status::Permanent, category::Forbidden),
    );
}

#[test]
fn unsupported_is_permanent() {
    assert_eq!(
        (NotImplemented.status(), NotImplemented.category()),
        (Status::Permanent, category::Unsupported),
    );
}

#[test]
fn not_found_delegates_status() {
    assert_eq!(
        (Missing.status(), Missing.category()),
        (Status::Permanent, category::NotFound),
    );
}

#[test]
fn conflict_is_permanent() {
    assert_eq!(
        (AlreadyExists.status(), AlreadyExists.category()),
        (Status::Permanent, category::Conflict),
    );
}

#[test]
fn fault_is_permanent() {
    assert_eq!(
        (InternalError.status(), InternalError.category()),
        (Status::Permanent, category::Fault),
    );
}
