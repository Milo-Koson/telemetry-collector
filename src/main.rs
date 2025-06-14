mod metrics;
mod types;
use crate::metrics::{gather_all_metrics};
fn main() {
    let report = gather_all_metrics();
    println!("{:#?}", report);
}
