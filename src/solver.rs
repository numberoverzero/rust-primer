use crate::bits::diagonal_multiply;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Factors {
    pub p: u64,
    pub q: u64,
}

pub fn solve(target: u64) -> Option<Factors> {
    if target < 4 {
        return None;
    }
    if target % 2 == 0 {
        return Some(Factors {
            p: 2,
            q: target / 2,
        });
    }
    if target < 9 {
        return None;
    }

    // Odd factors >= 3 => p,q <= target/3.
    let factor_limit = target / 3;
    let last_column = factor_limit.ilog2();
    let mut p = 1_u64;
    let mut q = 1_u64;
    let mut column = 1_u32;
    let mut next = [0_u8; 64];
    let mut carry = [0_u32; 64];

    loop {
        let index = column as usize;
        let mask = 1_u64 << column;
        if next[index] == 2 {
            p &= !mask;
            q &= !mask;
            next[index] = 0;
            if column == 1 {
                return None;
            }
            column -= 1;
            continue;
        }

        // Remove the endpoints p_i*q_0 and p_0*q_i (both LSBs are one).
        // At column one there are no interior terms.
        let inner = if column == 1 {
            0
        } else {
            diagonal_multiply(p >> 1, q >> 1, column - 2)
        };
        let sum = inner + carry[index - 1];
        let target_bit = ((target >> column) & 1) as u32;
        let parity = (sum ^ target_bit) & 1;
        let p_bit = u32::from(next[index]);
        let q_bit = p_bit ^ parity;
        next[index] += 1;
        p = (p & !mask) | (u64::from(p_bit) << column);
        q = (q & !mask) | (u64::from(q_bit) << column);
        carry[index] = (sum + p_bit + q_bit) >> 1;

        let product_order = compare_product(p, q, target);
        if product_order == Ordering::Equal && p > 1 && q > 1 {
            return Some(Factors {
                p: p.min(q),
                q: p.max(q),
            });
        }
        if product_order == Ordering::Less
            && p <= factor_limit
            && q <= factor_limit
            && column < last_column
        {
            column += 1;
        }
    }
}

fn compare_product(p: u64, q: u64, target: u64) -> Ordering {
    // Widen before multiplying: wrapping corrupts equality and pruning.
    (u128::from(p) * u128::from(q)).cmp(&u128::from(target))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_example() {
        assert_eq!(solve(365295443), Some(Factors { p: 17209, q: 21227 }));
    }

    #[test]
    fn exhaustive_small_inputs_against_trial_division() {
        for target in 0..10000 {
            let divisor = (2..)
                .take_while(|d| d * d <= target)
                .find(|d| target % d == 0);
            match (solve(target), divisor) {
                (Some(factors), Some(_)) => {
                    assert!(factors.p >= 2 && factors.p <= factors.q);
                    assert_eq!(
                        u128::from(factors.p) * u128::from(factors.q),
                        u128::from(target)
                    );
                }
                (None, None) => {}
                result => panic!("wrong result for {target}: {result:?}"),
            }
        }
    }

    #[test]
    fn does_not_accept_a_wrapped_product() {
        let target = (1_u64 << 63) + 3;
        assert_eq!(
            compare_product((1_u64 << 63) + 1, 3, target),
            Ordering::Greater
        );
        assert_eq!(compare_product(11, target / 11, target), Ordering::Equal);
        assert_eq!(
            compare_product(u64::MAX, u64::MAX, u64::MAX),
            Ordering::Greater
        );
    }

    #[test]
    #[ignore = "slow; run with --release --ignored"]
    fn full_width_odd_overflow_regression() {
        let target = (1_u64 << 63) + 3;
        let factors = solve(target).expect("target is divisible by 11");
        assert_eq!(
            u128::from(factors.p) * u128::from(factors.q),
            u128::from(target)
        );
    }

    #[test]
    fn handles_full_width_even_input() {
        assert_eq!(
            solve(u64::MAX - 1),
            Some(Factors {
                p: 2,
                q: (u64::MAX - 1) / 2
            })
        );
    }
}
