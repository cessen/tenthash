# TentHash Rust

[![Latest Release][crates-io-badge]][crates-io-url]
[![Documentation][docs-rs-img]][docs-rs-url]

Rust implementation of TentHash, a 160-bit *non-cryptographic* hash function.

**WARNING:** TentHash's design is not yet finalized, and digest results may change before the spec is declared 1.0.

TentHash is a high-quality, reasonably fast, large-output hash.  Its target applications are data fingerprinting, checksums, content-addressable systems, and other use cases that don't tolerate hash collisions.

Importantly, TentHash is explicitly *not* intended to stand up to attacks.  Its robustness against collisions is only meaningful under non-adversarial conditions.  In other words, like a good tent, it will protect you from the elements, but will do very little to protect you from attackers.


## License

This project is licensed under either of

* MIT license (licenses/MIT.txt or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0, (licenses/APACHE-2.0.txt or http://www.apache.org/licenses/LICENSE-2.0)

at your option.


## Contributing

Contributions are absolutely welcome!  Please keep in mind that this crate aims to be:

* no-std and allocation-free.  PRs that use allocation, etc. are very likely to be rejected.
* As small as it reasonably can be, including transitive dependencies.  PRs that pull in dependencies--especially deep dependency trees--are likely to be rejected unless they really pull their weight.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you will be licensed as above (MIT/Apache dual-license), without any additional terms or conditions.


[crates-io-badge]: https://img.shields.io/crates/v/tenthash.svg
[crates-io-url]: https://crates.io/crates/tenthash
[docs-rs-img]: https://docs.rs/tenthash/badge.svg
[docs-rs-url]: https://docs.rs/tenthash
