use rand_core::{impls, RngCore, SeedableRng};

use crate::Rand;

impl RngCore for Rand {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.random::<u32>()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.random::<u64>()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest);
    }
}

impl SeedableRng for Rand {
    type Seed = [u8; 8];

    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        Self::with_seed(u64::from_be_bytes(seed))
    }

    #[inline]
    fn seed_from_u64(state: u64) -> Self {
        Self::with_seed(state)
    }
}

#[cfg(feature = "std")]
impl RngCore for crate::ThreadRand {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.get_rng().random::<u32>()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.get_rng().random::<u64>()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.get_rng().fill_bytes(dest)
    }
}
