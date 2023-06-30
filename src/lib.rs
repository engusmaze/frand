use std::time::{SystemTime, UNIX_EPOCH};

mod traits;
pub use traits::*;

// Shifter hash
#[inline(always)]
pub fn hash64(mut hash: u64) -> u64 {
    hash = (hash ^ hash >> 32).wrapping_mul(15485907386658061715);
    hash = (hash ^ hash >> 32).wrapping_mul(15485907386658061715);
    hash.wrapping_add(1) // So that it won't be stuck at 0
     ^ hash >> 32
}
#[inline(always)]
pub fn hash64simple(hash: u64) -> u64 {
    (hash.wrapping_add(1) ^ hash >> 32).wrapping_mul(15485907386658061715)
}
#[inline(always)]
pub fn mix2_64(x: u64, y: u64) -> u64 {
    x ^ (y << 15).wrapping_add(y)
}

pub struct Rand {
    seed: u64,
}
impl Rand {
    #[inline(always)]
    pub fn with_seed(seed: u64) -> Self {
        Self { seed }
    }
    #[inline]
    pub fn new() -> Self {
        let start = SystemTime::now();
        let time = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        Self::with_seed(mix2_64(time.as_secs(), time.subsec_nanos() as u64))
    }

    #[inline(always)]
    pub fn gen<T: RandomGeneratable>(&mut self) -> T {
        T::create_random(self)
    }
}
impl RNG for Rand {
    #[inline(always)]
    fn generate_u64(&mut self) -> u64 {
        self.seed = hash64simple(self.seed);
        self.seed
    }
    #[inline(always)]
    fn gen<T: RandomGeneratable>(&mut self) -> T {
        self.gen()
    }
}

pub struct QualityRand {
    seed: u64,
}
impl QualityRand {
    #[inline(always)]
    pub fn with_seed(seed: u64) -> Self {
        Self { seed }
    }
    #[inline]
    pub fn new() -> Self {
        let start = SystemTime::now();
        let time = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        Self::with_seed(mix2_64(time.as_secs(), time.subsec_nanos() as u64))
    }

    #[inline(always)]
    pub fn gen<T: RandomGeneratable>(&mut self) -> T {
        T::create_random(self)
    }
}
impl RNG for QualityRand {
    #[inline(always)]
    fn generate_u64(&mut self) -> u64 {
        self.seed = hash64(self.seed);
        self.seed
    }
    #[inline(always)]
    fn gen<T: RandomGeneratable>(&mut self) -> T {
        self.gen()
    }
}
