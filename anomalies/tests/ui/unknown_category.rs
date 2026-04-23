use anomalies::anomaly::Anomaly;

#[derive(Anomaly, Debug)]
#[category(bogus)]
struct MyError;

fn main() {}
