/// Sum `p[j] * q[column-j]` for `j=0..=column`; bit zero is the LSB.
pub fn diagonal_multiply(p: u64, q: u64, column: u32) -> u32 {
    assert!(column < u64::BITS, "column must be below 64");
    (p & (q.reverse_bits() >> (63 - column))).count_ones()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_example_and_upper_bits() {
        assert_eq!(diagonal_multiply(0b011101101, 0b101010110, 7), 3);
        assert_eq!(diagonal_multiply(1 << 32, 1, 32), 1);
        assert_eq!(diagonal_multiply(1 << 63, 1, 63), 1);
        assert_eq!(diagonal_multiply(u64::MAX, u64::MAX, 63), 64);
    }

    #[test]
    fn matches_direct_long_multiplication() {
        let mut state = 1234_u64;
        for _ in 0..1000 {
            let p = random(&mut state);
            let q = random(&mut state);
            for column in 0..64 {
                let expected = (0..=column)
                    .map(|j| ((p >> j) & 1) * ((q >> (column - j)) & 1))
                    .sum::<u64>();
                assert_eq!(u64::from(diagonal_multiply(p, q, column)), expected);
            }
        }
    }

    fn random(state: &mut u64) -> u64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *state
    }

    #[test]
    #[should_panic(expected = "column must be below 64")]
    fn rejects_an_invalid_column() {
        diagonal_multiply(1, 1, 64);
    }
}
