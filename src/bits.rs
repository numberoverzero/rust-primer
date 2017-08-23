pub trait SwapBits {
    fn swap_bits(self) -> u64;
}

pub trait SetBitPos {
    fn enable_bit(self, pos: usize) -> u64;
    fn disable_bit(self, pos: usize) -> u64;
    fn set_bit(self, pos: usize, enable: u64) -> u64;
}

pub trait KeepLeftBits {
    fn keep_left(self, pos: usize) -> u64;
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
    fn enable_bit(self, pos: usize) -> u64 { self | (1u64 << pos) }
    fn disable_bit(self, pos: usize) -> u64 { self & !(1u64 << pos) }
    fn set_bit(self, pos: usize, enable: u64) -> u64 {
        // need overflow for -bool -> max val u64
        // https://graphics.stanford.edu/~seander/\
        //     bithacks.html#ConditionalSetOrClearBitsWithoutBranching
        let mask = 1u64 << pos;
        (self & !mask) | enable << pos
    }
}

impl KeepLeftBits for u64 {
    fn keep_left(self, pos: usize) -> u64 { self >> (63 - pos) }
}

pub fn diagonal_multiply(p: u64, q: u64, pos: usize) -> u8 {
    let x = q.swap_bits().keep_left(pos);
    (p & x).count_ones() as u8
}