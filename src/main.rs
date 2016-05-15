extern crate hamming;

use hamming::dist;

#[allow(dead_code)]
fn main() {
    let mut args = std::env::args();
    assert!(args.len() == 3);
    args.next().unwrap();
    let s1 = args.next().unwrap();
    let s2 = args.next().unwrap();

    println!("{}", dist(s1.as_bytes(), s2.as_bytes()));
}
