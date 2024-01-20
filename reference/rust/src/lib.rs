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
                0x48, 0x01, 0xdc, 0x8f, 0xd9, 0x75, 0x3d, 0xac, 0x45, 0x9d, 0xd9, 0x6b, 0x31, 0x2f,
                0xf8, 0xfc, 0x30, 0xad, 0x29, 0x96,
            ],
        ),
        (
            b"abcdefghijklmnopqrstuvwxyz",
            [
                0xbc, 0xac, 0x70, 0x4f, 0x1e, 0x65, 0xad, 0xfb, 0x5d, 0xe7, 0xd9, 0x66, 0x8c, 0xba,
                0xdc, 0x65, 0x8e, 0x4e, 0x27, 0x23,
            ],
        ),
        (
            b"The quick brown fox jumps over the lazy dog.",
            [
                0x4f, 0xe4, 0x81, 0x74, 0xc1, 0xaa, 0x89, 0x5a, 0x36, 0x8e, 0x5f, 0x05, 0xd5, 0x19,
                0x25, 0x9c, 0x32, 0x20, 0x04, 0xb0,
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
