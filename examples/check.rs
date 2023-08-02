use frand::Rand;
use rand::{thread_rng, Rng};

// Check bias

fn main() {
    let mut rng = Rand::new();
    let mut rng = thread_rng();

    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    loop {
        let mut last_value = rng.gen::<u64>();
        let mut total_bits: u64 = 0;
        let mut total_same_bits: u64 = 0;

        for _ in 0..100 {
            let new_value = rng.gen::<u64>();

            total_bits += 64;

            let same_bits = ((!last_value) ^ new_value).count_ones();
            total_same_bits += same_bits as u64;

            last_value = new_value;
        }

        let q = total_same_bits as f64 / total_bits as f64;
        min = min.min(q);
        max = max.max(q);

        // println!("Max: {max}\tMin: {min}");
        println!("Bias: {}", (max - 0.5).max(0.5 - min))
    }
}
