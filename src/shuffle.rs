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
