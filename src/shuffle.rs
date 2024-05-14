use crate::*;

trait Shuffle {
    fn shuffle(&mut self, rng: &mut Rand);
}
impl<T> Shuffle for [T] {
    fn shuffle(&mut self, rng: &mut Rand) {
        // https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
        for i in (0..self.len()).rev() {
            self.swap(i, usize::random(rng) % (i + 1));
        }
    }
}

/// Shuffle any array-like type
pub trait Shufflable<T>: AsMut<[T]> + Sized {
    #[inline]
    fn shuffle(&mut self, rng: &mut Rand) {
        self.as_mut().shuffle(rng);
    }
    #[inline]
    fn shuffled(mut self, rng: &mut Rand) -> Self {
        self.shuffle(rng);
        self
    }
}
impl<T, U: AsMut<[T]>> Shufflable<T> for U {}
