# FRand

##### example

```rs
use frand::Rand;

let mut rng = Rand::new();
println!("{}", rng.gen::<u64>());
```

## Speeds

| frand::Rand / u64 | rand::SmallRng / u64 | frand::QualityRand/ u64 | rand::Rand / u64 |
| :---------------: | :------------------: | :---------------------: | :--------------: |
|       100%        |         107%         |          241%           |       325%       |

| frand::Rand / u128 | rand::SmallRng / u128 | frand::QualityRand / u128 | rand::Rand / u128 |
| :----------------: | :-------------------: | :-----------------------: | :---------------: |
|        200%        |         287%          |           463%            |       565%        |

```
test frand_rand_bench              ... bench:     172,716 ns/iter (+/- 6,312)
test rand_small_rng_bench          ... bench:     185,326 ns/iter (+/- 3,666)
test frand_quality_rand_bench      ... bench:     416,062 ns/iter (+/- 21,106)
test rand_rand_bench               ... bench:     559,483 ns/iter (+/- 11,361)

test frand_rand_bench_u128         ... bench:     345,203 ns/iter (+/- 7,502)
test rand_small_rng_bench_u128     ... bench:     495,719 ns/iter (+/- 6,524)
test frand_quality_rand_bench_u128 ... bench:     798,820 ns/iter (+/- 13,968)
test rand_rand_bench_u128          ... bench:     972,493 ns/iter (+/- 12,191)
```

If you have suggestions on how to improve this library, you can contribute to this project!
