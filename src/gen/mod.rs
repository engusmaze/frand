pub use crate::*;

pub trait RandomGeneratable {
    fn random(rng: &mut Rand) -> Self;
}

// Base generators
impl RandomGeneratable for u32 {
    #[inline(always)]
    fn random(rng: &mut Rand) -> u32 {
        let value = rng.seed.wrapping_add(12964901029718341801);
        rng.seed = value;
        (value.wrapping_mul(13708351713526598943 ^ value) >> 16) as u32
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
        assume_is([a ^ a >> 32, b ^ b >> 32])
    }
}

/// Macro to implement repeating implementations
macro_rules! implement_cast {
    ($($from: ty => $to: ty)*) => {
        $(
            impl RandomGeneratable for $to {
                #[inline(always)]
                fn random(rng: &mut Rand) -> $to {
                    assume_is(<$from>::random(rng))
                }
            }
        )*
    };
}
implement_cast! {
    u32 => u8
    u32 => u16
    u32 => i8
    u32 => i16
    u64 => i64
    u128 => i128
    u64 => usize
    u64 => isize

    u16 => [u8; 2]

    u32 => [u8; 4]
    u32 => [u16; 2]

    u64 => [u8; 8]
    u64 => [u16; 4]
    u64 => [u32; 2]

    u128 => [u8; 16]
    u128 => [u16; 8]
    u128 => [u32; 4]
    u128 => [u64; 2]
}

impl RandomGeneratable for bool {
    #[inline(always)]
    fn random(rng: &mut Rand) -> bool {
        u8::random(rng) & 1 == 0
    }
}

mod float;
