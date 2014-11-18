use std::collections::bitv::from_bytes;
use std::iter::AdditiveIterator;

pub fn dist(s1: &[u8], s2: &[u8]) -> uint {
   let xor: Vec<u8> = s1
        .iter()
        .zip(s2.iter())
        .map(|(&x,&y)| x ^ y)
        .collect();

    from_bytes(xor.as_slice())
        .iter()
        .map(|x| x as uint)
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
