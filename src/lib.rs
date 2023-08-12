mod gen;
pub use gen::*;

mod shuffle;
pub use shuffle::*;

mod bench_util;

mod cast;
pub(crate) use cast::*;

// fhash
#[inline(always)]
fn hash64(mut hash: u64) -> u64 {
    hash = (hash ^ hash >> 32).wrapping_mul(4997996261773036203);
    hash = (hash ^ hash >> 32).wrapping_mul(4997996261773036203);
    hash ^ hash >> 32
}
#[inline(always)]
fn mix2_64(x: u64, y: u64) -> u64 {
    x.wrapping_add(y) ^ y << 10
}

#[inline]
fn hash_time() -> u64 {
    let duration = std::time::SystemTime::UNIX_EPOCH
        .elapsed()
        .expect("Failed to get current time");
    mix2_64(duration.as_secs(), duration.subsec_nanos() as u64)
}

pub struct Rand {
    pub(crate) seed: u64,
}
impl Rand {
    #[inline(always)]
    pub fn with_seed(seed: u64) -> Self {
        Self { seed: hash64(seed) }
    }
    #[inline]
    pub fn new() -> Self {
        Self::with_seed(hash_time())
    }
    #[inline]
    pub fn rehash(&mut self) {
        self.seed = hash64(hash_time());
    }

    #[inline(always)]
    pub fn gen<T: RandomGeneratable>(&mut self) -> T {
        T::random(self)
    }

    // #[inline(always)]
    // pub fn u32(&mut self) -> u32 {
    //     let value = self.seed.wrapping_add(18091625850528980805);
    //     self.seed = value;
    //     (value.wrapping_mul(value ^ 6297118212550992339) >> 32) as u32
    // }
    // #[inline(always)]
    // pub fn u64(&mut self) -> u64 {
    //     let value = self.seed.wrapping_add(13210271939021150491);
    //     self.seed = value;
    //     ((value as u128).wrapping_mul((value ^ 1415764303421181697) as u128) >> 48) as u64
    // }
}
