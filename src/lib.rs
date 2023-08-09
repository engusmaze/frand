mod gen;
pub use gen::*;

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
    let duration = std::time::SystemTime::UNIX_EPOCH.elapsed().expect(
        "Getting elapsed time since UNIX_EPOCH. If this fails, we've somehow violated causality",
    );
    mix2_64(duration.as_secs(), duration.subsec_nanos() as u64)
}

pub struct Rand {
    seed: u64,
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
    pub(crate) fn next_u64(&mut self) -> u64 {
        let mut value = self.seed.wrapping_add(16580203494071533729);
        self.seed = value;
        value = value.wrapping_mul(7744658036413730167);
        value ^ value >> 32
    }
    #[inline(always)]
    pub fn gen<T: RandomGeneratable>(&mut self) -> T {
        T::create_random(self)
    }
}
