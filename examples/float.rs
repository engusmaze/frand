use frand::Rand;

fn main() {
    let mut rng = Rand::new();
    // Floats are generated in range [0; 1)
    println!("{}", rng.random::<f32>());
    println!("{}", rng.random::<f64>());
}
