use frand::Rand;
use image::{ImageBuffer, Luma};

fn main() {
    let mut rand = Rand::new();
    for byte in 0..8 {
        ImageBuffer::from_fn(512, 512, |_x, _y| {
            Luma([rand.gen::<u64>().to_le_bytes()[byte]])
        })
        .save(format!("test-image-byte-{byte}.png"))
        .expect("Failed to save image");
    }
}
