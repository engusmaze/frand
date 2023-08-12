use frand::Rand;
use image::{ImageBuffer, Luma};

fn main() {
    let mut rand = Rand::new();
    // Create an image for each byte of u64
    for byte in 0..16 {
        ImageBuffer::from_fn(512, 512, |_x, _y| {
            Luma([rand.gen::<u128>().to_le_bytes()[byte]])
        })
        .save(format!("test-image-byte-{byte}.png"))
        .expect("Failed to save image");
    }
}
