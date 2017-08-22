pub trait SwapBits {
    fn swap_bits(self) -> u64;
}

pub trait SetBitPos {
    fn set_bit(self, pos: u8) -> u64;
    fn clear_bit(self, pos: u8) -> u64;
}

pub trait KeepLeftBits {
    fn keep_left(self, pos: u8) -> u64;
}

impl SwapBits for u64 {
    fn swap_bits(self) -> u64 {
        // https://github.com/EugeneGonzalez/bit_reverse/blob\
        //     /70cc4c4cd28b2e4b1d916d9d1a951e461cadfe8e/src/parallel.rs
        // Swap odd and even bits
        let mut v = self;
        v = ((v >> 1) & (0x5555555555555555u64)) | ((v & (0x5555555555555555u64)) << 1);
        // Swap consecutive pairs
        v = ((v >> 2) & (0x3333333333333333u64)) | ((v & (0x3333333333333333u64)) << 2);
        // Swap nibbles
        v = ((v >> 4) & (0x0F0F0F0F0F0F0F0Fu64)) | ((v & (0x0F0F0F0F0F0F0F0Fu64)) << 4);

        v.swap_bytes()
    }
}

impl SetBitPos for u64 {
    fn set_bit(self, pos: u8) -> u64 { self | (0b1u64 << pos) }
    fn clear_bit(self, pos: u8) -> u64 { self & !(0b1u64 << pos) }
}

impl KeepLeftBits for u64 {
    fn keep_left(self, pos: u8) -> u64 { self >> (63 - pos) }
}

fn diagonal_multiply(p: u64, q: u64, pos: u8) -> u8 {
    let x = p & q.swap_bits().keep_left(pos);
    x.count_ones() as u8
}

fn main() {
    let x = 0b1100100010000100000100000010000000100000000100000000010000000000u64;
    //                                                              |-- 15
    //                                                              V
    let y = 0b1100100010000100000100000010000000100000000100001000010000000000u64;
    //              |-- 63                                          |-- 15
    //              V                                               V
    let z = 0b0100100010000100000100000010000000100000000100001000010000000000u64;

    assert_eq!(x.set_bit(15), y);
    assert_eq!(y.clear_bit(63), z);


    let k = 0b1111101101u64.swap_bits();  // 1011011111 followed by 54 0s
    let left = 0b1011u64;
    assert_eq!(k.keep_left(3), left);


    let p = 0b011101101u64;
    let q = 0b101010110u64;
    let d = diagonal_multiply(p, q, 7);
    assert_eq!(d, 3);
}
