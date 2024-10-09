//! An exploration of the properties of TentHash's mixing function by making a
//! smaller-size version of it.  The purpose of the smaller size is primarily to
//! let us do statistical collision tests at an appreciable portion of the
//! function's output size, which is infeasible on the full-size mix function.
//! But it makes other analysis run faster as well.

mod stats;

use stats::{bit_combinations, compute_stats, generate_random};

fn main() {
    for pattern in PATTERNS {
        println!("\n{}:", pattern.name);
        collision_test(
            pattern.collision_log_population,
            1 << pattern.collision_log_population,
            pattern.gen_function,
        );
        let chart = compute_stats(
            pattern.gen_function,
            |a, b| {
                *b = *a;
                mix(b);
            },
            pattern.avalanche_rounds,
        );
        chart.print_report();
    }
}

type State = u32;

/// Same construction as TentHash's mixing function, just with a smaller state.
fn mix(state: &mut State) {
    const ROTS: &[[u32; 2]] = &[
        // The rounds above the line have been tuned to give similar diffusion
        // (relative to total bits) as TentHash's full-size mixing function.
        // Uncommenting the rounds below the line will give full diffusion.
        [2, 5],
        [7, 4],
        [1, 2],
        [2, 5],
        [7, 4],
        [1, 2],
        //----------
        // [2, 5],
        // [7, 4],
        // [1, 2],
    ];

    let mut bytes: [u8; 4] = unsafe { std::mem::transmute(*state) };
    for rot in ROTS {
        bytes[0] = bytes[0].wrapping_add(bytes[2]);
        bytes[1] = bytes[1].wrapping_add(bytes[3]);
        bytes[2] = bytes[2].rotate_left(rot[0]) ^ bytes[0];
        bytes[3] = bytes[3].rotate_left(rot[1]) ^ bytes[1];
        bytes.swap(0, 1);
    }
    *state = unsafe { std::mem::transmute(bytes) };
}

struct BitPattern<'a> {
    name: &'a str,
    gen_function: &'a dyn Fn(usize) -> State,
    avalanche_rounds: usize,
    collision_log_population: usize,
}

// Note that by changing the counting and bit-combo rounds, you're also changing
// what is being tested to some extent.  With the random rounds, on the other
// hand, cranking it up mainly just reduces variance.
const RANDOM_ROUNDS: usize = 1 << 14;
const COUNTING_ROUNDS: usize = 1 << 12;
const BIT_COMBO_ROUNDS: usize = 1 << 12;

const PATTERNS: &[BitPattern] = &[
    BitPattern {
        name: "Random",
        gen_function: &generate_random,
        avalanche_rounds: RANDOM_ROUNDS,
        collision_log_population: 20,
    },
    BitPattern {
        name: "Counting",
        gen_function: &|i| i as State,
        avalanche_rounds: COUNTING_ROUNDS,
        collision_log_population: 20,
    },
    BitPattern {
        name: "Counting bit-reversed",
        gen_function: &|i| (i as State).reverse_bits(),
        avalanche_rounds: COUNTING_ROUNDS,
        collision_log_population: 20,
    },
    BitPattern {
        name: "Bit combinations",
        gen_function: &bit_combinations,
        avalanche_rounds: BIT_COMBO_ROUNDS,
        collision_log_population: 20,
    },
    BitPattern {
        name: "Bit combinations bit-reversed",
        gen_function: &|i| bit_combinations(i).reverse_bits(),
        avalanche_rounds: BIT_COMBO_ROUNDS,
        collision_log_population: 20,
    },
    BitPattern {
        name: "Bit combinations inverted",
        gen_function: &|i| !bit_combinations(i),
        avalanche_rounds: BIT_COMBO_ROUNDS,
        collision_log_population: 20,
    },
    BitPattern {
        name: "single-bit",
        gen_function: &|i| 1 << i,

        // NOTE: because this test has a small, fixed number of rounds by its
        // nature, the generated statistics should be interpreted a little
        // differently. In particular, even a very good mixing function is
        // unlikely to achieve "perfect" avalanche or BIC by this measure,
        // purely because it's impossible to collect enough samples to reduce
        // variance enough.
        avalanche_rounds: 32,

        collision_log_population: 20,
    },
];

fn collision_test<F>(log_buckets: usize, item_count: usize, gen: F)
where
    F: Fn(usize) -> State,
{
    let bucket_count = 1 << log_buckets;
    let mut buckets_low_bits = vec![0u32; bucket_count];
    let mut buckets_high_bits = vec![0u32; bucket_count];
    let mut buckets_combined_bits = vec![0u32; bucket_count];

    for i in 0..item_count {
        let mut state = gen(i);

        mix(&mut state);
        let n = state as usize;
        buckets_low_bits[n % bucket_count] += 1;
        buckets_high_bits[(n >> (32 - log_buckets)) % bucket_count] += 1;
        let n = n ^ (n >> (32 - log_buckets));
        buckets_combined_bits[n % bucket_count] += 1;
    }

    let do_stats = |buckets: &[u32]| {
        let mut collisions = 0usize;
        let mut min_count = usize::MAX;
        let mut max_count = 0usize;
        for b in buckets {
            let b = *b as usize;
            collisions += b.saturating_sub(1);
            min_count = b.min(min_count);
            max_count = b.max(max_count);
        }

        println!(
            "            Collisions: {}
            Smallest bucket: {}
            Largest bucket:  {}",
            collisions, min_count, max_count,
        );
    };

    println!(
        "    Collision tests: {} buckets and {} items:",
        bucket_count, item_count
    );
    println!("        Using high bits:");
    do_stats(&buckets_low_bits);
    println!("        Using low bits:");
    do_stats(&buckets_high_bits);
    println!("        Using xored high and low bits:");
    do_stats(&buckets_combined_bits);
}
