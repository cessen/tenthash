/// TentHash reference implementation in Rust.

const BLOCK_SIZE: usize = 256 / 8; // Block size, in bytes.
const DIGEST_SIZE: usize = 160 / 8; // Digest size, in bytes.

pub fn hash(input_data: &[u8]) -> [u8; DIGEST_SIZE] {
    let mut state: [u64; 4] = [
        0x5d6daffc4411a967,
        0xe22d4dea68577f34,
        0xca50864d814cbc2e,
        0x894e29b9611eb173,
    ];

    // Process the input data.
    for chunk in input_data.chunks(BLOCK_SIZE) {
        // Copy the chunk into a zeroed-out buffer.  When the chunk is
        // smaller than 256 bits this pads it out to 256 bits with zeros.
        let mut buffer = [0u8; BLOCK_SIZE];
        (&mut buffer[..chunk.len()]).copy_from_slice(chunk);

        // Add the buffer into the hash state.
        state[0] = state[0].wrapping_add(u64::from_le_bytes((&buffer[0..8]).try_into().unwrap()));
        state[1] = state[1].wrapping_add(u64::from_le_bytes((&buffer[8..16]).try_into().unwrap()));
        state[2] = state[2].wrapping_add(u64::from_le_bytes((&buffer[16..24]).try_into().unwrap()));
        state[3] = state[3].wrapping_add(u64::from_le_bytes((&buffer[24..32]).try_into().unwrap()));

        mix_state(&mut state);
    }

    // Finalize.
    state[0] ^= (input_data.len() * 8) as u64;
    mix_state(&mut state);
    mix_state(&mut state);

    // Convert the hash state into a digest.
    let mut digest = [0u8; DIGEST_SIZE];
    digest[0..8].copy_from_slice(&state[0].to_le_bytes());
    digest[8..16].copy_from_slice(&state[1].to_le_bytes());
    digest[16..20].copy_from_slice(&state[2].to_le_bytes()[0..4]);

    return digest;
}

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

#[cfg(test)]
mod tests {
    use super::{hash, DIGEST_SIZE};

    // (input, digest)
    const TEST_VECTORS: &[(&[u8], [u8; DIGEST_SIZE])] = &[
        (
            &[],
            [
                0x52, 0x06, 0xdf, 0x94, 0x90, 0xca, 0xa9, 0x09, 0x3a, 0xd6, 0x19, 0x71, 0xa0, 0xfc,
                0xb2, 0xaa, 0x61, 0x15, 0xd5, 0x42,
            ],
        ),
        (
            &[0],
            [
                0xb9, 0x76, 0x9a, 0xf5, 0xa7, 0xf4, 0x21, 0xc0, 0xbb, 0xbe, 0x10, 0x63, 0xea, 0x69,
                0x5d, 0x8e, 0x13, 0xe6, 0xa1, 0x6d,
            ],
        ),
        (
            b"0123456789",
            [
                0x6a, 0x51, 0x32, 0x03, 0xd8, 0x5d, 0x60, 0xe6, 0x4c, 0xe3, 0xd1, 0x71, 0xa2, 0x80,
                0x98, 0xa4, 0x96, 0xf0, 0x12, 0x25,
            ],
        ),
        (
            b"abcdefghijklmnopqrstuvwxyz",
            [
                0x0f, 0xf9, 0xf1, 0xc4, 0x9a, 0x26, 0x4a, 0xce, 0xa3, 0x67, 0x39, 0xd9, 0x1f, 0x9a,
                0xc0, 0x44, 0xd5, 0x8f, 0x5d, 0x64,
            ],
        ),
        (
            b"The quick brown fox jumps over the lazy dog.",
            [
                0xa7, 0xe5, 0x56, 0x8c, 0xe1, 0xcb, 0x6d, 0x59, 0x33, 0xf2, 0xf7, 0x65, 0x4f, 0x69,
                0xf3, 0x09, 0xb3, 0x6d, 0xac, 0xac,
            ],
        ),
    ];

    #[test]
    fn test_vectors() {
        for (data, digest) in TEST_VECTORS.iter() {
            assert_eq!(hash(data), *digest);
        }
    }
}
