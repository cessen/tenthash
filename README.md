# TentHash v0.1

A strong 160-bit *non-cryptographic* hash function.

**WARNING:** TentHash's design is not yet finalized, and digest results may change before the specification is declared 1.0.  Please do not rely on this (yet) for persistent, long-term checksums.  Please see the (not-yet-final) [specification document](docs/specification.md) for more details.

TentHash is a reasonably fast but (more importantly) high-quality checksum for data identification.  Moreover, it has a simple portable design that is easy to audit, doesn't require special hardware instructions, and is easy to write conforming independent implementations of.

TentHash is explicitly *not* intended to stand up to attacks.  In fact, attacks are *quite easy* to mount against it.  Its otherwise strong collision resistance is only meaningful under non-adversarial conditions.  In other words, like a good tent, it will protect you from the elements, but will do very little to protect you from attackers.


## Why yet another hash?

TentHash aims to fill a gap in hash design between cryptographic and non-cryptographic hash functions.  Cryptographic hash functions produce messsage digests with strong collision resistance, but sacrifice speed and/or simplicity for security against attackers.  Most non-cryptographic hash functions, on the other hand, sacrifice collision resistance *even in the absence of attackers* and/or simplicity for speed.

TentHash is squarely non-cryptographic, and aims to be strongly collision resistant *in absence of attackers*.  It is designed with the following priorities (in order of importance):

1. **Strong collision resistance.**  For all practical purposes, it should be safe to assume that different pieces of (legitimate) data will never have colliding hashes.
2. **Simplicity.**  It shouldn't require diagrams to understand, and it should be straightforward to write conforming implementations for just about any platform.  Its design should also be *documented* and *easy to audit*.
3. **Reasonably fast.**  It doesn't need to be break-neck fast, but it should be able to process 100 GB of data in seconds, not minutes, without resorting to multi-threading.

For more details on the rationale behind TentHash's design, please see the [Design Rationale](docs/design_rationale.md) document.


## License

This project is licensed under either of

* MIT license (licenses/MIT.txt or https://opensource.org/licenses/MIT)
* Apache License, Version 2.0, (licenses/APACHE-2.0.txt or https://www.apache.org/licenses/LICENSE-2.0)

at your option.

The files under `docs/` are additionally released under

* Creative Commons Zero (licenses/CC0.txt or https://creativecommons.org/publicdomain/zero/1.0/legalcode)


## Contributing

Contributions are absolutely welcome!  Especially (but not limited to):

* Audits of the hash design for (non-security) weaknesses.
* Implementations of the current specification in other programming languages.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you will be licensed as above (MIT/Apache/CC0), without any additional terms or conditions.
