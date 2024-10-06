//! An exploration of the properties of TentHash's mixing function by making a
//! smaller-size version of it.  The purpose of the smaller size is primarily to
//! let us do statistical collision tests at an appreciable portion of the
//! function's output size, which is infeasible on the full-size mix function.
//! But it makes other analysis run faster as well.

mod stats;

use stats::{
    compute_stats, generate_bit_combinations, generate_bit_combinations_inv, generate_counting,
    generate_counting_rev, generate_random, generate_single_1_bit,
};

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

fn main() {
    println!("\nRandom:");
    collision_test(20, 1 << 20, generate_random);
    let chart_0 = compute_stats(
        generate_random,
        |a, b| {
            *b = *a;
            mix(b);
        },
        1 << 16,
    );
    chart_0.print_report();

    println!("\nCounting:");
    collision_test(20, 1 << 20, generate_counting);
    let chart_1 = compute_stats(
        generate_counting,
        |a, b| {
            *b = *a;
            mix(b);
        },
        1 << 16,
    );
    chart_1.print_report();

    println!("\nCounting reversed:");
    collision_test(20, 1 << 20, generate_counting_rev);
    let chart_2 = compute_stats(
        generate_counting_rev,
        |a, b| {
            *b = *a;
            mix(b);
        },
        1 << 16,
    );
    chart_2.print_report();

    println!("\nBit combinations:");
    collision_test(20, 1 << 20, generate_bit_combinations);
    let chart_2 = compute_stats(
        generate_bit_combinations,
        |a, b| {
            *b = *a;
            mix(b);
        },
        1 << 16,
    );
    chart_2.print_report();

    println!("\nBit combinations inverted:");
    collision_test(20, 1 << 20, generate_bit_combinations_inv);
    let chart_2 = compute_stats(
        generate_bit_combinations_inv,
        |a, b| {
            *b = *a;
            mix(b);
        },
        1 << 16,
    );
    chart_2.print_report();

    println!("\nSingle bit:");
    // No collision test because due to this generator's nature we can't get
    // enough buckets to be meaningful.
    // collision_test(5, 32, generate_single_1_bit);
    let chart_3 = compute_stats(
        generate_single_1_bit,
        |a, b| {
            *b = *a;
            mix(b);
        },
        32,
    );
    chart_3.print_report();
}

fn collision_test<F>(log_buckets: usize, item_count: usize, gen: F)
where
    F: Fn(usize, &mut State),
{
    let bucket_count = 1 << log_buckets;
    let mut buckets_low_bits = vec![0u32; bucket_count];
    let mut buckets_high_bits = vec![0u32; bucket_count];
    let mut buckets_combined_bits = vec![0u32; bucket_count];

    for i in 0..item_count {
        let mut state = 0;
        gen(i, &mut state);

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
