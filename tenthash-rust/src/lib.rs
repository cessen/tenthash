//! TentHash is a strong 160-bit *non-cryptographic* hash function.
//!
//! **WARNING:** TentHash's design is not yet finalized, and digest
//! results may change before 1.0 is declared.  Please do not rely on
//! this (yet) for persistent, long-term checksums.
//!
//! TentHash is intended to be used as a reasonably fast but (more
//! importantly) high-quality checksum for data identification.
//! Moreover, it has a simple portable design that is easy to audit,
//! doesn't require special hardware instructions, and is easy to write
//! conforming independent implementations of.
//!
//! TentHash is explicitly *not* intended to stand up to attacks.  In
//! fact, attacks are *quite easy* to mount against it.  Its otherwise
//! strong collision resistance is only meaningful under non-adversarial
//! conditions.  In other words, like a good tent, it will protect you
//! from the elements, but will do very little to protect you from
//! attackers.
//!
//! This implementation should work on platforms of any endianness,
//! but has only been tested on little endian platforms so far.
//! Running the test suite on a big-endian platform can verify.

#![no_std]

const DIGEST_SIZE: usize = 160 / 8; // Digest size, in bytes.
const BLOCK_SIZE: usize = 256 / 8; // Internal block size of the hash, in bytes.
const UPDATE_MIX_ROUNDS: usize = 6; // Number of mix rounds after each block of data is added.
const FINALIZE_MIX_ROUNDS: usize = 12; // Number of mix rounds used to finalize the hash.

/// Processes input bytes and outputs a TentHash digest.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[repr(align(32))]
pub struct TentHash {
    state: [u64; 4],       // Hash state.
    buf: [u8; BLOCK_SIZE], // Accumulates message data for processing when needed.
    buf_length: usize,     // The number of message bytes currently stored in buf[].
    message_length: u64,   // Accumulates the total message length, in bytes.
}

impl TentHash {
    pub fn new() -> TentHash {
        TentHash {
            state: [
                0xe2b8d3b67882709f,
                0x045e21ec46bcea22,
                0x51ea37fa96fbae67,
                0xf5d94991b6b9b944,
            ],
            buf: [0; BLOCK_SIZE],
            buf_length: 0,
            message_length: 0,
        }
    }

    /// Updates the hash with input data.
    pub fn update(&mut self, data: impl AsRef<[u8]>) {
        let mut data = data.as_ref();
        self.message_length += data.len() as u64;

        while !data.is_empty() {
            if self.buf_length == 0 && data.len() >= BLOCK_SIZE {
                // Process data directly, skipping the buffer.
                add_data_to_state(&mut self.state, data);
                mix_state(&mut self.state, UPDATE_MIX_ROUNDS);
                data = &data[BLOCK_SIZE..];
            } else if self.buf_length == BLOCK_SIZE {
                // Process the filled buffer.
                add_data_to_state(&mut self.state, &self.buf);
                mix_state(&mut self.state, UPDATE_MIX_ROUNDS);
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
            add_data_to_state(&mut self.state, &self.buf);
            mix_state(&mut self.state, UPDATE_MIX_ROUNDS);
        }

        // Incorporate the message length (in bits) and do the
        // final mixing.
        self.state[0] ^= self.message_length * 8;
        mix_state(&mut self.state, FINALIZE_MIX_ROUNDS);

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
fn add_data_to_state(state: &mut [u64; 4], data: &[u8]) {
    // Convert the data to native endian u64's and xor into the
    // hash state.
    assert!(data.len() >= BLOCK_SIZE);
    state[0] ^= u64::from_le_bytes((&data[0..8]).try_into().unwrap());
    state[1] ^= u64::from_le_bytes((&data[8..16]).try_into().unwrap());
    state[2] ^= u64::from_le_bytes((&data[16..24]).try_into().unwrap());
    state[3] ^= u64::from_le_bytes((&data[24..32]).try_into().unwrap());
}

/// Mixes the passed hash state.
///
/// Inspired by Skein's MIX function and permutation approach.
///
/// 6 rounds is enough for an effective 160 bits of diffusion.  9 rounds
/// is enough for full diffusion.
#[inline(always)]
fn mix_state(state: &mut [u64; 4], rounds: usize) {
    // Rotation constants.  These have been optimized to maximize the
    // minimum diffusion at 6 rounds of mixing.
    const ROTATIONS: &[[u32; 2]] = &[[31, 25], [5, 48], [20, 34], [21, 57], [11, 41], [18, 33]];

    for round in 0..rounds {
        let rot = ROTATIONS[round % ROTATIONS.len()];

        // MIX function.
        state[0] = state[0].wrapping_add(state[2]);
        state[2] = state[2].rotate_left(rot[0]) ^ state[0];
        state[1] = state[1].wrapping_add(state[3]);
        state[3] = state[3].rotate_left(rot[1]) ^ state[1];

        state.swap(2, 3);
    }
}
