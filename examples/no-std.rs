#![no_std]

use frand::Rand;

fn main() {
    let mut rand = Rand::with_seed(0);
    let value = rand.gen::<u64>();

    // Use std just to print a value
    {
        extern crate std;

        std::println!("{value}");
    }
}
