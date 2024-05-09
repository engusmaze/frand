// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Thread-local random number generator

use core::cell::UnsafeCell;
use std::rc::Rc;
use std::thread_local;

use rand::RngCore;

use crate::{RandomGeneratable, Rand as Frand };



#[derive(Clone, Debug,)]
pub struct ThreadFrand {
    // Rc is explicitly !Send and !Sync
    rng: Rc<UnsafeCell<Frand>>,
}

thread_local!(
    // We require Rc<..> to avoid premature freeing when thread_rng is used
    // within thread-local destructors. See #968.
    static THREAD_RNG_KEY: Rc<UnsafeCell<Frand>> = {
        let rng = Frand::new();
        Rc::new(UnsafeCell::new(rng))
    }
);


pub fn thread_randy() -> ThreadFrand {
    let rng = THREAD_RNG_KEY.with(|t| t.clone());
    ThreadFrand { rng }
}



impl Default for ThreadFrand {
    fn default() -> ThreadFrand {
        thread_randy()
    }
}

// impl ThreadFrand {
    
//     pub fn gen<T>() {
//         let rng = unsafe { &mut *self.rng.get() };
//     }
// }

impl ThreadFrand {
    /// Mixes the current seed with the provided value.
    /// This mixing operation is particularly useful in `no_std` environments when you want
    /// to create a PRNG that incorporates external factors or environmental entropy, such
    /// as time, to increase randomness.
    #[inline]
    pub fn mix(&mut self, value: u64) {
        (unsafe { &mut *self.rng.get() }).mix(value);
    }

    /// Rehashes the current Rand instance by creating a new one with a fresh seed.
    /// This function is only available when the "std" feature is enabled.
    #[inline]
    pub fn rehash(&mut self) {
        (unsafe { &mut *self.rng.get() }).rehash();
    }

    /// Generates a random value of type T using this Rand instance.
    /// T must implement the RandomGeneratable trait, which defines how to generate random values.
    #[inline(always)]
    pub fn gen<T: RandomGeneratable>(&mut self) -> T {
        (unsafe { &mut *self.rng.get() }).gen::<T>()
    }
}

impl RngCore for ThreadFrand {
    fn next_u32(&mut self) -> u32 {
        (unsafe { &mut *self.rng.get() }).gen::<u32>()
    }

    fn next_u64(&mut self) -> u64 {
        (unsafe { &mut *self.rng.get() }).gen::<u64>()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        (unsafe { &mut *self.rng.get() }).fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        (unsafe { &mut *self.rng.get() }).try_fill_bytes(dest)
    }
}





#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::thread::thread_randy;

    #[test]
    fn test_thread_rng() {

        let mut r = thread_randy();
        // r.gen::<i32>();
        assert_eq!(r.gen_range(0..1), 0);
    }
}
