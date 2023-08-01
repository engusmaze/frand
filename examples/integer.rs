use frand::Rand;

fn main() {
    let mut rng = Rand::new();
    println!("{}", rng.gen::<i8>());
    println!("{}", rng.gen::<u64>());
}
