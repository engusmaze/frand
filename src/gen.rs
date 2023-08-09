use crate::Rand;

pub trait RandomGeneratable {
    fn create_random(rng: &mut Rand) -> Self;
}

// Macro to implement repeating code
macro_rules! implement_cast {
    ($($typ: ty)*) => {
        $(
            impl RandomGeneratable for $typ {
                #[inline(always)]
                fn create_random(rng: &mut Rand) -> $typ {
                    rng.next_u64() as $typ
                }
            }
        )*
    };
}
implement_cast! {
    u8 u16 u32 u64
    i8 i16 i32 i64
    usize isize
}

impl RandomGeneratable for u128 {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> u128 {
        (rng.next_u64() as u128) << 64 | rng.next_u64() as u128
    }
}
impl RandomGeneratable for i128 {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> i128 {
        (rng.next_u64() as i128) << 64 | rng.next_u64() as i128
    }
}

impl RandomGeneratable for bool {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> bool {
        rng.gen::<u8>() & 1 == 0
    }
}

// Floats

// Shift to mentisa and make the exponent so that the value range of mentisa is [1; 2)
// Convert to float and subtract 1.0 so that we get [0; 1)
// See https://mina86.com/2016/random-reals/
impl RandomGeneratable for f32 {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> f32 {
        f32::from_bits(rng.next_u64() as u32 >> 9 | 0x3f800000) - 1.0
    }
}
impl RandomGeneratable for f64 {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> f64 {
        f64::from_bits(rng.next_u64() >> 12 | 0x3ff0000000000000) - 1.0
    }
}

pub trait Shuffle {
    fn shuffle(&mut self, rng: &mut Rand);
}
pub trait Shuffled {
    fn shuffled(self, rng: &mut Rand) -> Self;
}
impl<T> Shuffle for [T] {
    #[inline]
    fn shuffle(&mut self, rng: &mut Rand) {
        // https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
        for i in (1..self.len()).rev() {
            self.swap(i, rng.gen::<usize>() % (i + 1));
        }
    }
}
impl<T> Shuffled for Box<[T]> {
    #[inline]
    fn shuffled(mut self, rng: &mut Rand) -> Self {
        self.shuffle(rng);
        self
    }
}
impl<T> Shuffle for Vec<T> {
    #[inline]
    fn shuffle(&mut self, rng: &mut Rand) {
        self.as_mut_slice().shuffle(rng)
    }
}
impl<T> Shuffled for Vec<T> {
    #[inline]
    fn shuffled(mut self, rng: &mut Rand) -> Self {
        self.shuffle(rng);
        self
    }
}
