extern crate hamming;

use std::os;

use hamming::dist;

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = os::args();
    assert!(args.len() == 3);
    let s1 = args[1].as_bytes();
    let s2 = args[2].as_bytes();

    println!("{}", dist(s1.as_slice(), s2.as_slice()));
}
