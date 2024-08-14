//! A program that runs TentHash in reverse to intentionally create hash
//! collisions.
//!
//! The purpose of this is mainly for fun, but also to clearly demonstrate that
//! TentHash shouldn't be used under adversarial circumstances.  It is not a
//! cryptographic hash.

fn main() {
    let collisions = generate_collisions([0u8; 20], 10);

    println!("Here are ten pieces of data that each produce a hash of all zeros:");
    for data in &collisions {
        println!("    {:?}", data);
    }

    println!("And here are the corresponding hashes, to verify:");
    for data in &collisions {
        println!("    {:?}", hash(data));
    }
}

fn hash(data: &[u8]) -> [u8; 20] {
    let mut hasher = tenthash::TentHasher::new();
    hasher.update(data);
    hasher.finalize()
}

/// Generates `count` messages that when hashed by TentHash produce the given target output.
fn generate_collisions(target: [u8; 20], count: usize) -> Vec<Vec<u8>> {
    let mut colliding_data = Vec::new();
    for i in 0..count {
        let mut state = [
            u64::from_le_bytes((&target[0..8]).try_into().unwrap()),
            u64::from_le_bytes((&target[8..16]).try_into().unwrap()),
            u32::from_le_bytes((&target[16..20]).try_into().unwrap()) as u64,
            i as u64,
        ];

        unmix_state(&mut state);
        unmix_state(&mut state);
        state[0] ^= 256;

        unmix_state(&mut state);
        state[0] ^= 0x5d6daffc4411a967;
        state[1] ^= 0xe22d4dea68577f34;
        state[2] ^= 0xca50864d814cbc2e;
        state[3] ^= 0x894e29b9611eb173;

        let mut data = Vec::new();
        data.extend_from_slice(&state[0].to_le_bytes());
        data.extend_from_slice(&state[1].to_le_bytes());
        data.extend_from_slice(&state[2].to_le_bytes());
        data.extend_from_slice(&state[3].to_le_bytes());

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
    fn generate_collisions_01() {
        let h = hash(b"Hello world!");
        let cols = generate_collisions(h, 20);
        for data in cols {
            let h2 = hash(&data);
            assert_eq!(h, h2);
        }
    }
}
