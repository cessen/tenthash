use tenthash::TentHasher;

const TEST_VECTORS: &[(&[u8], &str)] = &[
    (&[], "e0f579d77b71ae9a5aacf67642f42a6f6d8b57e4"),
    (&[0], "6c3cc3b2da663baf113de2e8cbd923163a87847a"),
    (b"0123456789", "4b49acd59a5482f39a0e7f8935149df94ee69185"),
    (b"abcdefghijklmnopqrstuvwxyz", "66a2bd210f2cdf08aab5fe6627484c157c7c98b9"),
    (b"The quick brown fox jumps over the lazy dog.", "715cd573976859663c2ebaf21dacd7bd78e11a71"),
    (
        b"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
        "034aa1b52c0dbc25424d9e001fca39c458652647",
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
        let mut hasher = TentHasher::new();
        hasher.update(data);
        assert_eq!(digest_to_string(&hasher.finalize()), digest);
    }
}

#[test]
fn multi_chunk() {
    for chunk_size in 1..1024 {
        for (data, digest) in TEST_VECTORS.iter().copied() {
            if data.len() >= chunk_size {
                let mut hasher = TentHasher::new();
                for chunk in data.chunks(chunk_size) {
                    hasher.update(chunk);
                }
                assert_eq!(digest_to_string(&hasher.finalize()), digest);
            }
        }
    }
}
