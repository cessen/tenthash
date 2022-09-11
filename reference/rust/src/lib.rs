/// TentHash reference implementation in Rust.

const BLOCK_SIZE: usize = 256 / 8; // Block size, in bytes.
const DIGEST_SIZE: usize = 160 / 8; // Digest size, in bytes.

pub fn hash(input_data: &[u8]) -> [u8; DIGEST_SIZE] {
    let mut state: [u64; 4] = [
        0xe2b8d3b67882709f,
        0x045e21ec46bcea22,
        0x51ea37fa96fbae67,
        0xf5d94991b6b9b944,
    ];

    // Process the input data.
    for chunk in input_data.chunks(BLOCK_SIZE) {
        // Copy the chunk into a zeroed-out buffer.  When the chunk is
        // smaller than 256 bits this pads it out to 256 bits with zeros.
        let mut buffer = [0u8; BLOCK_SIZE];
        (&mut buffer[..chunk.len()]).copy_from_slice(chunk);

        // Xor the chunk/buffer into the hash state.
        state[0] ^= u64::from_le_bytes((&buffer[0..8]).try_into().unwrap());
        state[1] ^= u64::from_le_bytes((&buffer[8..16]).try_into().unwrap());
        state[2] ^= u64::from_le_bytes((&buffer[16..24]).try_into().unwrap());
        state[3] ^= u64::from_le_bytes((&buffer[24..32]).try_into().unwrap());

        mix_state(&mut state, 6);
    }

    // Finalize.
    state[0] ^= (input_data.len() * 8) as u64;
    mix_state(&mut state, 12);

    // Convert the hash state into a 160-bit digest.
    let mut digest = [0u8; DIGEST_SIZE];
    digest[0..8].copy_from_slice(&state[0].to_le_bytes());
    digest[8..16].copy_from_slice(&state[1].to_le_bytes());
    digest[16..20].copy_from_slice(&state[2].to_le_bytes()[0..4]);

    return digest;
}

fn mix_state(state: &mut [u64; 4], rounds: usize) {
    // Rotation constants.
    const ROTATIONS: &[[u32; 2]] = &[[31, 25], [5, 48], [20, 34], [21, 57], [11, 41], [18, 33]];

    for round in 0..rounds {
        let rot = ROTATIONS[round % ROTATIONS.len()];

        state[0] = state[0].wrapping_add(state[2]);
        state[2] = state[2].rotate_left(rot[0]) ^ state[0];
        state[1] = state[1].wrapping_add(state[3]);
        state[3] = state[3].rotate_left(rot[1]) ^ state[1];

        state.swap(2, 3);
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
                0xe0, 0xd4, 0xe0, 0xa2, 0x60, 0x8a, 0x87, 0x41, 0xe3, 0x49, 0xfa, 0x1e, 0xa0, 0x26,
                0x3f, 0xed, 0xbd, 0x65, 0xf6, 0x6d,
            ],
        ),
        (
            &[0],
            [
                0x6e, 0x5f, 0x48, 0x3d, 0x20, 0x44, 0x3b, 0xb6, 0xe7, 0x0c, 0x30, 0x0b, 0x0a, 0x5a,
                0xa6, 0x4c, 0xe3, 0x6d, 0x34, 0x67,
            ],
        ),
        (
            b"0123456789",
            [
                0xf1, 0x2f, 0x79, 0x59, 0x67, 0x31, 0x3e, 0x9a, 0x0e, 0x82, 0x2e, 0xda, 0xa3, 0x07,
                0xc3, 0xd7, 0xb7, 0xd1, 0x9c, 0xe3,
            ],
        ),
        (
            b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            [
                0x9f, 0x4c, 0x56, 0xc9, 0x9c, 0x8f, 0xb9, 0x71, 0xbf, 0xbf, 0xcb, 0xcf, 0x9c, 0x62,
                0x96, 0xc8, 0x5f, 0xba, 0x77, 0x33,
            ],
        ),
        (
            b"The quick brown fox jumps over the lazy dog.",
            [
                0x0b, 0xe1, 0x9c, 0x6d, 0xc0, 0x3f, 0x68, 0x00, 0x74, 0x3e, 0x41, 0xc7, 0x0f, 0x0e,
                0xe0, 0xc2, 0xd7, 0x5b, 0xad, 0x67,
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
