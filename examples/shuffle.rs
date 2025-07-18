use std::array;

use frand::{Rand, Shufflable};

fn main() {
    let mut rng = Rand::new();

    let shuffled: [u8; 16] = array::from_fn(|i| i as u8).shuffled(&mut rng);
    println!("{shuffled:?}");

    let shuffled_boxed_slice = (0..=16).collect::<Box<[u8]>>().shuffled(&mut rng);
    println!("{shuffled_boxed_slice:?}");

    let mut shuffle_box_slice: Box<[u8]> = (0..=16).collect();
    shuffle_box_slice.shuffle(&mut rng);
    println!("{shuffle_box_slice:?}");

    let mut shuffle_vec: Vec<u8> = (0..=16).collect();
    shuffle_vec.shuffle(&mut rng);
    println!("{shuffle_vec:?}");
}
