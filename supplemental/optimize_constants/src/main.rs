mod avalanche_chart;

use std::io::Write;

use nanorand::{Rng, WyRand};
use rayon::prelude::*;

use avalanche_chart::{
    compute_avalanche_chart, generate_counting, generate_counting_rev, generate_random,
    generate_single_1_bit,
};

/// An "item", representing one set of rotation constants for the mixing
/// function.  It tracks the item's score, and contains an rng for making
/// mutations (so we don't have to syncronize that across threads).
#[derive(Debug, Clone)]
struct Item {
    rng: WyRand,
    rotations: [[u32; 2]; 7],
    score: f64,

    /// Just for fun, give each item a unique ID so we can track them through
    /// the whole process.
    id: u64,
}

fn main() {
    // Lets us dole out incrementing ids to each new item.  Needs to be atomic
    // due to multithreading.
    let item_counter = std::sync::atomic::AtomicU64::new(0);

    // Function that creates a fresh new, completely random item.
    let new_item = || {
        let mut rng = WyRand::new();

        let rotations = [
            [rng.generate_range(1u32..64), rng.generate_range(1u32..64)],
            [rng.generate_range(1u32..64), rng.generate_range(1u32..64)],
            [rng.generate_range(1u32..64), rng.generate_range(1u32..64)],
            [rng.generate_range(1u32..64), rng.generate_range(1u32..64)],
            [rng.generate_range(1u32..64), rng.generate_range(1u32..64)],
            [rng.generate_range(1u32..64), rng.generate_range(1u32..64)],
            [rng.generate_range(1u32..64), rng.generate_range(1u32..64)],
        ];

        // The new item's score is computed with the same number of rounds as in
        // the first and second phase, since those are the only two phases that
        // create new items.
        let score = compute_score_standard(&rotations, 256);

        let prev_id = item_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        Item {
            rng: rng,
            rotations: rotations,
            score: score,
            id: prev_id + 1,
        }
    };

    // Function for mutating an item.  It makes random mutations, checks if the
    // new score is lower than the old one, and if it does it replaces the item
    // with the new mutated one, and otherwise leaves it as-is.
    //
    // It does this iteratively, `iterations` number of times, and uses
    // `scoring_fn()` to do the scoring.
    fn do_random_tweaks(
        scoring_fn: &dyn Fn(&[[u32; 2]]) -> f64,
        iterations: usize,
        item: &mut Item,
    ) {
        item.score = scoring_fn(&item.rotations);

        for _ in 0..iterations {
            std::io::stdout().flush().unwrap();

            let mut r = item.rotations;
            for _ in 0..item.rng.generate_range(1..=2usize) {
                let i = item.rng.generate_range(0..7);
                let j = item.rng.generate_range(0..2);

                let n = item.rng.generate_range(1..64);
                r[i][j] = n;
            }
            let new_score = scoring_fn(&r);

            if new_score > item.score {
                item.score = new_score;
                item.rotations = r;
            }
        }
    }

    // Initial large population.
    let mut population: Vec<Item> = (0..1024).into_par_iter().map(|_| new_item()).collect();

    // This first phase does a small number of mutation iterations on the large
    // population.  The idea is that we're starting out by quickly filtering
    // down a large set of items to a smaller set that seem to be promising.
    println!("\nPhase 1: large population");
    for iteration in 0..100 {
        print!("\r                                  \r");
        print!("Iteration {}", iteration);
        std::io::stdout().flush().unwrap();

        population.par_iter_mut().for_each(|item| {
            do_random_tweaks(&|rots| compute_score_standard(rots, 256), 1, item);
        });
        population.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        println!();
        for item in &population[..16] {
            println!("{}: {}:\n    {:?}", item.id, item.score, item.rotations);
        }
    }

    // This second phase does a larger number of mutation iterations on the most
    // promising (best scoring so far) items from the larger population.
    //
    // Additionally, this periodically replaces the worst performers with new
    // random items, to help prevent getting "stuck" with a population that
    // can't progress any further.  In practice, that probably doesn't matter
    // too much, but it generally doesn't hurt either.
    println!("\nPhase 2: medium population");
    population.truncate(32);
    for iteration in 0..1000 {
        print!("\r                                  \r");
        print!("Iteration {}", iteration);
        std::io::stdout().flush().unwrap();

        population.par_iter_mut().for_each(|item| {
            do_random_tweaks(&|rots| compute_score_standard(rots, 256), 1, item);
        });
        population.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        if iteration % 10 == 0 {
            println!();
            for item in &population[..16] {
                println!("{}: {}:\n    {:?}", item.id, item.score, item.rotations);
            }
        }

        if iteration > 0 && iteration % 100 == 0 {
            let start = population.len() / 2;

            population[start..].par_iter_mut().for_each(|item| {
                *item = new_item();
            });
        }
    }

    // This third phase takes the top performers and does additional mutation
    // iterations on them, but with slower, higher-quality scoring.  The idea is
    // to refine those top performers as much as we reasonably can.
    println!("\n\nPhase 3: small population");
    for iteration in 0..1000 {
        print!("\r                                  \r");
        print!("Iteration {}", iteration);
        std::io::stdout().flush().unwrap();

        population.par_iter_mut().for_each(|item| {
            do_random_tweaks(&|rots| compute_score_standard(rots, 1 << 12), 1, item);
        });
        population.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        if iteration % 10 == 0 {
            // We truncate here rather than before the phase starts because
            // we want to be sure that the scoring we truncate based on is the
            // higher-quality scoring.  And because I'm lazy, and didn't feel
            // like writing the code to re-score them before the phase.
            population.truncate(4);

            println!();
            for item in &population {
                println!("{}: {}:\n    {:?}", item.id, item.score, item.rotations);
            }
        }
    }

    println!(
        "\n\nWinner: {}:\n    {:?}",
        population[0].score, population[0].rotations
    );

    // This last phase takes the single highest performer, and does some
    // systematic changing of the rotation constants to check that the purely
    // random mutuations didn't miss something easy.  In practice, this does
    // usually find some additional improvements, ocassionally even significant
    // ones.
    println!("\nPhase 4: optimizing winner");
    'foo: loop {
        for i in (0..7).rev() {
            for j in (0..2).rev() {
                let mut found_better = false;
                for n in 1..64 {
                    print!(
                        "\r                                       \ritem [{}][{}] as {}",
                        i, j, n,
                    );
                    std::io::stdout().flush().unwrap();

                    let mut r = population[0].rotations;
                    r[i][j] = n;
                    let new_score = compute_score_standard(&r, 1 << 12);
                    if new_score > population[0].score {
                        found_better = true;
                        population[0].rotations = r;
                        population[0].score = new_score;
                        print!("\r                                  \r");
                        println!(
                            "{}: {}:\n    {:?}",
                            population[0].id, population[0].score, population[0].rotations
                        );
                    }
                }
                if found_better && j < 1 {
                    continue 'foo;
                }
            }
        }
        break;
    }

    println!(
        "\n\nFinal: {}:\n    {:?}",
        population[0].score, population[0].rotations
    );
}

/// Computes the score of a set of rotation constants, which is always between
/// zero (worst) and one (best).
///
/// The score is based on the diffusion of the least-well-diffused input
/// bit.  It takes into account multiple input patterns (random, counting, and
/// single-bit) using least squares.
fn compute_score_standard(rotations: &[[u32; 2]], rounds: usize) -> f64 {
    let forward_mix = |a: &[u64; 4], b: &mut [u64; 4]| {
        *b = *a;
        mix_state(b, rotations);
    };

    let chart_inputs: &[(
        &(dyn Fn(usize, &mut [u64; 4]) + Sync),
        &(dyn Fn(&[u64; 4], &mut [u64; 4]) + Sync),
        usize,
    )] = &[
        (&generate_random, &forward_mix, rounds),
        (&generate_counting, &forward_mix, rounds),
        (&generate_counting_rev, &forward_mix, rounds),
        // Always 256 rounds for this one because it only has 256 possible
        // variations, so more rounds is purely redundant.
        (&generate_single_1_bit, &forward_mix, 256),
    ];

    let mut score: f64 = 0.0;
    for (gen, mix, rounds) in chart_inputs {
        let chart = compute_avalanche_chart(gen, mix, *rounds);

        let a = 256.0 - chart.min_input_bit_diffusion();
        let b = 256.0 - chart.min_input_bit_entropy();

        score -= (a * a) + (b * b);
    }
    let m = chart_inputs.len() as f64 * 256.0 * 256.0 * 2.0;
    score += m;
    score /= m;

    score
}

/// Unused.
///
/// This was an experiment to also optimize for diffusion when doing a round-trip
/// unmix and mix (referred to as a "bounce" here) with a xor of the hash state
/// between the unix and mix.  In the end, this didn't actually provide any
/// meaningful benefit, and it ends up being difficult to balance (in terms of
/// scoring) with the metrics that actually do matter.
#[allow(dead_code)]
fn compute_score_with_bounce(rotations: &[[u32; 2]], rounds: usize) -> f64 {
    let forward_mix = |a: &[u64; 4], b: &mut [u64; 4]| {
        *b = *a;
        mix_state(b, rotations);
    };

    // Flipping almost any of the bits of the hash state between unmixing
    // and mixing ends up being itself a fine mixing function with very good
    // diffusion.
    //
    // However, there are precisely two bits that are stubborn.  These two
    // mix functions test those two cases, so that they can be scored and
    // optimized.
    //
    // In practice this didn't end up really mattering, though, since even
    // though they're "stubborn", even they still end up diffusing the hash
    // state *pretty* well.  And in TentHash's larger design, even if you
    // discount these two bits as lost, a data-generating function would still
    // need to guess 126 bits right, which is vanishingly unlikely to happen in
    // practice.  See TentHash's design rationale document for more details.
    let bounce_0_mix = |a: &[u64; 4], b: &mut [u64; 4]| {
        *b = *a;
        unmix_state(b, rotations);
        b[0] ^= 1 << 63;
        mix_state(b, rotations);
    };
    let bounce_1_mix = |a: &[u64; 4], b: &mut [u64; 4]| {
        *b = *a;
        unmix_state(b, rotations);
        b[1] ^= 1 << 63;
        mix_state(b, rotations);
    };

    let standard_chart_inputs: &[(
        &(dyn Fn(usize, &mut [u64; 4]) + Sync),
        &(dyn Fn(&[u64; 4], &mut [u64; 4]) + Sync),
        usize,
    )] = &[
        (&generate_random, &forward_mix, rounds),
        (&generate_counting, &forward_mix, rounds),
        (&generate_counting_rev, &forward_mix, rounds),
        (&generate_single_1_bit, &forward_mix, 256),
    ];

    let bounce_chart_inputs: &[(
        &(dyn Fn(usize, &mut [u64; 4]) + Sync),
        &(dyn Fn(&[u64; 4], &mut [u64; 4]) + Sync),
        usize,
    )] = &[
        (&generate_random, &bounce_0_mix, rounds),
        (&generate_counting, &bounce_0_mix, rounds),
        (&generate_counting_rev, &bounce_0_mix, rounds),
        (&generate_single_1_bit, &bounce_0_mix, 256),
        (&generate_random, &bounce_1_mix, rounds),
        (&generate_counting, &bounce_1_mix, rounds),
        (&generate_counting_rev, &bounce_1_mix, rounds),
        (&generate_single_1_bit, &bounce_1_mix, 256),
    ];

    let mut standard_score: f64 = 0.0;
    for (gen, mix, rounds) in standard_chart_inputs {
        let chart = compute_avalanche_chart(gen, mix, *rounds);

        let a = 256.0 - chart.min_input_bit_diffusion();
        let b = 256.0 - chart.min_input_bit_entropy();

        standard_score -= (a * a) + (b * b);
    }
    let m = standard_chart_inputs.len() as f64 * 256.0 * 256.0 * 2.0;
    standard_score += m;
    standard_score /= m;

    let mut bounce_score: f64 = 0.0;
    for (gen, mix, rounds) in bounce_chart_inputs {
        let chart = compute_avalanche_chart(gen, mix, *rounds);

        let a = 256.0 - chart.min_input_bit_diffusion();
        let b = 256.0 - chart.min_input_bit_entropy();

        bounce_score -= (a * a) + (b * b);
    }
    let m = bounce_chart_inputs.len() as f64 * 256.0 * 256.0 * 2.0;
    bounce_score += m;
    bounce_score /= m;

    (standard_score + bounce_score) / 2.0
}

/// Core TentHash mixing function, using `rotations` as the rotation constants.
fn mix_state(state: &mut [u64; 4], rotations: &[[u32; 2]]) {
    for rot_pair in rotations.iter() {
        state[0] = state[0].wrapping_add(state[2]);
        state[1] = state[1].wrapping_add(state[3]);
        state[2] = state[2].rotate_left(rot_pair[0]) ^ state[0];
        state[3] = state[3].rotate_left(rot_pair[1]) ^ state[1];

        state.swap(0, 1);
    }
}

/// Inverse of `mix_state()` above.
fn unmix_state(state: &mut [u64; 4], rotations: &[[u32; 2]]) {
    for rot_pair in rotations.iter().rev() {
        state.swap(0, 1);

        state[3] = (state[3] ^ state[1]).rotate_right(rot_pair[1]);
        state[2] = (state[2] ^ state[0]).rotate_right(rot_pair[0]);
        state[1] = state[1].wrapping_sub(state[3]);
        state[0] = state[0].wrapping_sub(state[2]);
    }
}
