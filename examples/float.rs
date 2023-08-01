use frand::Rand;

fn main() {
    let mut rng = Rand::new();
    // Floats are generated in range [0; 1)
    println!("{}", rng.gen::<f32>());
    println!("{}", rng.gen::<f64>());
}
