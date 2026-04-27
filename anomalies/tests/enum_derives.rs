use std::fmt::{Display, Formatter, Result};

use anomalies::anomaly::{Anomaly, HasCategory, HasStatus};
use anomalies::category::{Busy, Fault, Forbidden, NotFound};
use anomalies::status::Status;

// ── Plain enum ────────────────────────────────────────────────────────────────

#[derive(Anomaly, Debug)]
enum RockNSoul {
    /// not_found + explicit #[status(permanent)]
    #[category(not_found)]
    #[status(permanent)]
    ShesGone(String),

    /// fault — inherits default Permanent
    #[category(fault)]
    Falling { message: String },

    /// busy — inherits default Temporary
    #[category(busy)]
    WaitForMe,

    /// forbidden — inherits default Permanent
    #[category(forbidden)]
    ICantGoForThat(String),
}

impl std::error::Error for RockNSoul {}
impl Display for RockNSoul {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::ShesGone(s) => write!(f, "she's gone: {s}"),
            Self::Falling { message } => write!(f, "falling: {message}"),
            Self::WaitForMe => write!(f, "wait for me"),
            Self::ICantGoForThat(s) => write!(f, "i can't go for that: {s}"),
        }
    }
}

#[test]
fn shes_gone_has_category_not_found() {
    assert_eq!(RockNSoul::ShesGone("x".into()).category(), NotFound)
}

#[test]
fn shes_gone_has_status_permanent_from_attribute() {
    assert_eq!(RockNSoul::ShesGone("x".into()).status(), Status::Permanent)
}

#[test]
fn falling_has_category_fault() {
    assert_eq!(
        RockNSoul::Falling {
            message: "start any way you can".into()
        }
        .category(),
        Fault
    )
}

#[test]
fn falling_has_status_permanent_by_default() {
    assert_eq!(
        RockNSoul::Falling {
            message: "start any way you can".into()
        }
        .status(),
        Status::Permanent
    )
}

#[test]
fn wait_for_me_has_category_busy() {
    assert_eq!(RockNSoul::WaitForMe.category(), Busy)
}

#[test]
fn wait_for_me_has_status_temporary_by_default() {
    assert_eq!(RockNSoul::WaitForMe.status(), Status::Temporary)
}

#[test]
fn i_cant_go_for_that_has_category_forbidden() {
    assert_eq!(
        RockNSoul::ICantGoForThat("no can do".into()).category(),
        Forbidden
    )
}

#[test]
fn i_cant_go_for_that_has_status_permanent_by_default() {
    assert_eq!(
        RockNSoul::ICantGoForThat("no can do".into()).status(),
        Status::Permanent
    )
}

// ── thiserror compatibility ───────────────────────────────────────────────────
//
// Demonstrates that thiserror's #[error(...)] and anomalies' #[category(...)] /
// #[status(...)] attributes coexist on the same enum variants without conflict.

#[derive(Anomaly, thiserror::Error, Debug)]
enum VoicesCarry {
    #[category(not_found)]
    #[status(permanent)]
    #[error("she's gone: {0}")]
    ShesGone(String),

    #[category(fault)]
    #[error("start any way you can")]
    Falling,

    #[category(busy)]
    #[error("midnight hour almost over")]
    WaitForMe,
}

#[test]
fn voices_carry_shes_gone_has_category_not_found() {
    assert_eq!(VoicesCarry::ShesGone("x".into()).category(), NotFound)
}

#[test]
fn voices_carry_shes_gone_has_status_permanent() {
    assert_eq!(
        VoicesCarry::ShesGone("x".into()).status(),
        Status::Permanent
    )
}

#[test]
fn voices_carry_falling_has_category_fault() {
    assert_eq!(VoicesCarry::Falling.category(), Fault)
}

#[test]
fn voices_carry_falling_has_status_permanent() {
    assert_eq!(VoicesCarry::Falling.status(), Status::Permanent)
}

#[test]
fn voices_carry_wait_for_me_has_category_busy() {
    assert_eq!(VoicesCarry::WaitForMe.category(), Busy)
}

#[test]
fn voices_carry_wait_for_me_has_status_temporary() {
    assert_eq!(VoicesCarry::WaitForMe.status(), Status::Temporary)
}

#[test]
fn thiserror_display_is_unaffected() {
    assert_eq!(
        VoicesCarry::ShesGone("Record Row".into()).to_string(),
        "she's gone: Record Row"
    )
}
