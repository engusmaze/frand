use frand::Rand;

fn main() {
    let mut rng = Rand::new();
    println!("{}", rng.random::<i8>());
    println!("{}", rng.random::<u64>());
}
