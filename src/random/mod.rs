use crate::*;

mod float;
mod int;

/// A trait for types that can be generated randomly using **FRand**.
pub trait Random: Sized {
    /// Generates a random value of the implementing type using the provided `Rand` instance.
    fn random(rng: &mut Rand) -> Self;
}

impl Rand {
    /// Generates a random value of type T using this Rand instance.
    /// T must implement the RandomGeneratable trait, which defines how to generate random values.
    #[inline(always)]
    pub fn random<T: Random>(&mut self) -> T {
        T::random(self)
    }
}

pub trait RandomRange: Sized {
    fn random_range(rng: &mut Rand, range: Range<Self>) -> Self;
}

impl Rand {
    #[inline(always)]
    pub fn random_range<T: RandomRange>(&mut self, range: Range<T>) -> T {
        T::random_range(self, range)
    }
}

/// Ugly ass macros
/// TODO: Replace with codegen
macro_rules! impl_int_random_range {
    ($($i_ty: ty, $u_ty: ty);* $(;)?) => {
        $(
            impl RandomRange for $u_ty {
                #[inline(always)]
                fn random_range(rng: &mut Rand, range: Range<$u_ty>) -> Self {
                    (Self::random(rng) % range.end.wrapping_sub(range.start)).wrapping_add(range.start)
                }
            }
            impl RandomRange for $i_ty {
                #[inline(always)]
                fn random_range(rng: &mut Rand, range: Range<$i_ty>) -> Self {
                    RandomRange::random_range(rng, range.start as $u_ty..range.end as $u_ty) as $i_ty
                }
            }
        )*
    };
}

impl_int_random_range! {
    i8, u8;
    i16, u16;
    i32, u32;
    i64, u64;
    i128, u128;
}

macro_rules! impl_float_random_range {
    ($($ty: ty),* $(,)?) => {
        $(
            impl crate::RandomRange for $ty {
                #[inline(always)]
                fn random_range(rng: &mut Rand, range: Range<$ty>) -> Self {
                    Self::random(rng) * (range.end - range.start) + range.start
                }
            }
        )*
    };
}

impl_float_random_range! {
    f32,
    f64,
}

#[cfg(feature = "glam")]
mod glam_impl {
    use crate::{Rand, Random, Range};
    use glam::{Vec2, Vec3, Vec3A, Vec4};

    impl_float_random_range! {
        Vec2,
        Vec3,
        Vec3A,
        Vec4,
    }
}
