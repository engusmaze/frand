use rand_core::{impls, Error, RngCore, SeedableRng};

use crate::{Rand, ThreadRand};

impl RngCore for Rand {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.gen::<u32>()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.gen::<u64>()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest);
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
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

impl RngCore for ThreadRand {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.get_rng().gen::<u32>()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.get_rng().gen::<u64>()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.get_rng().fill_bytes(dest)
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.get_rng().try_fill_bytes(dest)
    }
}
