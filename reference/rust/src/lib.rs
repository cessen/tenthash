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
                0x68, 0xc8, 0x21, 0x3b, 0x7a, 0x76, 0xb8, 0xed, 0x26, 0x7d, 0xdd, 0xb3, 0xd8, 0x71,
                0x7b, 0xb3, 0xb6, 0xe7, 0xcc, 0x0a,
            ],
        ),
        (
            &[0],
            [
                0x3c, 0xf6, 0x83, 0x3c, 0xca, 0x9c, 0x4d, 0x5e, 0x21, 0x13, 0x18, 0x57, 0x7b, 0xab,
                0x74, 0xbf, 0x12, 0xa4, 0xf0, 0x90,
            ],
        ),
        (
            b"0123456789",
            [
                0xa7, 0xd3, 0x24, 0xbd, 0xe0, 0xbf, 0x6c, 0xe3, 0x42, 0x77, 0x01, 0x62, 0x8f, 0x0f,
                0x8f, 0xc3, 0x29, 0xc2, 0xa1, 0x16,
            ],
        ),
        (
            b"abcdefghijklmnopqrstuvwxyz",
            [
                0xf1, 0xbe, 0x4b, 0xe1, 0xa0, 0xf9, 0xea, 0xe6, 0x50, 0x0f, 0xb2, 0xf6, 0xb6, 0x4f,
                0x3d, 0xaa, 0x39, 0x90, 0xac, 0x1a,
            ],
        ),
        (
            b"The quick brown fox jumps over the lazy dog.",
            [
                0xde, 0x77, 0xf1, 0xc1, 0x34, 0x22, 0x8b, 0xe1, 0xb5, 0xb2, 0x5c, 0x94, 0x1d, 0x51,
                0x02, 0xf8, 0x7f, 0x3e, 0x6d, 0x39,
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
