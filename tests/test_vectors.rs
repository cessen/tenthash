use tenthash::TentHash;

const TEST_VECTORS: &[(&[u8], &str)] = &[
    (&[], "e0d4e0a2608a8741e349fa1ea0263fedbd65f66d"),
    (&[0], "6e5f483d20443bb6e70c300b0a5aa64ce36d3467"),
    (&[0, 0], "ffaf1c6954edb55a7ac10c16b6f309c8e1cc7b5c"),
    (b"0123456789", "f12f795967313e9a0e822edaa307c3d7b7d19ce3"),
    (b"abcdefghijklmnopqrstuvwxyz", "8f578c05439217eeac0dc46d7df2805f91ffad99"),
    (b"The quick brown fox jumps over the lazy dog.", "0be19c6dc03f6800743e41c70f0ee0c2d75bad67"),
    (
        b"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
        "6faf47daac8a767a1d7ed6da36cbe50616a1b83a",
    ),
];

/// Returns a printable hex string version of the digest.
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
fn one_chunk() {
    for (data, digest) in TEST_VECTORS.iter().copied() {
        let mut hasher = TentHash::new();
        hasher.update(data);
        assert_eq!(digest_to_string(&hasher.finalize()), digest);
    }
}

#[test]
fn multi_chunk() {
    for chunk_size in 1..260 {
        for (data, digest) in TEST_VECTORS.iter().copied() {
            if data.len() >= chunk_size {
                let mut hasher = TentHash::new();
                for chunk in data.chunks(chunk_size) {
                    hasher.update(chunk);
                }
                assert_eq!(digest_to_string(&hasher.finalize()), digest);
            }
        }
    }
}
