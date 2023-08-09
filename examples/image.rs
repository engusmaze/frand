use fastrand::Rng;
use frand::Rand;
use image::{ImageBuffer, Luma};

fn main() {
    let mut rng = Rng::new();
    ImageBuffer::from_fn(512, 512, |_x, _y| Luma([rng.u8(..)]))
        .save("test-image.png")
        .expect("Failed to save image");
}
