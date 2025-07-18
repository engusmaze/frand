#![feature(test)]

use fastrand::Rng;

frand::setup_benches! {
    _0_bool_fastrand {
        rng: Rng::with_seed(1),
        iteration: rng.bool(),
    }
    _1_u8_fastrand {
        rng: Rng::with_seed(1),
        iteration: rng.u8(..),
    }
    _2_u16_fastrand {
        rng: Rng::with_seed(1),
        iteration: rng.u16(..),
    }
    _3_u32_fastrand {
        rng: Rng::with_seed(1),
        iteration: rng.u32(..),
    }
    _4_u64_fastrand {
        rng: Rng::with_seed(1),
        iteration: rng.u64(..),
    }
    _5_u128_fastrand {
        rng: Rng::with_seed(1),
        iteration: rng.u128(..),
    }
    _6_f32_fastrand {
        rng: Rng::with_seed(1),
        iteration: rng.f32(),
    }
    _7_f64_fastrand {
        rng: Rng::with_seed(1),
        iteration: rng.f64(),
    }
}
