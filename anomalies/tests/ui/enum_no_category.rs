use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use anomalies::anomaly::Anomaly;

#[derive(Anomaly, Debug)]
enum MyError {
    #[category(fault)]
    Legit,
    Missing,
}

impl Error for MyError {}
impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "my error")
    }
}

fn main() {}
