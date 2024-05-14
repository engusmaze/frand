use frand::Rand;
use glam::Vec2;

fn main() {
    let mut rng = Rand::new();

    println!("{}", rng.gen_range(0..69));

    println!("{}", rng.gen_range(-128..128));

    println!("{}", rng.gen_range(Vec2::ZERO..Vec2::ONE));
}
