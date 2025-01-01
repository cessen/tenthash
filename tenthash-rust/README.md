# TentHash Rust

[![Latest Release][crates-io-badge]][crates-io-url]
[![Documentation][docs-rs-img]][docs-rs-url]

Rust implementation of [TentHash](https://github.com/cessen/tenthash).  TentHash is a high-quality, non-cryptographic, 160-bit hash function.  It is also portable, easy to implement, and reasonably fast.

TentHash's target applications are data fingerprinting, content-addressable systems, and other use cases that don't tolerate hash collisions.

Importantly, TentHash is explicitly *not* intended to stand up to attacks, and should never be used where the hash function itself has security considerations.  Its robustness against collisions is only meaningful under non-adversarial conditions.  In other words, like a good tent, it will protect you from the elements, but will do very little to protect you from attackers.


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
