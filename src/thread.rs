// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Thread-local random number generator

use std::{cell::UnsafeCell, rc::Rc, thread_local};

use crate::{Rand, Random};

#[derive(Clone, Debug)]
pub struct ThreadRand {
    // Rc is explicitly !Send and !Sync
    rng: Rc<UnsafeCell<Rand>>,
}

thread_local! {
    // We require Rc<..> to avoid premature freeing when thread_rng is used
    // within thread-local destructors. See #968.
    static THREAD_RNG_KEY: Rc<UnsafeCell<Rand>> = {
        let rng = Rand::new();
        Rc::new(UnsafeCell::new(rng))
    }
}

/// Retrieve the lazily-initialized thread-local random number generator.
pub fn thread_rand() -> ThreadRand {
    let rng = THREAD_RNG_KEY.with(|t| t.clone());
    ThreadRand { rng }
}

impl Default for ThreadRand {
    fn default() -> ThreadRand {
        thread_rand()
    }
}

impl ThreadRand {
    #[inline(always)]
    pub fn get_rng(&mut self) -> &mut Rand {
        unsafe { &mut *self.rng.get() }
    }

    /// Mixes the current seed with the provided value.
    /// This mixing operation is particularly useful in `no_std` environments when you want
    /// to create a PRNG that incorporates external factors or environmental entropy, such
    /// as time, to increase randomness.
    #[inline]
    pub fn mix(&mut self, value: u64) {
        self.get_rng().mix(value);
    }

    /// Rehashes the current Rand instance by creating a new one with a fresh seed.
    /// This function is only available when the "std" feature is enabled.
    #[inline]
    pub fn rehash(&mut self) {
        self.get_rng().rehash();
    }

    /// Generates a random value of type T using this Rand instance.
    /// T must implement the Random trait, which defines how to generate random values.
    #[inline(always)]
    pub fn random<T: Random>(&mut self) -> T {
        self.get_rng().random::<T>()
    }
}

#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::thread_rand;

    #[test]
    fn test_thread_rng() {
        let mut r = thread_rand();
        // r.random::<i32>();
        assert_eq!(r.random_range(0..1), 0);
    }
}
