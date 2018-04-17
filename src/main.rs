mod binary;

use binary::*;

fn main() {
    println!("{:e}", f64::build("000000000000000000000000000000000001").unwrap());
}
