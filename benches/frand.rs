#![feature(test)]

use frand::Rand;

frand::setup_benches! {
    _00_bool_frand {
        rand: Rand::new(),
        iteration: rand.gen::<bool>(),
    }
    _01_u8_frand {
        rand: Rand::new(),
        iteration: rand.gen::<u8>(),
    }
    _02_u16_frand {
        rand: Rand::new(),
        iteration: rand.gen::<u16>(),
    }
    _03_u32_frand {
        rand: Rand::new(),
        iteration: rand.gen::<u32>(),
    }
    _04_u64_frand {
        rand: Rand::new(),
        iteration: rand.gen::<u64>(),
    }
    _05_u128_frand {
        rand: Rand::new(),
        iteration: rand.gen::<u128>(),
    }
    _06_f32_frand {
        rand: Rand::new(),
        iteration: rand.gen::<f32>(),
    }
    _07_f32x2_frand {
        rand: Rand::new(),
        iteration: rand.gen::<[f32; 2]>(),
    }
    _08_f32x3_frand {
        rand: Rand::new(),
        iteration: rand.gen::<[f32; 3]>(),
    }
    _09_f32x4_frand {
        rand: Rand::new(),
        iteration: rand.gen::<[f32; 4]>(),
    }
    _10_f64_frand {
        rand: Rand::new(),
        iteration: rand.gen::<f64>(),
    }
    _11_f64x2_frand {
        rand: Rand::new(),
        iteration: rand.gen::<[f64; 2]>(),
    }
}
