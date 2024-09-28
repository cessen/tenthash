//! A program that runs TentHash in reverse to intentionally create hash
//! collisions with specific target hash outputs.
//!
//! The purpose of this is mainly for fun, but also to clearly demonstrate that
//! TentHash shouldn't be used under adversarial conditions.  It is not a
//! cryptographic hash.

use nanorand::{Rng, WyRand};

fn main() {
    let target_hash = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ];

    let collisions = generate_colliding_messages(target_hash, 10);

    println!("Here are ten pieces of data that all produce the same hash:");
    for data in &collisions {
        println!("    {:?}", data);
    }

    println!("\nAnd here are the corresponding hashes, to verify:");
    for data in &collisions {
        println!("    {:?}", tenthash::hash(data));
    }
}

/// Generates `count` messages that when hashed by TentHash produce the given
/// target output.
fn generate_colliding_messages(target: [u8; 20], count: usize) -> Vec<Vec<u8>> {
    let mut rng = WyRand::new();
    let mut colliding_data = Vec::new();

    // You can set this to whatever size you like, for longer or shorter
    // messages.
    let payload_size_in_blocks = 1;

    for _ in 0..count {
        let mut data = Vec::new();

        let mut state = [
            u64::from_le_bytes((&target[0..8]).try_into().unwrap()),
            u64::from_le_bytes((&target[8..16]).try_into().unwrap()),
            u32::from_le_bytes((&target[16..20]).try_into().unwrap()) as u64,
            rng.generate::<u64>(),
        ];

        unmix_state(&mut state);
        unmix_state(&mut state);
        state[0] ^= 256 * (1 + payload_size_in_blocks);

        for _ in 0..payload_size_in_blocks {
            unmix_state(&mut state);

            // This can be any data.  In this case, we randomly generate it.
            let data0 = rng.generate::<u64>();
            let data1 = rng.generate::<u64>();
            let data2 = rng.generate::<u64>();
            let data3 = rng.generate::<u64>();

            state[0] ^= data0;
            state[1] ^= data1;
            state[2] ^= data2;
            state[3] ^= data3;
            data.extend_from_slice(&data3.to_be_bytes());
            data.extend_from_slice(&data2.to_be_bytes());
            data.extend_from_slice(&data1.to_be_bytes());
            data.extend_from_slice(&data0.to_be_bytes());
        }

        unmix_state(&mut state);
        state[0] ^= 0x5d6daffc4411a967;
        state[1] ^= 0xe22d4dea68577f34;
        state[2] ^= 0xca50864d814cbc2e;
        state[3] ^= 0x894e29b9611eb173;

        data.extend_from_slice(&state[3].to_be_bytes());
        data.extend_from_slice(&state[2].to_be_bytes());
        data.extend_from_slice(&state[1].to_be_bytes());
        data.extend_from_slice(&state[0].to_be_bytes());

        data.reverse();

        colliding_data.push(data);
    }

    colliding_data
}

fn unmix_state(state: &mut [u64; 4]) {
    const ROTATIONS: &[[u32; 2]] = &[
        [16, 28],
        [14, 57],
        [11, 22],
        [35, 34],
        [57, 16],
        [59, 40],
        [44, 13],
    ];

    for rot_pair in ROTATIONS.iter().rev() {
        state.swap(0, 1);
        state[3] = (state[3] ^ state[1]).rotate_right(rot_pair[1]);
        state[2] = (state[2] ^ state[0]).rotate_right(rot_pair[0]);
        state[1] = state[1].wrapping_sub(state[3]);
        state[0] = state[0].wrapping_sub(state[2]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn unmix() {
        for i in 0..1024 {
            let mut a = [0u64; 4];
            for j in 0..4 {
                a[j] = i + j as u64;
            }
            let mut b = a;

            unmix_state(&mut b);
            assert!(a != b);
            mix_state(&mut b);
            assert_eq!(a, b);

            mix_state(&mut b);
            assert!(a != b);
            unmix_state(&mut b);
            assert_eq!(a, b);
        }
    }

    #[test]
    fn generate_colliding_messages_01() {
        let h = tenthash::hash(b"Hello world!");
        let cols = generate_colliding_messages(h, 20);
        for data in cols {
            let h2 = tenthash::hash(&data);
            assert_eq!(h, h2);
        }
    }
}
