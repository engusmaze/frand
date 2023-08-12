#![feature(test)]

use rand::{thread_rng, Rng};

frand::setup_benches! {
    _0_bool_rand_thread_rng {
        rand: thread_rng(),
        iteration: rand.gen::<bool>(),
    }
    _1_u8_rand_thread_rng {
        rand: thread_rng(),
        iteration: rand.gen::<u8>(),
    }
    _2_u16_rand_thread_rng {
        rand: thread_rng(),
        iteration: rand.gen::<u16>(),
    }
    _3_u32_rand_thread_rng {
        rand: thread_rng(),
        iteration: rand.gen::<u32>(),
    }
    _4_u64_rand_thread_rng {
        rand: thread_rng(),
        iteration: rand.gen::<u64>(),
    }
    _5_u128_rand_thread_rng {
        rand: thread_rng(),
        iteration: rand.gen::<u128>(),
    }
    _6_f32_rand_thread_rng {
        rand: thread_rng(),
        iteration: rand.gen::<f32>(),
    }
    _7_f64_rand_thread_rng {
        rand: thread_rng(),
        iteration: rand.gen::<f64>(),
    }
}
