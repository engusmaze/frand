#![feature(test)]

use rand::{rngs::SmallRng, Rng, SeedableRng};

frand::setup_benches! {
    _0_bool_rand_small_rng {
        rand: SmallRng::from_entropy(),
        iteration: rand.gen::<bool>(),
    }
    _1_u8_rand_small_rng {
        rand: SmallRng::from_entropy(),
        iteration: rand.gen::<u8>(),
    }
    _2_u16_rand_small_rng {
        rand: SmallRng::from_entropy(),
        iteration: rand.gen::<u16>(),
    }
    _3_u32_rand_small_rng {
        rand: SmallRng::from_entropy(),
        iteration: rand.gen::<u32>(),
    }
    _4_u64_rand_small_rng {
        rand: SmallRng::from_entropy(),
        iteration: rand.gen::<u64>(),
    }
    _5_u128_rand_small_rng {
        rand: SmallRng::from_entropy(),
        iteration: rand.gen::<u128>(),
    }
    _6_f32_rand_small_rng {
        rand: SmallRng::from_entropy(),
        iteration: rand.gen::<f32>(),
    }
    _7_f64_rand_small_rng {
        rand: SmallRng::from_entropy(),
        iteration: rand.gen::<f64>(),
    }
}
