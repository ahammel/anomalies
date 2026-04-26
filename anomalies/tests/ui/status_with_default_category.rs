use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use anomalies::anomaly::Anomaly;

#[derive(Anomaly, Debug)]
#[category(fault)]
#[status(temporary)]
struct Falling;

impl Error for Falling {}
impl Display for Falling {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "start any way you can")
    }
}

fn main() {}
