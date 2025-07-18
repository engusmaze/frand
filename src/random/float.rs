use core::mem::transmute;

use crate::*;

const F32_VALUE: u32 = !0 >> 9;
const F32_MENTISA: u32 = 0x3f800000;

const F64_VALUE: u64 = !0 >> 12;
const F64_MENTISA: u64 = 0x3ff0000000000000;

// Shift to mentisa and make the exponent so that the value range of mentisa is [1; 2)
// Convert to float and subtract 1.0 so that we get [0; 1)
// See https://mina86.com/2016/random-reals/
impl Random for f32 {
    #[inline(always)]
    fn random(rng: &mut Rand) -> f32 {
        f32::from_bits(u32::random(rng) >> 9 | 0x3f800000) + -1.0
    }
}
impl Random for f64 {
    #[inline(always)]
    fn random(rng: &mut Rand) -> f64 {
        f64::from_bits(u64::random(rng) >> 12 | 0x3ff0000000000000) + -1.0
    }
}

impl Random for [f32; 2] {
    #[inline(always)]
    fn random(rng: &mut Rand) -> Self {
        const F32_VALUE_X2: u64 = unsafe { transmute([F32_VALUE; 2]) };
        const F32_MENTISA_X2: u64 = unsafe { transmute([F32_MENTISA; 2]) };
        let vec: [f32; 2] = unsafe { transmute(u64::random(rng) & F32_VALUE_X2 | F32_MENTISA_X2) };

        [vec[0] + -1.0, vec[1] + -1.0]
    }
}
impl Random for [f32; 3] {
    #[inline(always)]
    fn random(rng: &mut Rand) -> Self {
        unsafe { transmute((<[f32; 2]>::random(rng), f32::random(rng))) }
    }
}
impl Random for [f32; 4] {
    #[inline(always)]
    fn random(rng: &mut Rand) -> Self {
        const F32_VALUE_X2: u128 = unsafe { transmute([F32_VALUE; 4]) };
        const F32_MENTISA_X2: u128 = unsafe { transmute([F32_MENTISA; 4]) };
        let vec: [f32; 4] = unsafe { transmute(u128::random(rng) & F32_VALUE_X2 | F32_MENTISA_X2) };

        [vec[0] + -1.0, vec[1] + -1.0, vec[2] + -1.0, vec[3] + -1.0]
    }
}

impl Random for [f64; 2] {
    #[inline(always)]
    fn random(rng: &mut Rand) -> Self {
        const F64_VALUE_X2: u128 = unsafe { transmute([F64_VALUE; 2]) };
        const F64_MENTISA_X2: u128 = unsafe { transmute([F64_MENTISA; 2]) };
        let vec: [f64; 2] = unsafe { transmute(u128::random(rng) & F64_VALUE_X2 | F64_MENTISA_X2) };

        [vec[0] + -1.0, vec[1] + -1.0]
    }
}

#[cfg(feature = "glam")]
mod glam_impl {
    use crate::{Rand, Random};
    pub use glam::{Vec2, Vec3, Vec3A, Vec4};

    impl Random for Vec2 {
        #[inline(always)]
        fn random(rng: &mut Rand) -> Self {
            Vec2::from_array(rng.random())
        }
    }
    impl Random for Vec3 {
        #[inline(always)]
        fn random(rng: &mut Rand) -> Self {
            Vec3::from_array(rng.random())
        }
    }
    impl Random for Vec3A {
        #[inline(always)]
        fn random(rng: &mut Rand) -> Self {
            Vec3A::from_array(rng.random())
        }
    }
    impl Random for Vec4 {
        #[inline(always)]
        fn random(rng: &mut Rand) -> Self {
            Vec4::from_array(rng.random())
        }
    }
}
