use nanorand::{Rng, WyRand};

type Bits = u32;
const SIZE: usize = std::mem::size_of::<Bits>() * 8;

// Size needed for the higher-order avalanche test.
//
// This is set to measure avalanche up to order 4 (flipping all combinations of
// up to 4 bits).  You can set `ORDER` higher to test even higher orders, but be
// forewarned that the time and memory requirements rapidly increase.  `ORDER =
// 1` is a standerd avalanche test.
const HIGHER_ORDER_SIZE: usize = {
    const ORDER: usize = 4;

    // The below amounts to just `(1..=ORDER).map(|i| binomial(32, i)).sum()`,
    // but all the niceties that allow this to be written better (even for
    // loops!) aren't allowed in a const context yet.
    let mut combo_count = 0;
    let mut i = 1;
    while i <= ORDER {
        combo_count += binomial(32, i);
        i += 1;
    }
    combo_count
};

pub struct Stats {
    // The number of samples accumulated.  Or put another way, the number of
    // rounds used to generate the chart.
    pub sample_count: usize,

    // For every input bit, the BIC quadrants for each pair of output bits.
    // Note: this table is actually twice as large as it needs to be, because it
    // stores each output pair twice.  We just live with that because the code
    // is easier to write this way, and it doesn't alter the results.
    pub bic_chart: [[[u32; 4]; SIZE * (SIZE - 1)]; SIZE],

    // Each element is a count of the number of bit flips for a given in/out
    // bit set, where "in" can be up to two bits rather than just one. The first
    // 32 rows are single bit flips, and the remainder are all combinations of
    // two-bit flips.
    //
    // This is basically just a more advanced avalanche chart, that accounts for
    // one additional higher-order avalanche check.
    pub avalanche_chart: Vec<[u32; SIZE]>,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            sample_count: 0,
            bic_chart: [[[0; 4]; SIZE * (SIZE - 1)]; SIZE],
            avalanche_chart: vec![[0; SIZE]; HIGHER_ORDER_SIZE],
        }
    }

    /// The diffusion (computed as average inverse bias) of a single row of the
    /// chart, corresponding to a single input bit.
    pub fn row_diffusion(&self, in_bit: usize) -> f64 {
        let norm = 1.0 / self.sample_count as f64;
        self.avalanche_chart[in_bit]
            .iter()
            .map(|&flips| 1.0 - p_to_bias(flips as f64 * norm))
            .sum()
    }

    /// Same as `row_diffusion()`, except computed as Shannon entropy.
    pub fn row_entropy(&self, in_bit: usize) -> f64 {
        let norm = 1.0 / self.sample_count as f64;
        self.avalanche_chart[in_bit]
            .iter()
            .map(|&flips| p_to_entropy(flips as f64 * norm))
            .sum()
    }

    /// The total average bias of all output bits.
    pub fn average_bias(&self) -> f64 {
        let norm = 1.0 / self.sample_count as f64;

        let bias_sum: f64 = self
            .avalanche_chart
            .iter()
            .flatten()
            .map(|&flips| p_to_bias(flips as f64 * norm))
            .sum();
        bias_sum / self.avalanche_chart.iter().flatten().count() as f64
    }

    /// The minimum bias of all output bits.
    pub fn min_bias(&self) -> f64 {
        let norm = 1.0 / self.sample_count as f64;

        let mut min_bias = 0.0f64;
        for &flips in self.avalanche_chart.iter().flatten() {
            let bias = p_to_bias(flips as f64 * norm);
            min_bias = min_bias.min(bias);
        }
        min_bias
    }

    /// The maximum bias of all output bits.
    pub fn max_bias(&self) -> f64 {
        let norm = 1.0 / self.sample_count as f64;

        let mut max_bias = 0.0f64;
        for &flips in self.avalanche_chart.iter().flatten() {
            let bias = p_to_bias(flips as f64 * norm);
            max_bias = max_bias.max(bias);
        }
        max_bias
    }

    /// The diffusion (computed as sum of inverse bias) of the least-well
    /// diffused input bit combination.
    pub fn min_input_bit_combo_diffusion(&self) -> f64 {
        let mut min_diffusion = f64::INFINITY;
        for i in 0..self.avalanche_chart.len() {
            min_diffusion = min_diffusion.min(self.row_diffusion(i));
        }
        min_diffusion
    }

    /// The average diffusion (computed as sum of inverse bias) of all input
    /// bit combinations.
    pub fn avg_input_bit_combo_diffusion(&self) -> f64 {
        let mut avg_diffusion = 0.0f64;
        for i in 0..self.avalanche_chart.len() {
            avg_diffusion += self.row_diffusion(i);
        }
        avg_diffusion / self.avalanche_chart.len() as f64
    }

    /// The diffusion (computed as sum of inverse bias) of the most diffused
    /// input bit combination.
    pub fn max_input_bit_combo_diffusion(&self) -> f64 {
        let mut max_diffusion = 0.0f64;
        for i in 0..self.avalanche_chart.len() {
            max_diffusion = max_diffusion.max(self.row_diffusion(i));
        }
        max_diffusion
    }

    /// Same as `min_input_bit_combo_diffusion()` except computed with Shannon
    /// entropy.
    pub fn min_input_bit_combo_entropy(&self) -> f64 {
        let mut min_entropy = f64::INFINITY;
        for i in 0..self.avalanche_chart.len() {
            min_entropy = min_entropy.min(self.row_entropy(i));
        }
        min_entropy
    }

    /// Same as `avg_input_bit_combo_diffusion()` except computed with Shannon
    /// entropy.
    pub fn avg_input_bit_combo_entropy(&self) -> f64 {
        let mut avg_entropy = 0.0f64;
        for i in 0..self.avalanche_chart.len() {
            avg_entropy += self.row_entropy(i);
        }
        avg_entropy / self.avalanche_chart.len() as f64
    }

    /// Same as `max_input_bit_combo_diffusion()` except computed with Shannon
    /// entropy.
    pub fn max_input_bit_combo_entropy(&self) -> f64 {
        let mut max_entropy = 0.0f64;
        for i in 0..self.avalanche_chart.len() {
            max_entropy = max_entropy.max(self.row_entropy(i));
        }
        max_entropy
    }

    /// Computes the average deviation from the bit independence criterion for a
    /// given input bit.
    ///
    /// For a perfect mixer, the result of flipping a given input bit should
    /// be statistically uncorrelated between any two output bits.  Or in other
    /// words, there are four possible outcomes when considering two bits: both
    /// flip, neither flip, only the first flips, or only the second flips.  All
    /// four outcomes should be equally likely.
    ///
    /// For each pair of output bits, this function computes the relative
    /// difference between the most common and least common of those four
    /// outcomes, mapping between 0.0 (perfect non-correlation, good) and
    /// 1.0 (perfect correlation, bad).  It then takes the average of that
    /// computation over all those pairs.
    ///
    /// This gives a good measure of how statistically independent the effects
    /// of flipping a given input are, with 0.0 being best and 1.0 being worst.
    pub fn row_bic_avg_deviation(&self, in_bit_idx: usize) -> f64 {
        let bic = self.bic_chart[in_bit_idx];

        let mut sum = 0.0;
        for [a, b, c, d] in bic.iter() {
            let min = *a.min(b).min(c).min(d);
            let max = *a.max(b).max(c).max(d);

            sum += (max - min) as f64 / max as f64;
        }
        sum / (SIZE * (SIZE - 1)) as f64
    }

    /// The minimum of `row_bic_avg_deviation()` across all input bits.
    pub fn min_bic_deviation(&self) -> f64 {
        let mut n = 999.0_f64;
        for i in 0..SIZE {
            n = n.min(self.row_bic_avg_deviation(i));
        }
        n
    }

    /// The average of `row_bic_avg_deviation()` across all input bits.
    pub fn avg_bic_deviation(&self) -> f64 {
        let mut n = 0.0;
        for i in 0..SIZE {
            n += self.row_bic_avg_deviation(i);
        }
        n / SIZE as f64
    }

    /// The maximum of `row_bic_avg_deviation()` across all input bits.
    pub fn max_bic_deviation(&self) -> f64 {
        let mut n = 0.0_f64;
        for i in 0..SIZE {
            n = n.max(self.row_bic_avg_deviation(i));
        }
        n
    }

    /// Prints a summary report of the chart statistics.
    #[allow(dead_code)]
    pub fn print_report(&self) {
        println!(
            "    Bias (lower is better):
        Min: {:0.2}
        Avg: {:0.2}
        Max: {:0.2}
    Diffusion (higher is better):
        Min: {:0.1} bits
        Avg: {:0.1} bits 
        Max: {:0.1} bits
    Diffusion Entropy (higher is better):
        Min: {:0.1} bits
        Avg: {:0.1} bits
        Max: {:0.1} bits
    BIC deviation (lower is better):
        Min: {:0.4}
        Avg: {:0.4}
        Max: {:0.4}",
            self.min_bias(),
            self.average_bias(),
            self.max_bias(),
            self.min_input_bit_combo_diffusion(),
            self.avg_input_bit_combo_diffusion(),
            self.max_input_bit_combo_diffusion(),
            self.min_input_bit_combo_entropy(),
            self.avg_input_bit_combo_entropy(),
            self.max_input_bit_combo_entropy(),
            self.min_bic_deviation(),
            self.avg_bic_deviation(),
            self.max_bic_deviation(),
        );
    }
}

/// Computes mixing statiastics for a given mix/absorb function, using a
/// provided input generator.
///
/// - `generate_input`: function that takes a seed and generates an input block.
///   The result should be deterministic based on the seed.  Note that the seed
///   starts from zero, and simply increments each round.
/// - `mix`: function that takes input and mixes it to produce an output. Note
///   that any data in the passed output parameter should *not* be used by this
///   function, and should instead be ignored and overwritten.  In other words,
///   it is purely an out paramater, not an in-out parameter.
/// - `rounds`: how many test rounds to perform to produce the estimated chart.
pub fn compute_stats<F1, F2>(generate_input: F1, mix: F2, rounds: usize) -> Stats
where
    F1: Fn(usize) -> Bits,
    F2: Fn(&Bits, &mut Bits),
{
    let mut chart = Stats::new();

    for round in 0..rounds {
        if (round % (10000 / HIGHER_ORDER_SIZE).max(1)) == 0 {
            use std::io::Write;
            print!(
                "\r                                \rRound {} / {}",
                round, rounds
            );
            let _ = std::io::stdout().flush();
        }

        let input = generate_input(round);
        let mut output = 0usize as Bits;
        mix(&input, &mut output);

        // Avalanche.
        for flip_idx in 0..HIGHER_ORDER_SIZE {
            let mut input_tweaked = input;
            input_tweaked ^= bit_combinations(flip_idx + 1);
            let mut output_tweaked = 0usize as Bits;
            mix(&input_tweaked, &mut output_tweaked);

            let flips = output ^ output_tweaked;

            // Avalanche.
            for out_bit_idx in 0..SIZE {
                let flipped = (flips & (1 << out_bit_idx)) != 0;
                chart.avalanche_chart[flip_idx][out_bit_idx] += flipped as u32;
            }
        }

        // Bit independence criterion.
        for in_bit_idx in 0..SIZE {
            let mut input_tweaked = input;
            input_tweaked ^= 1 << in_bit_idx;
            let mut output_tweaked = 0usize as Bits;
            mix(&input_tweaked, &mut output_tweaked);

            let flips = output ^ output_tweaked;

            for i in 0..(SIZE - 1) {
                let flips_s = flips.rotate_left(i as u32 + 1);

                let both = flips & flips_s;
                let neither = !flips & !flips_s;
                let only_left = flips & !flips_s;
                let only_right = !flips & flips_s;

                for j in 0..SIZE {
                    let mask = 1 << j;
                    let k = j * (SIZE - 1) + i;
                    chart.bic_chart[in_bit_idx][k][0] += ((both & mask) != 0) as u32;
                    chart.bic_chart[in_bit_idx][k][1] += ((neither & mask) != 0) as u32;
                    chart.bic_chart[in_bit_idx][k][2] += ((only_left & mask) != 0) as u32;
                    chart.bic_chart[in_bit_idx][k][3] += ((only_right & mask) != 0) as u32;
                }
            }
        }

        chart.sample_count += 1;
    }

    print!("\r                                \r");

    chart
}

pub fn p_to_bias(p: f64) -> f64 {
    (p * 2.0 - 1.0).abs()
}

pub fn p_to_entropy(p: f64) -> f64 {
    if p <= 0.0 || p >= 1.0 {
        0.0
    } else {
        let q = 1.0 - p;
        -(p * p.log2()) - (q * q.log2())
    }
}

//-------------------------------------------------------------

/// Generates a random data block.
#[allow(dead_code)]
pub fn generate_random(seed: usize) -> Bits {
    fn mix64(mut n: u64) -> u64 {
        // Break zero sensitivity.
        n ^= 0x7be355f7c2e736d2;

        // http://zimbry.blogspot.ch/2011/09/better-bit-mixing-improving-on.html
        // (variant "Mix13")
        n ^= n >> 30;
        n = n.wrapping_mul(0xbf58476d1ce4e5b9);
        n ^= n >> 27;
        n = n.wrapping_mul(0x94d049bb133111eb);
        n ^= n >> 31;

        n
    }

    WyRand::new_seed(mix64(seed as u64)).generate::<Bits>()
}

/// Computes the nth bit combination, ordered by first no set bits, then all
/// combinations of one set bit, then two set bits, and so on.
#[allow(dead_code)]
pub fn bit_combinations(index: usize) -> u32 {
    let mut n = index & 0xffffffff;
    let mut bits = 0;
    let mut combos = binomial(32, bits);
    while n >= combos {
        n -= combos;
        bits += 1;
        combos = binomial(32, bits);
    }

    let mut result = 0;
    let mut t = 32;
    while t > 0 && bits > 0 {
        let y = if t > bits { binomial(t - 1, bits) } else { 0 };

        if n >= y {
            result |= 1 << (t - 1);
            n -= y;
            bits -= 1;
        }

        t -= 1;
    }

    result
}

const fn binomial(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }

    if k == 0 {
        1
    } else if k > (n / 2) {
        binomial(n, n - k)
    } else {
        n * binomial(n - 1, k - 1) / k
    }
}
