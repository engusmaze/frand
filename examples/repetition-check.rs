use frand::Rand;
use hashbrown::HashSet;

fn main() {
    // We insert a new number after every billion values, effectively checking for a loop
    let mut set = HashSet::new();

    let mut rng = Rand::new();
    let mut i = 0usize;
    loop {
        let next_end = i + 1_000_000_000;
        loop {
            let value = rng.gen::<u64>();
            if set.contains(&value) {
                panic!("Cringe {i}!");
            }
            i += 1;
            if i == next_end {
                set.insert(value);
                break;
            }
        }
        if i % 1_000_000_000 == 0 {
            println!("Checked {} billion numbers!", i / 1_000_000_000);
        }
    }
}
