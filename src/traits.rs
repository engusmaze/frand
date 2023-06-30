pub trait RNG {
    fn generate_u64(&mut self) -> u64;
    fn gen<T: RandomGeneratable>(&mut self) -> T;
}
pub trait RandomGeneratable {
    fn create_random(rng: &mut impl RNG) -> Self;
}

// Unsigned integers
impl RandomGeneratable for u8 {
    #[inline(always)]
    fn create_random(rng: &mut impl RNG) -> u8 {
        rng.generate_u64() as u8
    }
}
impl RandomGeneratable for u16 {
    #[inline(always)]
    fn create_random(rng: &mut impl RNG) -> u16 {
        rng.generate_u64() as u16
    }
}
impl RandomGeneratable for u32 {
    #[inline(always)]
    fn create_random(rng: &mut impl RNG) -> u32 {
        rng.generate_u64() as u32
    }
}
impl RandomGeneratable for u64 {
    #[inline(always)]
    fn create_random(rng: &mut impl RNG) -> u64 {
        rng.generate_u64()
    }
}
impl RandomGeneratable for u128 {
    #[inline(always)]
    fn create_random(rng: &mut impl RNG) -> u128 {
        let bytes = [rng.generate_u64(), rng.generate_u64()];
        unsafe { *(&bytes as *const _ as *const u128) }
        // (rng.next_u64() as u128) << 64 | rng.next_u64()
    }
}

// Signed integers
impl RandomGeneratable for i8 {
    #[inline(always)]
    fn create_random(rng: &mut impl RNG) -> i8 {
        rng.generate_u64() as i8
    }
}
impl RandomGeneratable for i16 {
    #[inline(always)]
    fn create_random(rng: &mut impl RNG) -> i16 {
        rng.generate_u64() as i16
    }
}
impl RandomGeneratable for i32 {
    #[inline(always)]
    fn create_random(rng: &mut impl RNG) -> i32 {
        rng.generate_u64() as i32
    }
}
impl RandomGeneratable for i64 {
    #[inline(always)]
    fn create_random(rng: &mut impl RNG) -> i64 {
        rng.generate_u64() as i64
    }
}
impl RandomGeneratable for i128 {
    #[inline(always)]
    fn create_random(rng: &mut impl RNG) -> i128 {
        let bytes = [rng.generate_u64(), rng.generate_u64()];
        unsafe { *(&bytes as *const _ as *const i128) }
    }
}

impl RandomGeneratable for f32 {
    #[inline(always)]
    fn create_random(rng: &mut impl RNG) -> f32 {
        // Shift to mentisa and make the exponent so that the value range of mentisa is [1; 2)
        // Convert to float and subtract 1.0 so that we get [0; 1)
        unsafe { *(&(rng.generate_u64() as u32 >> 9 | 0x3f800000) as *const _ as *const f32) - 1.0 }
    }
}
impl RandomGeneratable for f64 {
    #[inline(always)]
    fn create_random(rng: &mut impl RNG) -> f64 {
        unsafe {
            *(&(rng.generate_u64() >> 12 | 0x3ff0000000000000) as *const _ as *const f64) - 1.0
        }
    }
}
