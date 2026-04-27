use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use anomalies::anomaly::Anomaly;

#[derive(Debug)]
struct NotAnAnomaly(String);

#[derive(Anomaly, Debug)]
enum MyError {
    #[category(fault)]
    Legit,
    #[anomaly(transparent)]
    Bad(NotAnAnomaly),
}

impl Error for MyError {}
impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "my error")
    }
}

fn main() {}
