use frand::Rand;
use image::{ImageBuffer, Luma};

fn main() {
    let mut rand = Rand::new();
    ImageBuffer::from_fn(512, 512, |_x, _y| Luma([rand.random::<u8>()]))
        .save("test-random-image.png")
        .expect("Failed to save image");
}
