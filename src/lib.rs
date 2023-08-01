use std::time::{SystemTime, UNIX_EPOCH};

mod rng;
pub use rng::*;

// fhash
#[inline(always)]
pub fn hash64(mut hash: u64) -> u64 {
    hash = (hash ^ hash >> 32).wrapping_mul(15485907386658061715);
    hash = (hash ^ hash >> 32).wrapping_mul(15485907386658061715);
    // Add so that it won't be stuck at 0
    (hash.wrapping_add(1) ^ hash >> 32).wrapping_mul(15485907386658061715)
}
#[inline(always)]
pub fn hash64simple(hash: u64) -> u64 {
    (hash.wrapping_add(1) ^ hash >> 32).wrapping_mul(15485907386658061715)
}
#[inline(always)]
pub fn mix2_64(x: u64, y: u64) -> u64 {
    x.wrapping_add(y) ^ x.wrapping_add(123)
}

#[inline]
fn hash_time() -> u64 {
    let time = SystemTime::now();
    let time = time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    hash64(mix2_64(time.as_secs(), time.subsec_nanos() as u64))
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
        Self::with_seed(hash_time())
    }

    #[inline(always)]
    pub(crate) fn next_u64(&mut self) -> u64 {
        let value = self.seed;
        self.seed = hash64simple(self.seed);

        // We need to pass the last generated value so that float values already have
        // pre-generated u64 values and calculate their own value from them
        value
    }
    #[inline(always)]
    pub fn gen<T: RandomGeneratable>(&mut self) -> T {
        T::create_random(self)
    }
}
