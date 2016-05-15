#![feature(iter_arith)]

extern crate bit_vec;

use bit_vec::BitVec;

pub fn dist(s1: &[u8], s2: &[u8]) -> u64 {
   let xor: Vec<u8> = s1
        .iter()
        .zip(s2.iter())
        .map(|(&x,&y)| x ^ y)
        .collect();

    BitVec::from_bytes(xor.as_slice())
        .iter()
        .map(|x| x as u64)
        .sum()
}

#[cfg(test)]
mod test {
    use super::dist;

    #[test]
    fn dist_test_1() {
        let input1 = b"YELLOW SUBMARINE";
        let input2 = b"YELLOW SUBMACINE";
        let expected = 2;
        assert_eq!(dist(input1, input2), expected);
    }
}
