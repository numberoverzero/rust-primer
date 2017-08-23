extern crate primer;
use primer::bits::{KeepLeftBits,SetBitPos,SwapBits, diagonal_multiply};

fn main() {
    test_utils();
//    test_solver();
}

fn test_utils() {
    let x = 0b1100100010000100000100000010000000100000000100000000010000000000u64;
    //                                                              |-- 15
    //                                                              V
    let y = 0b1100100010000100000100000010000000100000000100001000010000000000u64;
    //              |-- 63                                          |-- 15
    //              V                                               V
    let z = 0b0100100010000100000100000010000000100000000100001000010000000000u64;

    assert_eq!(x.enable_bit(15), y);
    assert_eq!(y.disable_bit(63), z);

    assert_eq!(x.set_bit(15, true as u64), y);
    assert_eq!(y.set_bit(63, false as u64), z);


    let k = 0b1111101101u64.swap_bits();  // 1011011111 followed by 54 0s
    let left = 0b1011u64;
    assert_eq!(k.keep_left(3), left);


    let p = 0b011101101u64;
    let q = 0b101010110u64;
    let d = diagonal_multiply(p, q, 7);
    assert_eq!(d, 3);
}

//fn test_solver() {
//    let p = 21227u64;
//    let q = 17209u64;
//    let goal = p * q;
//    println!("goal {}", goal);
//    let (p_solve, q_solve) = solve(goal);
//    println!("p: ({}, {})  ||  q: ({}, {})", p, p_solve, q, q_solve);
//
//}