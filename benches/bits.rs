use std::{hint::black_box, sync::LazyLock, time::Instant};

static TABLE8: LazyLock<[u8; 256]> =
    LazyLock::new(|| std::array::from_fn(|i| (i as u8).reverse_bits()));
static TABLE16: LazyLock<Box<[u16]>> =
    LazyLock::new(|| (0..=u16::MAX).map(u16::reverse_bits).collect());

fn parallel(mut value: u64) -> u64 {
    value = ((value >> 1) & 0x5555555555555555) | ((value & 0x5555555555555555) << 1);
    value = ((value >> 2) & 0x3333333333333333) | ((value & 0x3333333333333333) << 2);
    value = ((value >> 4) & 0x0f0f0f0f0f0f0f0f) | ((value & 0x0f0f0f0f0f0f0f0f) << 4);
    value.swap_bytes()
}

fn table8(value: u64, table: &[u8; 256]) -> u64 {
    let bytes = value.to_le_bytes();
    u64::from_be_bytes(bytes.map(|byte| table[usize::from(byte)]))
}

fn table16(value: u64, table: &[u16]) -> u64 {
    let mut result = 0;
    for index in 0..4 {
        result |=
            u64::from(table[((value >> (index * 16)) & 0xffff) as usize]) << (48 - index * 16);
    }
    result
}

fn measure(name: &str, inputs: &[u64], reverse: impl Fn(u64) -> u64) {
    const ROUNDS: usize = 128;
    let mut samples = Vec::new();
    for _ in 0..5 {
        let start = Instant::now();
        for _ in 0..ROUNDS {
            for &value in inputs {
                black_box(reverse(black_box(value)));
            }
        }
        samples.push(start.elapsed().as_secs_f64() * 1e9 / (inputs.len() * ROUNDS) as f64);
    }
    samples.sort_by(f64::total_cmp);
    println!("{name:24} {:8.2} ns/reversal (median of 5)", samples[2]);
}

fn main() {
    let mut state = 1234_u64;
    let inputs: Vec<_> = (0..16384)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        })
        .collect();
    let t8 = &*TABLE8;
    let t16 = &*TABLE16;
    for &value in &inputs {
        assert_eq!(parallel(value), value.reverse_bits());
        assert_eq!(table8(value, t8), value.reverse_bits());
        assert_eq!(table16(value, t16), value.reverse_bits());
    }
    measure("native reverse_bits", &inputs, u64::reverse_bits);
    measure("legacy parallel shifts", &inputs, parallel);
    measure("8-bit table (256 B)", &inputs, |v| table8(v, t8));
    measure("16-bit table (128 KiB)", &inputs, |v| table16(v, t16));

    let start = Instant::now();
    let iterations = 100;
    for _ in 0..iterations {
        let factors = primer::solver::solve(black_box(365295443)).unwrap();
        assert_eq!((factors.p, factors.q), (17209, 21227));
        black_box(factors);
    }
    println!(
        "original factor search   {:8.2} us/solve",
        start.elapsed().as_secs_f64() * 1e6 / f64::from(iterations)
    );
}
