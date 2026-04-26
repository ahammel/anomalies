use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use anomalies::anomaly::{Anomaly, HasCategory, HasStatus};
use anomalies::category::{
    Busy, Conflict, Fault, Forbidden, Incorrect, Interrupted, NotFound, Unavailable, Unsupported,
};
use anomalies::status::Status;

#[derive(Anomaly, Debug)]
#[category(unavailable)]
struct OutOfTouch();
impl Error for OutOfTouch {}
impl Display for OutOfTouch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", "You're out of touch / I'm out of time")
    }
}

#[derive(Anomaly, Debug)]
#[category(interrupted)]
struct ItDoesntMatterAnymore();
impl HasStatus for ItDoesntMatterAnymore {
    fn status(&self) -> Status {
        Status::Persistent
    }
}
impl Error for ItDoesntMatterAnymore {}
impl Display for ItDoesntMatterAnymore {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            "You know it doesn't matter, yes, it doesn't matter anymore, oh-oh"
        )
    }
}

#[derive(Anomaly, Debug)]
#[category(busy)]
struct WaitForMe();
impl Error for WaitForMe {}
impl Display for WaitForMe {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", "Midnight hour almost over")
    }
}

#[derive(Anomaly, Debug)]
#[category(incorrect)]
struct YoullNeverLearn();
impl Error for YoullNeverLearn {}
impl Display for YoullNeverLearn {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", "'Cause I know you can't change")
    }
}

#[derive(Anomaly, Debug)]
#[category(forbidden)]
struct ICantGoForThat();
impl Error for ICantGoForThat {}
impl Display for ICantGoForThat {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", "No can do")
    }
}

#[derive(Anomaly, Debug)]
#[category(unsupported)]
struct YourImagination();
impl Error for YourImagination {}
impl Display for YourImagination {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", "I remember when I used to be the jealous kind")
    }
}

#[derive(Anomaly, Debug)]
#[category(not_found)]
struct ShesGone();
impl HasStatus for ShesGone {
    fn status(&self) -> Status {
        Status::Persistent
    }
}
impl Error for ShesGone {}
impl Display for ShesGone {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", "I better learn how to face it")
    }
}

#[derive(Anomaly, Debug)]
#[category(not_found)]
#[status(permanent)]
struct SoCloseYetSoFarAway();
impl Error for SoCloseYetSoFarAway {}
impl Display for SoCloseYetSoFarAway {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", "I guess I'll never know")
    }
}

#[derive(Anomaly, Debug)]
#[category(interrupted)]
#[status(temporary)]
struct StopLovinMe();
impl Error for StopLovinMe {}
impl Display for StopLovinMe {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", "I can't make it without you")
    }
}

#[derive(Anomaly, Debug)]
#[category(conflict)]
struct GiveItUp();
impl Error for GiveItUp {}
impl Display for GiveItUp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", "Old habits")
    }
}

#[derive(Anomaly, Debug)]
#[category(fault)]
struct Falling();
impl Error for Falling {}
impl Display for Falling {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", "Start any way you can")
    }
}

#[test]
fn out_of_touch_has_category_unavailable() {
    assert_eq!((OutOfTouch {}).category(), Unavailable)
}

#[test]
fn out_of_touch_has_status_temporary() {
    assert_eq!(OutOfTouch().status(), Status::Temporary)
}

#[test]
fn it_doesnt_matter_anymore_has_category_interrupted() {
    assert_eq!(ItDoesntMatterAnymore().category(), Interrupted)
}

#[test]
fn it_doesnt_matter_anymore_has_status_set_by_implementation() {
    assert_eq!(ItDoesntMatterAnymore().status(), Status::Persistent)
}

#[test]
fn wait_for_me_has_category_busy() {
    assert_eq!(WaitForMe().category(), Busy)
}

#[test]
fn wait_for_me_has_status_temporary() {
    assert_eq!(WaitForMe().status(), Status::Temporary)
}

#[test]
fn youll_never_learn_has_category_incorrect() {
    assert_eq!(YoullNeverLearn().category(), Incorrect)
}

#[test]
fn youll_never_learn_has_status_permanent() {
    assert_eq!(YoullNeverLearn().status(), Status::Permanent)
}

#[test]
fn i_cant_go_for_that_has_category_forbidden() {
    assert_eq!(ICantGoForThat().category(), Forbidden)
}

#[test]
fn i_cant_go_for_that_has_status_permanent() {
    assert_eq!(ICantGoForThat().status(), Status::Permanent)
}

#[test]
fn your_imagination_has_category_unsupported() {
    assert_eq!(YourImagination().category(), Unsupported)
}

#[test]
fn your_imagination_has_status_permanent() {
    assert_eq!(YourImagination().status(), Status::Permanent)
}

#[test]
fn shes_gone_has_category_not_found() {
    assert_eq!(ShesGone().category(), NotFound)
}

#[test]
fn shes_gone_has_status_set_by_implementer() {
    assert_eq!(ShesGone().status(), Status::Persistent)
}

#[test]
fn so_close_yet_so_far_away_has_category_not_found() {
    assert_eq!(SoCloseYetSoFarAway().category(), NotFound)
}

#[test]
fn so_close_yet_so_far_away_has_status_permanent_from_attribute() {
    assert_eq!(SoCloseYetSoFarAway().status(), Status::Permanent)
}

#[test]
fn stop_lovin_me_has_category_interrupted() {
    assert_eq!(StopLovinMe().category(), Interrupted)
}

#[test]
fn stop_lovin_me_has_status_temporary_from_attribute() {
    assert_eq!(StopLovinMe().status(), Status::Temporary)
}

#[test]
fn give_it_up_has_category_conflict() {
    assert_eq!(GiveItUp().category(), Conflict)
}

#[test]
fn give_it_up_has_status_permanent() {
    assert_eq!(GiveItUp().status(), Status::Permanent)
}

#[test]
fn falling_has_category_fault() {
    assert_eq!(Falling().category(), Fault)
}

#[test]
fn falling_has_status_permanent() {
    assert_eq!(Falling().status(), Status::Permanent)
}
