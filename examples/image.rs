use frand::{hash64, mix2_64, Rand};
use image::{ImageBuffer, Luma};

fn main() {
    let mut rng = Rand::new();
    ImageBuffer::from_fn(512, 512, |_x, _y| Luma([(rng.gen::<f64>() * 256.0) as u8]))
        .save("test-image.png")
        .expect("Failed to save image");
}
