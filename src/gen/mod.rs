pub use super::*;

/// A trait for types that can be generated randomly using **FRand**.
pub trait RandomGeneratable {
    /// Generates a random value of the implementing type using the provided `Rand` instance.
    fn random(rng: &mut Rand) -> Self;
}

mod float;
mod int;
