use crate::Rand;

pub trait RandomGeneratable {
    fn create_random(rng: &mut Rand) -> Self;
}

// Unsigned integers
impl RandomGeneratable for u8 {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> u8 {
        rng.next_u64() as u8
    }
}
impl RandomGeneratable for u16 {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> u16 {
        rng.next_u64() as u16
    }
}
impl RandomGeneratable for u32 {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> u32 {
        rng.next_u64() as u32
    }
}
impl RandomGeneratable for u64 {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> u64 {
        rng.next_u64()
    }
}
impl RandomGeneratable for u128 {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> u128 {
        (rng.next_u64() as u128) << 64 | rng.next_u64() as u128
    }
}

// Signed integers
impl RandomGeneratable for i8 {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> i8 {
        rng.next_u64() as i8
    }
}
impl RandomGeneratable for i16 {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> i16 {
        rng.next_u64() as i16
    }
}
impl RandomGeneratable for i32 {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> i32 {
        rng.next_u64() as i32
    }
}
impl RandomGeneratable for i64 {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> i64 {
        rng.next_u64() as i64
    }
}
impl RandomGeneratable for i128 {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> i128 {
        (rng.next_u64() as i128) << 64 | rng.next_u64() as i128
    }
}

impl RandomGeneratable for usize {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> usize {
        rng.next_u64() as usize
    }
}
impl RandomGeneratable for isize {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> isize {
        rng.next_u64() as isize
    }
}

// Floats

// Shift to mentisa and make the exponent so that the value range of mentisa is [1; 2)
// Convert to float and subtract 1.0 so that we get [0; 1)
impl RandomGeneratable for f32 {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> f32 {
        unsafe { *(&(rng.next_u64() as u32 >> 9 | 0x3f800000) as *const _ as *const f32) - 1.0 }
    }
}
impl RandomGeneratable for f64 {
    #[inline(always)]
    fn create_random(rng: &mut Rand) -> f64 {
        unsafe { *(&(rng.next_u64() >> 12 | 0x3ff0000000000000) as *const _ as *const f64) - 1.0 }
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
        for i in 0..self.len() {
            self.swap(i, rng.gen::<usize>() % self.len());
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
