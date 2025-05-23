//! TentHash is a high-quality, non-cryptographic, 160-bit hash function.  It is
//! also portable, easy to implement, and reasonably fast.
//!
//! TentHash's target applications are data fingerprinting, content-addressable
//! systems, and other use cases that don't tolerate hash collisions.
//!
//! Importantly, TentHash is explicitly *not* intended to stand up to attacks,
//! and should never be used where the choice of hash function has security
//! considerations.  Its robustness against collisions is only meaningful under
//! non-adversarial conditions.  In other words, like a good tent, it will
//! protect you from the elements, but will do very little to protect you from
//! attackers.
//!
//!
//! # Example
//!
//! ```rust
//! let hash = tenthash::hash("Hello world!");
//!
//! assert_eq!(&hash[..4], &[0x15, 0x5f, 0xa, 0x35]);
//! ```

#![no_std]
#![forbid(unsafe_code)]

const DIGEST_SIZE: usize = 160 / 8; // Digest size, in bytes.
const BLOCK_SIZE: usize = 256 / 8; // Internal block size of the hash, in bytes.

/// Computes TentHash in one go for a contiguous slice of data.
///
/// # Example
///
/// ```rust
/// let hash = tenthash::hash("Hello world!");
///
/// assert_eq!(&hash[..4], &[0x15, 0x5f, 0xa, 0x35]);
/// ```
pub fn hash(data: impl AsRef<[u8]>) -> [u8; DIGEST_SIZE] {
    let mut state = [
        0x5d6daffc4411a967,
        0xe22d4dea68577f34,
        0xca50864d814cbc2e,
        0x894e29b9611eb173,
    ];

    let mut data = data.as_ref();
    let message_bit_length = data.len() as u64 * 8;

    // Process full-size chunks.
    while data.len() >= BLOCK_SIZE {
        xor_data_into_state(&mut state, data);
        mix_state(&mut state);
        data = &data[BLOCK_SIZE..];
    }

    // Process any remaining data if needed.
    if !data.is_empty() {
        let mut buffer = [0u8; BLOCK_SIZE];
        (&mut buffer[..data.len()]).copy_from_slice(data);
        xor_data_into_state(&mut state, &buffer);
        mix_state(&mut state);
    }

    // Incorporate the message length (in bits) and do the
    // final mixing.
    state[0] ^= message_bit_length;
    mix_state(&mut state);
    mix_state(&mut state);

    // Get the digest as a byte array and return it.
    let mut digest = [0u8; DIGEST_SIZE];
    digest[0..8].copy_from_slice(&state[0].to_le_bytes());
    digest[8..16].copy_from_slice(&state[1].to_le_bytes());
    digest[16..20].copy_from_slice(&state[2].to_le_bytes()[0..4]);
    return digest;
}

/// Computes TentHash incrementally, taking input data in chunks.
///
/// The hash output is unaffected by how the input data is split into chunks.
/// For example, the data can be passed a byte at a time or all at once, and the
/// hash output will be the same.
///
/// However, chunk size does affect performance, with larger chunks being
/// faster.
///
/// # Example
///
/// ```rust
/// # use tenthash::TentHash;
/// // As one chunk.
/// let mut hasher1 = TentHash::new();
/// hasher1.update("Hello world!");
/// let hash1 = hasher1.finalize();
///
/// // As multiple chunks.
/// let mut hasher2 = TentHash::new();
/// hasher2.update("Hello");
/// hasher2.update(" world!");
/// let hash2 = hasher2.finalize();
///
/// assert_eq!(&hash1[..4], &[0x15, 0x5f, 0xa, 0x35]);
/// assert_eq!(hash1, hash2);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct TentHash {
    state: [u64; 4],       // Hash state.
    buf: [u8; BLOCK_SIZE], // Accumulates message data for processing.
    buf_length: usize,     // The number of message bytes currently stored in buf[].
    message_length: u64,   // Accumulates the total message length, in bytes.
}

impl TentHash {
    pub fn new() -> TentHash {
        TentHash {
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
    /// Call this repeatedly to incrementally append more and more data.
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

/// Xor message data into the hash state.
///
/// The data must be at least 32 bytes long, and only the first 32 bytes are
/// used.
#[inline(always)]
fn xor_data_into_state(state: &mut [u64; 4], data: &[u8]) {
    // Convert the data to native endian u64's and xor into the hash state.
    assert!(data.len() >= BLOCK_SIZE);
    state[0] ^= u64::from_le_bytes((&data[0..8]).try_into().unwrap());
    state[1] ^= u64::from_le_bytes((&data[8..16]).try_into().unwrap());
    state[2] ^= u64::from_le_bytes((&data[16..24]).try_into().unwrap());
    state[3] ^= u64::from_le_bytes((&data[24..32]).try_into().unwrap());
}

/// Mixes the passed hash state.
///
/// Running this on the hash state once results in 179 bits of diffusion.
/// Running it twice achieves full 256-bit diffusion.
#[inline(always)]
fn mix_state(state: &mut [u64; 4]) {
    const ROTATIONS: &[[u32; 2]] = &[
        [16, 28],
        [14, 57],
        [11, 22],
        [35, 34],
        [57, 16],
        [59, 40],
        [44, 13],
    ];

    for rot_pair in ROTATIONS.iter() {
        state[0] = state[0].wrapping_add(state[2]);
        state[2] = state[2].rotate_left(rot_pair[0]) ^ state[0];
        state[1] = state[1].wrapping_add(state[3]);
        state[3] = state[3].rotate_left(rot_pair[1]) ^ state[1];

        state.swap(0, 1);
    }
}

/// A helper trait with convenience methods for TentHash's output digest.
pub trait DigestExt {
    /// Truncates the digest to 128 bits, returned as an array of 16 bytes.
    fn to_16_bytes(self) -> [u8; 16];

    /// Truncates the digest to 128 bits, returned as a [`u128`].
    ///
    /// The digest bytes are interpreted as little endian.
    fn to_u128(self) -> u128;
}

impl DigestExt for [u8; 20] {
    #[inline(always)]
    fn to_16_bytes(self) -> [u8; 16] {
        (&self[0..16]).try_into().unwrap()
    }

    #[inline(always)]
    fn to_u128(self) -> u128 {
        u128::from_le_bytes(self.to_16_bytes())
    }
}
