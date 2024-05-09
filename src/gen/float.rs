use super::*;

// Shift to mentisa and make the exponent so that the value range of mentisa is [1; 2)
// Convert to float and subtract 1.0 so that we get [0; 1)
// See https://mina86.com/2016/random-reals/
impl RandomGeneratable for f32 {
    #[inline(always)]
    fn random(rng: &mut Rand) -> f32 {
        f32::from_bits(u32::random(rng) >> 9 | 0x3f800000) - 1.0
    }
}
impl RandomGeneratable for f64 {
    #[inline(always)]
    fn random(rng: &mut Rand) -> f64 {
        f64::from_bits(u64::random(rng) >> 12 | 0x3ff0000000000000) - 1.0
    }
}

const F32_VALUE: u32 = !0 >> 9;
const F32_MENTISA: u32 = 0x3f800000;
impl RandomGeneratable for [f32; 2] {
    #[inline(always)]
    fn random(rng: &mut Rand) -> Self {
        const F32_VALUE_X2: u64 = assume_copy(&[F32_VALUE; 2]);
        const F32_MENTISA_X2: u64 = assume_copy(&[F32_MENTISA; 2]);
        let vec: [f32; 2] = assume_is(u64::random(rng) & F32_VALUE_X2 | F32_MENTISA_X2);

        [vec[0] - 1.0, vec[1] - 1.0]
    }
}
impl RandomGeneratable for [f32; 3] {
    #[inline(always)]
    fn random(rng: &mut Rand) -> Self {
        assume_is((<[f32; 2]>::random(rng), f32::random(rng)))
    }
}
impl RandomGeneratable for [f32; 4] {
    #[inline(always)]
    fn random(rng: &mut Rand) -> Self {
        const F32_VALUE_X2: u128 = assume_copy(&[F32_VALUE; 4]);
        const F32_MENTISA_X2: u128 = assume_copy(&[F32_MENTISA; 4]);
        let vec: [f32; 4] = assume_is(u128::random(rng) & F32_VALUE_X2 | F32_MENTISA_X2);

        [vec[0] - 1.0, vec[1] - 1.0, vec[2] - 1.0, vec[3] - 1.0]
    }
}

const F64_VALUE: u64 = !0 >> 12;
const F64_MENTISA: u64 = 0x3ff0000000000000;
impl RandomGeneratable for [f64; 2] {
    #[inline(always)]
    fn random(rng: &mut Rand) -> Self {
        const F64_VALUE_X2: u128 = assume_copy(&[F64_VALUE; 2]);
        const F64_MENTISA_X2: u128 = assume_copy(&[F64_MENTISA; 2]);
        let vec: [f64; 2] = assume_is(u128::random(rng) & F64_VALUE_X2 | F64_MENTISA_X2);

        [vec[0] - 1.0, vec[1] - 1.0]
    }
}

#[cfg(feature = "glam")]
mod glam_impl {
    #[cfg(feature = "glam_027")]
    pub use glam_027::{Vec2, Vec3, Vec3A, Vec4};
    #[cfg(feature = "glam_026")]
    pub use glam_026::{Vec2, Vec3, Vec3A, Vec4};
    #[cfg(feature = "glam_025")]
    pub use glam_025::{Vec2, Vec3, Vec3A, Vec4};
    #[cfg(feature = "glam_024")]
    pub use glam_025::{Vec2, Vec3, Vec3A, Vec4};
    use super::*;

    impl RandomGeneratable for Vec2 {
        #[inline(always)]
        fn random(rng: &mut Rand) -> Self {
            Vec2::from_array(rng.gen())
        }
    }
    impl RandomGeneratable for Vec3 {
        #[inline(always)]
        fn random(rng: &mut Rand) -> Self {
            Vec3::from_array(rng.gen())
        }
    }
    impl RandomGeneratable for Vec3A {
        #[inline(always)]
        fn random(rng: &mut Rand) -> Self {
            Vec3A::from_array(rng.gen())
        }
    }
    impl RandomGeneratable for Vec4 {
        #[inline(always)]
        fn random(rng: &mut Rand) -> Self {
            Vec4::from_array(rng.gen())
        }
    }
}
