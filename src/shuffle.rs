use crate::*;

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
        for i in (0..self.len()).rev() {
            self.swap(i, usize::random(rng) % (i + 1));
        }
    }
}

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
impl<T> Shuffled for alloc::boxed::Box<[T]> {
    #[inline]
    fn shuffled(mut self, rng: &mut Rand) -> Self {
        self.shuffle(rng);
        self
    }
}
#[cfg(feature = "alloc")]
impl<T> Shuffle for alloc::vec::Vec<T> {
    #[inline]
    fn shuffle(&mut self, rng: &mut Rand) {
        self.as_mut_slice().shuffle(rng)
    }
}
#[cfg(feature = "alloc")]
impl<T> Shuffled for alloc::vec::Vec<T> {
    #[inline]
    fn shuffled(mut self, rng: &mut Rand) -> Self {
        self.shuffle(rng);
        self
    }
}
