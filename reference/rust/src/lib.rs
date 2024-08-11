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
        [16, 28],
        [14, 57],
        [11, 22],
        [35, 34],
        [57, 16],
        [59, 40],
        [44, 13],
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
                0xe0, 0xf5, 0x79, 0xd7, 0x7b, 0x71, 0xae, 0x9a, 0x5a, 0xac, 0xf6, 0x76, 0x42, 0xf4,
                0x2a, 0x6f, 0x6d, 0x8b, 0x57, 0xe4,
            ],
        ),
        (
            &[0],
            [
                0x6c, 0x3c, 0xc3, 0xb2, 0xda, 0x66, 0x3b, 0xaf, 0x11, 0x3d, 0xe2, 0xe8, 0xcb, 0xd9,
                0x23, 0x16, 0x3a, 0x87, 0x84, 0x7a,
            ],
        ),
        (
            b"0123456789",
            [
                0x4b, 0x49, 0xac, 0xd5, 0x9a, 0x54, 0x82, 0xf3, 0x9a, 0x0e, 0x7f, 0x89, 0x35, 0x14,
                0x9d, 0xf9, 0x4e, 0xe6, 0x91, 0x85,
            ],
        ),
        (
            b"abcdefghijklmnopqrstuvwxyz",
            [
                0x66, 0xa2, 0xbd, 0x21, 0x0f, 0x2c, 0xdf, 0x08, 0xaa, 0xb5, 0xfe, 0x66, 0x27, 0x48,
                0x4c, 0x15, 0x7c, 0x7c, 0x98, 0xb9,
            ],
        ),
        (
            b"The quick brown fox jumps over the lazy dog.",
            [
                0x71, 0x5c, 0xd5, 0x73, 0x97, 0x68, 0x59, 0x66, 0x3c, 0x2e, 0xba, 0xf2, 0x1d, 0xac,
                0xd7, 0xbd, 0x78, 0xe1, 0x1a, 0x71,
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
