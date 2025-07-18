use frand::Rand;
use glam::Vec2;

fn main() {
    let mut rng = Rand::new();

    println!("{}", rng.random_range(0..69));

    println!("{}", rng.random_range(-128..128));

    println!("{}", rng.random_range(Vec2::ZERO..Vec2::ONE));
}
