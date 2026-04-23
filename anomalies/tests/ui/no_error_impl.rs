use std::fmt::{Display, Formatter, Result};

use anomalies::anomaly::Anomaly;

#[derive(Anomaly, Debug)]
#[category(fault)]
struct MyError;

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "my error")
    }
}

fn main() {}
