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

#![cfg_attr(not(test), no_std)]

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

    /// Finalizes the hash and returnd the digest.
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

#[cfg(test)]
mod test {
    use super::*;

    fn hash(data: &[u8]) -> [u8; DIGEST_SIZE] {
        let mut h = TentHash::new();
        h.update(data);
        h.finalize()
    }

    /// Returns the printable hex string version of a digest.
    pub fn digest_to_string(digest: &[u8]) -> String {
        fn low_bits_to_char(n: u8) -> char {
            match n {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                10 => 'a',
                11 => 'b',
                12 => 'c',
                13 => 'd',
                14 => 'e',
                15 => 'f',
                _ => unreachable!(),
            }
        }

        let mut s = String::new();
        for byte in digest.iter() {
            s.push(low_bits_to_char(byte >> 4u8));
            s.push(low_bits_to_char(byte & 0b00001111));
        }
        s
    }

    #[test]
    fn hash_empty() {
        let correct_digest = "e0d4e0a2608a8741e349fa1ea0263fedbd65f66d";
        assert_eq!(digest_to_string(&hash(&[])), correct_digest);
    }

    #[test]
    fn hash_zero() {
        let correct_digest = "6e5f483d20443bb6e70c300b0a5aa64ce36d3467";
        assert_eq!(digest_to_string(&hash(&[0u8])), correct_digest);
    }

    #[test]
    fn hash_string_01() {
        let s = "0123456789";
        let correct_digest = "f12f795967313e9a0e822edaa307c3d7b7d19ce3";
        assert_eq!(digest_to_string(&hash(s.as_bytes())), correct_digest);
    }

    #[test]
    fn hash_string_02() {
        let s = "abcdefghijklmnopqrstuvwxyz";
        let correct_digest = "8f578c05439217eeac0dc46d7df2805f91ffad99";
        assert_eq!(digest_to_string(&hash(s.as_bytes())), correct_digest);
    }

    #[test]
    fn hash_string_03() {
        let s = "The quick brown fox jumps over the lazy dog.";
        let correct_digest = "0be19c6dc03f6800743e41c70f0ee0c2d75bad67";
        assert_eq!(digest_to_string(&hash(s.as_bytes())), correct_digest);
    }

    #[test]
    fn hash_string_04() {
        let s = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
        let correct_digest = "6faf47daac8a767a1d7ed6da36cbe50616a1b83a";
        assert_eq!(digest_to_string(&hash(s.as_bytes())), correct_digest);
    }

    #[test]
    fn hash_multi_part_processing() {
        let test_string1 =
            "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor";
        let test_string2 = " incididunt ut l";
        let test_string3 = "abore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat ";
        let test_string4 = "cup";
        let test_string5 =
            "idatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
        let correct_digest = "6faf47daac8a767a1d7ed6da36cbe50616a1b83a";

        let mut hasher = TentHash::new();
        hasher.update(test_string1.as_bytes());
        hasher.update(test_string2.as_bytes());
        hasher.update(test_string3.as_bytes());
        hasher.update(test_string4.as_bytes());
        hasher.update(test_string5.as_bytes());
        let digest = hasher.finalize();

        assert_eq!(digest_to_string(&digest), correct_digest);
    }

    #[test]
    fn hash_length() {
        // We're testing here to make sure the length of the data properly
        // affects the hash.  Internally in the hash, the last block of data
        // is padded with zeros, so here we're forcing that last block to be
        // all zeros, and only changing the length of input.
        let len_0 = &[];
        let len_1 = &[0u8];
        let len_2 = &[0u8, 0];

        assert_eq!(
            digest_to_string(&hash(len_0)),
            "e0d4e0a2608a8741e349fa1ea0263fedbd65f66d",
        );
        assert_eq!(
            digest_to_string(&hash(len_1)),
            "6e5f483d20443bb6e70c300b0a5aa64ce36d3467",
        );
        assert_eq!(
            digest_to_string(&hash(len_2)),
            "ffaf1c6954edb55a7ac10c16b6f309c8e1cc7b5c",
        );
    }
}
