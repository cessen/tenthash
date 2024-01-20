//! TentHash is a robust 160-bit non-cryptographic hash function.
//!
//! **WARNING:** TentHash's design is not yet finalized, and digest
//! results may change before 1.0 is declared.
//!
//! TentHash is intended to be used as a reasonably fast but (more
//! importantly) high-quality checksum for data identification.
//! Moreover, it has a simple design that is easy to understand and
//! straightforward to write conforming implementations of.
//!
//! TentHash is explicitly *not* intended to stand up to attacks.  Its
//! otherwise strong collision resistance is only meaningful under
//! non-adversarial conditions.  In other words, like a good tent, it
//! will protect you from the elements, but will do very little to
//! protect you from attackers.
//!
//! This implementation should work on platforms of any endianness,
//! but has only been tested on little endian platforms so far.
//! Running the test suite on a big-endian platform can verify.
//!
//! # Example
//!
//! ```rust
//! # use tenthash::TentHasher;
//! let mut hasher = TentHasher::new();
//! hasher.update("Hello world!");
//! let hash = hasher.finalize();
//!
//! assert_eq!(&hash[..4], &[0x30, 0xd0, 0x8a, 0x79]);
//! ```

#![no_std]

const DIGEST_SIZE: usize = 160 / 8; // Digest size, in bytes.
const BLOCK_SIZE: usize = 256 / 8; // Internal block size of the hash, in bytes.

/// The TentHash hasher.  Processes input bytes and outputs a TentHash digest.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[repr(align(32))]
pub struct TentHasher {
    state: [u64; 4],       // Hash state.
    buf: [u8; BLOCK_SIZE], // Accumulates message data for processing when needed.
    buf_length: usize,     // The number of message bytes currently stored in buf[].
    message_length: u64,   // Accumulates the total message length, in bytes.
}

impl TentHasher {
    pub fn new() -> TentHasher {
        TentHasher {
            state: [
                0x5d6daffc4411a967,
                0xe22d4dea68577f34,
                0xca50864d814cbc2e,
                0x894e29b9611eb173,
            ],
            buf: [0; BLOCK_SIZE],
            buf_length: 0,
            message_length: 0,
        }
    }

    /// Appends data to the data stream being hashed.
    ///
    /// This can be called repeatedly to incrementally append more and more data.
    ///
    /// ```rust
    /// # use tenthash::TentHasher;
    /// // As one chunk.
    /// let mut hasher1 = TentHasher::new();
    /// hasher1.update("Hello world!");
    ///
    /// // As multiple chunks.
    /// let mut hasher2 = TentHasher::new();
    /// hasher2.update("Hello");
    /// hasher2.update(" world!");
    ///
    /// assert_eq!(hasher1.finalize(), hasher2.finalize());
    /// ```
    pub fn update(&mut self, data: impl AsRef<[u8]>) {
        let mut data = data.as_ref();
        self.message_length += data.len() as u64;

        while !data.is_empty() {
            if self.buf_length == 0 && data.len() >= BLOCK_SIZE {
                // Process data directly, skipping the buffer.
                xor_data_into_state(&mut self.state, data);
                mix_state(&mut self.state);
                data = &data[BLOCK_SIZE..];
            } else if self.buf_length == BLOCK_SIZE {
                // Process the filled buffer.
                xor_data_into_state(&mut self.state, &self.buf);
                mix_state(&mut self.state);
                self.buf_length = 0;
            } else {
                // Fill the buffer.
                let n = (BLOCK_SIZE - self.buf_length).min(data.len());
                (&mut self.buf[self.buf_length..(self.buf_length + n)]).copy_from_slice(&data[..n]);
                data = &data[n..];
                self.buf_length += n;
            }
        }
    }

    /// Finalizes the hash and returns the digest.
    pub fn finalize(mut self) -> [u8; DIGEST_SIZE] {
        // Hash the remaining bytes if there are any.
        if self.buf_length > 0 {
            (&mut self.buf[self.buf_length..]).fill(0); // Pad with zeros as needed.
            xor_data_into_state(&mut self.state, &self.buf);
            mix_state(&mut self.state);
        }

        // Incorporate the message length (in bits) and do the
        // final mixing.
        self.state[0] ^= self.message_length * 8;
        mix_state(&mut self.state);
        mix_state(&mut self.state);

        // Get the digest as a byte array and return it.
        let mut digest = [0u8; DIGEST_SIZE];
        digest[0..8].copy_from_slice(&self.state[0].to_le_bytes());
        digest[8..16].copy_from_slice(&self.state[1].to_le_bytes());
        digest[16..20].copy_from_slice(&self.state[2].to_le_bytes()[0..4]);
        return digest;
    }
}

/// Adds message data to the hash state.
///
/// The data must be at least 32 bytes long.  Only the first 32 bytes
/// are added.
#[inline(always)]
fn xor_data_into_state(state: &mut [u64; 4], data: &[u8]) {
    // Convert the data to native endian u64's and add to the
    // hash state.
    assert!(data.len() >= BLOCK_SIZE);
    state[0] ^= u64::from_le_bytes((&data[0..8]).try_into().unwrap());
    state[1] ^= u64::from_le_bytes((&data[8..16]).try_into().unwrap());
    state[2] ^= u64::from_le_bytes((&data[16..24]).try_into().unwrap());
    state[3] ^= u64::from_le_bytes((&data[24..32]).try_into().unwrap());
}

/// Mixes the passed hash state.
///
/// Running this on the hash state once is enough to diffuse the bits
/// equivalently to a full 170-bit diffusion.  Running it twice achieves
/// full 256-bit diffusion.
#[inline(always)]
fn mix_state(state: &mut [u64; 4]) {
    const ROTATIONS: &[[u32; 2]] = &[
        [51, 59],
        [25, 19],
        [8, 10],
        [35, 3],
        [45, 38],
        [61, 32],
        [23, 53],
    ];

    for rot_pair in ROTATIONS.iter() {
        state[0] = state[0].wrapping_add(state[2]);
        state[1] = state[1].wrapping_add(state[3]);
        state[2] = state[2].rotate_left(rot_pair[0]) ^ state[0];
        state[3] = state[3].rotate_left(rot_pair[1]) ^ state[1];

        state.swap(0, 1);
    }
}
