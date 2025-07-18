# FRand

[![Crate](https://img.shields.io/crates/v/frand.svg)](https://crates.io/crates/frand)

**FRand** is a blazingly fast, small, and simple pseudo-random number generator (PRNG) written in Rust. The advantage of using FRand is that it can produce more random numbers per second than other libraries. It also produces high-quality random numbers using a fast **non-cryptographic** hashing algorithm.

To find the best constants for the algorithm, I used an automated program that tried many random constants per second and measured the bias of the output. The bias was estimated using the avalanche effect, which is a property of good hash functions that ensures that a small change in the input produces a large change in the output. The program selected the constants that minimized the bias and maximized the randomness.

**FRand** is really simple to use. Here is a simple example of how to use FRand to generate a random float:

```rs
use frand::Rand;

let mut rng = Rand::new();
println!("{}", rng.random::<f32>());
```

## Speeds

### u64

| rand::ThreadRng | rand::SmallRng | fastrand::Rng | frand::Rand |
| :-------------: | :------------: | :-----------: | :---------: |
|      1.00x      |     3.74x      |     5.80x     |    6.09x    |

### f64

| rand::ThreadRng | rand::SmallRng | fastrand::Rng | frand::Rand |
| :-------------: | :------------: | :-----------: | :---------: |
|      1.00x      |     3.79x      |     2.34x     |    5.57x    |

### u128

| rand::ThreadRng | rand::SmallRng | fastrand::Rng | frand::Rand |
| :-------------: | :------------: | :-----------: | :---------: |
|      1.00x      |     2.15x      |     6.79x     |    7.24x    |

If you have suggestions on how to improve this library, you can contribute to this project!
