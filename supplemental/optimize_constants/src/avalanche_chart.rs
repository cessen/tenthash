use nanorand::{Rng, WyRand};

const IN_SIZE: usize = 256;
const OUT_SIZE: usize = 256;

pub struct AvalancheChart {
    // The number of samples accumulated.  Or put another way, the number of
    // rounds used to generate the chart.
    pub sample_count: usize,

    // Each element is a count of the number of bit flips for a given in/out
    // bit pairing.
    pub chart: [[u32; OUT_SIZE]; IN_SIZE],
}

impl AvalancheChart {
    pub fn new() -> Self {
        Self {
            sample_count: 0,
            chart: [[0; OUT_SIZE]; IN_SIZE],
        }
    }

    /// The diffusion (computed as average inverse bias) of a single row of the
    /// chart, corresponding to a single input bit.
    pub fn row_diffusion(&self, in_bit: usize) -> f64 {
        let norm = 1.0 / self.sample_count as f64;
        self.chart[in_bit]
            .iter()
            .map(|&flips| 1.0 - p_to_bias(flips as f64 * norm))
            .sum()
    }

    /// Same as `row_diffusion()`, except computed as Shannon entropy.
    pub fn row_entropy(&self, in_bit: usize) -> f64 {
        let norm = 1.0 / self.sample_count as f64;
        self.chart[in_bit]
            .iter()
            .map(|&flips| p_to_entropy(flips as f64 * norm))
            .sum()
    }

    /// The total average bias of all in-out bit pairs.
    pub fn average_bias(&self) -> f64 {
        let norm = 1.0 / self.sample_count as f64;

        let bias_sum: f64 = self
            .chart
            .iter()
            .flatten()
            .map(|&flips| p_to_bias(flips as f64 * norm))
            .sum();
        bias_sum / (IN_SIZE * OUT_SIZE) as f64
    }

    /// The minimum bias of all in-out bit pairs.
    pub fn min_bias(&self) -> f64 {
        let norm = 1.0 / self.sample_count as f64;

        let mut min_bias = 0.0f64;
        for &flips in self.chart.iter().flatten() {
            let bias = p_to_bias(flips as f64 * norm);
            min_bias = min_bias.min(bias);
        }
        min_bias
    }

    /// The maximum bias of all in-out bit pairs.
    pub fn max_bias(&self) -> f64 {
        let norm = 1.0 / self.sample_count as f64;

        let mut max_bias = 0.0f64;
        for &flips in self.chart.iter().flatten() {
            let bias = p_to_bias(flips as f64 * norm);
            max_bias = max_bias.max(bias);
        }
        max_bias
    }

    /// The diffusion (computed as sum of inverse bias) of the least-well
    /// diffused input bit.
    pub fn min_input_bit_diffusion(&self) -> f64 {
        let mut min_diffusion = f64::INFINITY;
        for i in 0..IN_SIZE {
            min_diffusion = min_diffusion.min(self.row_diffusion(i));
        }
        min_diffusion
    }

    /// The average diffusion (computed as sum of inverse bias) of all input
    /// bits.
    pub fn avg_input_bit_diffusion(&self) -> f64 {
        let mut avg_diffusion = 0.0f64;
        for i in 0..IN_SIZE {
            avg_diffusion += self.row_diffusion(i);
        }
        avg_diffusion / IN_SIZE as f64
    }

    /// The diffusion (computed as sum of inverse bias) of the most diffused
    /// input bit.
    pub fn max_input_bit_diffusion(&self) -> f64 {
        let mut max_diffusion = 0.0f64;
        for i in 0..IN_SIZE {
            max_diffusion = max_diffusion.max(self.row_diffusion(i));
        }
        max_diffusion
    }

    /// Same as `min_input_bit_diffusion()` except computed with Shannon
    /// entropy.
    pub fn min_input_bit_entropy(&self) -> f64 {
        let mut min_entropy = f64::INFINITY;
        for i in 0..IN_SIZE {
            min_entropy = min_entropy.min(self.row_entropy(i));
        }
        min_entropy
    }

    /// Same as `avg_input_bit_diffusion()` except computed with Shannon
    /// entropy.
    pub fn avg_input_bit_entropy(&self) -> f64 {
        let mut avg_entropy = 0.0f64;
        for i in 0..IN_SIZE {
            avg_entropy += self.row_entropy(i);
        }
        avg_entropy / IN_SIZE as f64
    }

    /// Same as `max_input_bit_diffusion()` except computed with Shannon
    /// entropy.
    pub fn max_input_bit_entropy(&self) -> f64 {
        let mut max_entropy = 0.0f64;
        for i in 0..IN_SIZE {
            max_entropy = max_entropy.max(self.row_entropy(i));
        }
        max_entropy
    }

    /// Prints a summary report of the chart statistics.
    #[allow(dead_code)]
    pub fn print_report(&self) {
        println!(
            "    Bias:
        Min: {:0.2}
        Avg: {:0.2}
        Max: {:0.2}
    Input Bit Diffusion:
        Min: {:0.1} bits
        Avg: {:0.1} bits 
        Max: {:0.1} bits
    Input Bit Diffusion Entropy:
        Min: {:0.1} bits
        Avg: {:0.1} bits
        Max: {:0.1} bits",
            self.min_bias(),
            self.average_bias(),
            self.max_bias(),
            self.min_input_bit_diffusion(),
            self.avg_input_bit_diffusion(),
            self.max_input_bit_diffusion(),
            self.min_input_bit_entropy(),
            self.avg_input_bit_entropy(),
            self.max_input_bit_entropy(),
        );
    }
}

/// Computes an avalanche chart for a given mix/absorb function, using a provided
/// input generator.
///
/// - `generate_input`: function that takes a seed and generates an input block.
///   The result should be deterministic based on the seed.  Note that the seed
///   starts from zero, and simply increments each round.
/// - `mix`: function that takes input and mixes it to produce an output. Note
///   that any data in the passed output parameter should *not* be used by this
///   function, and should instead be ignored and overwritten.  In other words,
///   it is purely an out paramater, not an in-out parameter.
/// - `rounds`: how many test rounds to perform to produce the estimated chart.
pub fn compute_avalanche_chart<F1, F2>(generate_input: F1, mix: F2, rounds: usize) -> AvalancheChart
where
    F1: Fn(usize, &mut [u64; 4]) + Sync,
    F2: Fn(&[u64; 4], &mut [u64; 4]) + Sync,
{
    let mut chart = AvalancheChart::new();

    for round in 0..rounds {
        let mut input = [0u64; 4];
        let mut output = [0u64; 4];

        generate_input(round, &mut input);

        mix(&input, &mut output);
        for in_bit_idx in 0..IN_SIZE {
            let mut input_tweaked = input;
            input_tweaked[in_bit_idx / 64] ^= 1 << (in_bit_idx % 64);
            let mut output_tweaked = [0u64; 4];
            mix(&input_tweaked, &mut output_tweaked);

            for out_bit_idx in 0..OUT_SIZE {
                let i = out_bit_idx / 64;
                let mask = 1 << (out_bit_idx % 64);
                let flipped = (output[i] & mask) != (output_tweaked[i] & mask);

                chart.chart[in_bit_idx][out_bit_idx] += flipped as u32;
            }
        }

        chart.sample_count += 1;
    }

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
pub fn generate_random(seed: usize, out: &mut [u64; 4]) {
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

    let mut rng = WyRand::new_seed(mix64(seed as u64));
    out[0] = rng.generate::<u64>();
    out[1] = rng.generate::<u64>();
    out[2] = rng.generate::<u64>();
    out[3] = rng.generate::<u64>();
}

/// Generates a data block with all zero bits except one.
pub fn generate_single_1_bit(seed: usize, out: &mut [u64; 4]) {
    let bit_idx = seed % 256;
    let i = bit_idx / 64;
    let item = 1 << (bit_idx % 64);
    out.fill(0);
    out[i] = item;
}

/// Generates a data block with the lowest bits simply counting up as an
/// incrementing integer.
pub fn generate_counting(seed: usize, out: &mut [u64; 4]) {
    out.fill(0);
    out[0] = seed as u64;
}

/// Generates a data block with the *highest* bits simply counting up as an
/// incrementing integer, with reversed bits.
pub fn generate_counting_rev(seed: usize, out: &mut [u64; 4]) {
    out.fill(0);
    out[3] = (seed as u64).reverse_bits();
}
