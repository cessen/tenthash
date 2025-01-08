use tenthash::TentHasher;

const TEST_VECTORS: &[(&[u8], &str)] = &[
    (&[], "68c8213b7a76b8ed267dddb3d8717bb3b6e7cc0a"),
    (&[0], "3cf6833cca9c4d5e211318577bab74bf12a4f090"),
    (b"0123456789", "a7d324bde0bf6ce3427701628f0f8fc329c2a116"),
    (b"abcdefghijklmnopqrstuvwxyz", "f1be4be1a0f9eae6500fb2f6b64f3daa3990ac1a"),
    (b"This string is exactly 32 bytes.", "f7c5e4763d89bddce33e97712b712d869aabcfe9"),
    (b"The quick brown fox jumps over the lazy dog.", "de77f1c134228be1b5b25c941d5102f87f3e6d39"),
    (
        b"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
        "53da1e3920a9e5743065f28acaa2a93c51389b3d",
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
fn single_call() {
    for (data, digest) in TEST_VECTORS.iter().copied() {
        assert_eq!(digest_to_string(&tenthash::hash(data)), digest);
    }
}

#[test]
fn streaming_one_chunk() {
    for (data, digest) in TEST_VECTORS.iter().copied() {
        let mut hasher = TentHasher::new();
        hasher.update(data);
        assert_eq!(digest_to_string(&hasher.finalize()), digest);
    }
}

#[test]
fn streaming_multi_chunk() {
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

#[test]
fn test_hasher_trait() {
    use core::hash::Hasher;

    for (data, _) in TEST_VECTORS.iter() {
        let mut hasher1 = TentHasher::new();
        hasher1.write(data);
        let hash1 = hasher1.finish();

        let mut hasher2 = TentHasher::new();
        hasher2.write(data);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2, "Hasher should produce consistent results");
    }
}

#[test]
fn test_build_hasher() {
    use core::hash::{BuildHasher, Hasher};
    let builder = tenthash::TentHashBuilder;

    for (data, _) in TEST_VECTORS.iter() {
        let mut hasher1 = builder.build_hasher();
        let mut hasher2 = builder.build_hasher();

        hasher1.write(data);
        hasher2.write(data);

        assert_eq!(hasher1.finish(), hasher2.finish(),
            "BuildHasher should create consistent hashers");
    }
}

#[test]
fn test_hasher_write_chunks() {
    use core::hash::Hasher;

    for (data, _) in TEST_VECTORS.iter() {
        let mut hasher1 = TentHasher::new();
        hasher1.write(data);
        let hash1 = hasher1.finish();

        let mut hasher2 = TentHasher::new();
        if !data.is_empty() {
            let mid = (data.len() + 1) / 2;
            hasher2.write(&data[..mid]);
            hasher2.write(&data[mid..]);
        } else {
            hasher2.write(data);
        }
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2,
            "Hash should be independent of how data is chunked");
    }
}
