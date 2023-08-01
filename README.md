# FRand

##### example

```rs
use frand::Rand;

let mut rng = Rand::new();
println!("{}", rng.gen::<u64>());
```

## Speeds

### u128

| rand::ThreadRng | rand::SmallRng | frand::Rand |
| :-------------: | :------------: | :---------: |
|      1.0x       |     1.88x      |    2.68x    |

### f64

| rand::ThreadRng | rand::SmallRng | frand::Rand |
| :-------------: | :------------: | :---------: |
|      1.0x       |     2.94x      |    3.21x    |

### u64

| rand::ThreadRng | rand::SmallRng | frand::Rand |
| :-------------: | :------------: | :---------: |
|      1.0x       |     2.97x      |    3.19x    |

If you have suggestions on how to improve this library, you can contribute to this project!
