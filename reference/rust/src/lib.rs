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
    for block in input_data.chunks(BLOCK_SIZE) {
        // Copy the block into a zeroed-out buffer.  When the block is
        // smaller than 256 bits this pads it out to 256 bits with zeros.
        let mut buffer = [0u8; BLOCK_SIZE];
        (&mut buffer[..block.len()]).copy_from_slice(block);

        // Incorporate the block into the hash state.
        state[0] ^= u64::from_le_bytes((&buffer[0..8]).try_into().unwrap());
        state[1] ^= u64::from_le_bytes((&buffer[8..16]).try_into().unwrap());
        state[2] ^= u64::from_le_bytes((&buffer[16..24]).try_into().unwrap());
        state[3] ^= u64::from_le_bytes((&buffer[24..32]).try_into().unwrap());

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

    state[0] ^= 0x2ea6370ac28ae776;
    state[1] ^= 0x5abb00d71a7850cc;

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
                0x71, 0xa1, 0x25, 0x21, 0xe9, 0x27, 0x61, 0x73, 0x53, 0xca, 0xdc, 0xbb, 0x22, 0x47,
                0x53, 0x22, 0x03, 0x6e, 0x57, 0x52,
            ],
        ),
        (
            &[0],
            [
                0x3e, 0x45, 0xee, 0x52, 0x42, 0x2a, 0xa2, 0xf9, 0x35, 0xc7, 0x0a, 0x22, 0x06, 0xb4,
                0x3a, 0xfc, 0x92, 0xdc, 0xe3, 0x37,
            ],
        ),
        (
            b"0123456789",
            [
                0x9c, 0x21, 0xdd, 0xc6, 0x5d, 0xcc, 0x3a, 0xd2, 0x98, 0x2c, 0x32, 0x1c, 0x71, 0x74,
                0xc4, 0x75, 0xdc, 0x70, 0x5e, 0x03,
            ],
        ),
        (
            b"abcdefghijklmnopqrstuvwxyz",
            [
                0xbf, 0x1d, 0x10, 0x62, 0x7b, 0x6f, 0x80, 0x94, 0xe3, 0x33, 0x11, 0x1e, 0x64, 0x6a,
                0x37, 0xf7, 0xed, 0xcf, 0xab, 0xb6,
            ],
        ),
        (
            b"The quick brown fox jumps over the lazy dog.",
            [
                0x21, 0x22, 0x7d, 0x4c, 0xb3, 0xe7, 0x52, 0xb0, 0x5a, 0x38, 0x6d, 0x68, 0x58, 0x06,
                0x72, 0xec, 0x8f, 0x26, 0xd6, 0x77,
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
