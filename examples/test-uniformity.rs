use frand::Rand;

// Randomness test
// Check how evenly distributed the flipped bits are

fn main() {
    let mut flipped_bits = [0u64; 64];
    let mut rand = Rand::new();
    loop {
        for _ in 0..1_000_000 {
            let integer = rand.gen::<u64>();
            for shift in 0..64 {
                if (integer >> shift) & 1 == 1 {
                    flipped_bits[shift] += 1;
                }
            }
        }
        let total_bits_flipped: u64 = flipped_bits.iter().sum();
        let top_deviation = flipped_bits.iter().max().unwrap();
        let bottom_deviation = flipped_bits.iter().min().unwrap();

        let average_flips = total_bits_flipped / 64;
        println!(
            "Maximum deviation: {:?}%",
            ((top_deviation - average_flips).max(average_flips - bottom_deviation) as f64
                / total_bits_flipped as f64)
                * 100.0
        );
    }
}
