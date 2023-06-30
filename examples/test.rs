use frand::Rand;

fn main() {
    let mut rng = Rand::new();
    println!("{}", rng.gen::<u64>());
}
