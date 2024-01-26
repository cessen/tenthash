# TentHash v0.3

A robust 160-bit *non-cryptographic* hash function.

- [TentHash Specification v0.3](docs/specification.md) **WARNING:** TentHash's design is not yet finalized, and digest results may change before the specification is declared 1.0.
- [Design Rationale Document](docs/design_rationale.md)

TentHash is a reasonably fast but (more importantly) high-quality checksum for data identification.  Moreover, it has a simple portable design that is easy to audit, doesn't require special hardware instructions, and is easy to write conforming independent implementations of.

TentHash is explicitly *not* intended to stand up to attacks.  Its otherwise robust<sup>1</sup> collision resistance is only meaningful under non-adversarial conditions.  In other words, like a good tent, it will protect you from the elements, but will do very little to protect you from attackers.


## Why yet another hash?

TentHash aims to fill a gap in hash design between cryptographic and non-cryptographic hash functions.  Cryptographic hash functions produce message digests with strong collision resistance, but sacrifice speed and/or simplicity for security against attackers.  Most non-cryptographic hash functions, on the other hand, sacrifice simplicity and/or collision resistance for speed.

TentHash is squarely non-cryptographic, and is designed with the following goals in order of priority:

1. **Robust<sup>1</sup> collision resistance.**  For all practical purposes, it should be safe to assume that different pieces of (legitimate) data will never have colliding hashes.
2. **Simplicity and portability.**  It should be easy to understand and straightforward to write conforming implementations, without need for special hardware instructions.
3. **Reasonably fast.**  It doesn't need to win any speed competitions, but its speed should be measured in GB/sec, not MB/sec, on typical hardware.

Additionally, TentHash aims to be *well documented*.  Independent implementations shouldn't have to figure out the hash design from sprawling source code, and the design rationale/justifications should be publicly available and collected in one place.


## Comparison with other hashes.

Below is a comparison of hashes that have outputs large enough to be used as message digests.  Some cryptographic hashes are included at the bottom for reference.  Data throughput is measured single-threaded on an AMD Ryzen Threadripper 3960X.

| Name                                  | Digest size          | Min diffusion per block | Blocks per full diffusion        | Data throughput<sup>2</sup> |
|---------------------------------------|----------------------|-------------------------|----------------------------------|-----------------------------|
| TentHash                              | 160 bits<sup>3</sup> | full                    | 1 block                          | 5.4 GB/s                    |
| -                                     |                      |                         |                                  |                             |
| MeowHash v0.5                         | 128 bits             | ~32 bits                | ~6 blocks                        | 35.1 GB/s                   |
| xxHash3 (128-bit)                     | 128 bits             | ~33 bits                | Never fully diffuses<sup>4</sup> | 38.3 GB/s                   |
| Murmur3 (x64 128-bit)                 | 128 bits             | ~32 bits                | ~6 blocks                        | 7.6 GB/s                    |
| FNV-1a (128-bit)                      | 128 bits             | ~12 bits<sup>5</sup>    | Never fully diffuses             | 0.42 GB/s                   |
| CityHash128 / FarmHash128<sup>6</sup> | 128 bits             | ~3 bits                 | ~3 blocks                        | 14.8 GB/s                   |
| MetroHash128                          | 128 bits             | ~3 bits                 | ~22 blocks                       | 15.6 GB/s                   |
| -                                     |                      |                         |                                  |                             |
| SHA2-256                              | 256 bits             | full                    | 1 block                          | 0.25 GB/s                   |
| Blake2b                               | 256 bits             | full                    | 1 block                          | 0.62 GB/s                   |
| Blake3                                | 256 bits             | full                    | 1 block                          | 1.2 GB/s                    |

The "Min diffusion per block" column is a measure of how well the internal hash state is diffused between incorporating input blocks.  Details of this metric are discussed in the rotation constants section of TentHash's [design rationale document](docs/design_rationale.md).  This metric is not feasible to determine from testing final hash outputs—it requires testing the internal state of the hash function's inner loop.

**CityHash** and **MetroHash** both have extremely poor minimum diffusion per block.  It's not 100% clear how this impacts collision resistance, but it is *suspicious*.  Other hashes should be preferred in contexts where good collision resistance is critical.  But if you do use one of these, CityHash is a better choice because it diffuses after just a few additional blocks.

**Murmur3** is a so-so choice here.  Its min diffusion isn't the worst of the bunch, but it's not especially good.  And it's certainly not conservative.  Both **MeowHash v0.5** and **xxHash3** are similar to Murmur3 in that respect, but substantially faster.  However, unlike Murmur3, xxHash3's implementation is very complex and relies on manually written SIMD code to achieve its high speeds.  And MeowHash relies on both manually written SIMD code *and* AES hardware instructions for its high speeds.

**TentHash** is the only non-cryptographic hash in the list that is conservative about hash quality and collision resistance.  It is also the simplest of those hashes to implement and port, and has reasonable (though not extreme) performance.  Finally, it is the only non-cryptographic hash in the list with a thorough write up and justification of its design.


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


## Footnotes

1. The term "robust" is used to avoid confusion, since "strong collision resistance" has a specific cryptographic meaning.  But in the colloquial rather than technical sense, "strong collision resistance" is the intended meaning here.
2.  This does not reflect small-input performance, since TentHash's target use case is message digests, not hash maps.  TentHash's data throughput is relatively worse on small inputs.
3. For non-cryptographic hashes, a 160-bit digest isn't meaningfully better than a 128-bit digest in the vast majority of practical applications.  See the design rationale document for how TentHash ended up at 160 bits.
4. For xxhash3 this isn't *quite* as bad as it sounds, because although the inner accumulation loop never fully diffuses on its own, there is a loop outside of that which does diffuse the hash state.  However, that diffusion is only run once every several blocks, so it is still a weakness.
5. For FNV, "per block" isn't easy to define, because it hashes a byte at a time rather than a block at a time.  The listed diffusion number is what you get if you pretend it takes input data in blocks of 128 bits.
6. CityHash128 and FarmHash128 use exactly the same construction for the part of the hash relevant to this metric.  (They may even just be identical hashes...?  I didn't check that far.)
