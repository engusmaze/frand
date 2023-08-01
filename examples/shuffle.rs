use frand::{Rand, Shuffle, Shuffled};

fn main() {
    let mut rng = Rand::new();

    let shuffled_array = (0..=255).collect::<Box<[u8]>>().shuffled(&mut rng);
    println!("{:?}", shuffled_array);

    let mut mutable_array_shuffle = (0..=255).collect::<Box<[u8]>>();
    mutable_array_shuffle.shuffle(&mut rng);
    println!("{:?}", mutable_array_shuffle);
}
