# TentHash v0.2

A robust 160-bit *non-cryptographic* hash function.

- [TentHash Specification v0.2](docs/specification.md) **WARNING:** TentHash's design is not yet finalized, and digest results may change before the specification is declared 1.0.
- [Design Rationale Document](docs/design_rationale.md)


TentHash is a reasonably fast but (more importantly) high-quality checksum for data identification.  Moreover, it has a simple portable design that is easy to audit, doesn't require special hardware instructions, and is easy to write conforming independent implementations of.

TentHash is explicitly *not* intended to stand up to attacks.  Its otherwise strong collision resistance is only meaningful under non-adversarial conditions.  In other words, like a good tent, it will protect you from the elements, but will do very little to protect you from attackers.




## Why yet another hash?

TentHash aims to fill a gap in hash design between cryptographic and non-cryptographic hash functions.  Cryptographic hash functions produce message digests with strong collision resistance, but sacrifice speed and/or simplicity for security against attackers.  Most non-cryptographic hash functions, on the other hand, sacrifice either simplicity or collision resistance for speed.

TentHash is squarely non-cryptographic, and is designed with the following goals in order of priority:

1. **Strong collision resistance.**  For all practical purposes, it should be safe to assume that different pieces of (legitimate) data will never have colliding hashes.
2. **Simplicity.**  It should be easy to understand and straightforward to write conforming implementations.
3. **Reasonably fast.**  It doesn't need to win any speed competitions, but its speed should be measured in GB/sec not MB/sec.

Additionally, TentHash aims to be *well documented*.  Independent implementations shouldn't have to figure out the hash design from sprawling source code, and the design justifications should documented.


## License

This project is licensed under either of

* MIT license (licenses/MIT.txt or https://opensource.org/licenses/MIT)
* Apache License, Version 2.0, (licenses/APACHE-2.0.txt or https://www.apache.org/licenses/LICENSE-2.0)

at your option.

The files under `docs/` are additionally released under

* Creative Commons Zero (licenses/CC0.txt or https://creativecommons.org/publicdomain/zero/1.0/legalcode)


## Contributing

Contributions are absolutely welcome!  Especially (but not limited to):

* Audits of the hash design for (non-cryptographic) weaknesses.
* Implementations of the current specification in other programming languages.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you will be licensed as above (MIT/Apache/CC0), without any additional terms or conditions.
