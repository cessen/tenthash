use tenthash::TentHasher;

const TEST_VECTORS: &[(&[u8], &str)] = &[
    (&[], "5206df9490caa9093ad61971a0fcb2aa6115d542"),
    (&[0], "b9769af5a7f421c0bbbe1063ea695d8e13e6a16d"),
    (b"0123456789", "4801dc8fd9753dac459dd96b312ff8fc30ad2996"),
    (b"abcdefghijklmnopqrstuvwxyz", "bcac704f1e65adfb5de7d9668cbadc658e4e2723"),
    (b"The quick brown fox jumps over the lazy dog.", "4fe48174c1aa895a368e5f05d519259c322004b0"),
    (
        b"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
        "f9d38490ca5a880b08dd75edc3985c0a2a61999e",
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
    for chunk_size in 1..260 {
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
