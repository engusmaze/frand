use core::mem::transmute;

use super::*;

// Base generators
impl RandomGeneratable for u32 {
    #[inline(always)]
    fn random(rng: &mut Rand) -> u32 {
        let value = rng.seed.wrapping_add(12964901029718341801);
        rng.seed = value;
        (value.wrapping_mul(18162115696561729952 ^ value) >> 32) as u32
    }
}
impl RandomGeneratable for u64 {
    #[inline(always)]
    fn random(rng: &mut Rand) -> u64 {
        let mut value = rng.seed.wrapping_add(12964901029718341801);
        rng.seed = value;
        value = value.wrapping_mul(149988720821803190 ^ value);
        value ^ value >> 32
    }
}
impl RandomGeneratable for u128 {
    #[inline(always)]
    fn random(rng: &mut Rand) -> u128 {
        let value = rng.seed.wrapping_add(12964901029718341801);
        rng.seed = value;
        let a = value.wrapping_mul(6713055444315782188 ^ value);
        let b = value.wrapping_mul(4683141479006300164 ^ value);
        unsafe { transmute([a ^ a >> 32, b ^ b >> 32]) }
    }
}

impl RandomGeneratable for u8 {
    #[inline(always)]
    fn random(rng: &mut Rand) -> u8 {
        rng.gen::<u32>() as u8
    }
}
impl RandomGeneratable for u16 {
    #[inline(always)]
    fn random(rng: &mut Rand) -> u16 {
        rng.gen::<u32>() as u16
    }
}

impl RandomGeneratable for usize {
    #[inline(always)]
    fn random(rng: &mut Rand) -> usize {
        rng.gen::<u32>() as usize
    }
}

impl RandomGeneratable for bool {
    #[inline(always)]
    fn random(rng: &mut Rand) -> bool {
        u8::random(rng) & 1 == 0
    }
}

/// Macro to implement repeating implementations
macro_rules! implement_cast {
    ($($from: ty => $to: ty,)*) => {
        $(
            impl RandomGeneratable for $to {
                #[inline(always)]
                fn random(rng: &mut Rand) -> $to {
                    unsafe { transmute(<$from>::random(rng)) }
                }
            }
        )*
    };
}
implement_cast! {
    u8 => i8,
    u16 => i16,
    u32 => i32,
    u64 => i64,
    u128 => i128,

    usize => isize,

    u16 => [u8; 2],

    u32 => [u8; 4],
    u32 => [u16; 2],

    u64 => [u8; 8],
    u64 => [u16; 4],
    u64 => [u32; 2],

    u128 => [u8; 16],
    u128 => [u16; 8],
    u128 => [u32; 4],
    u128 => [u64; 2],
}
