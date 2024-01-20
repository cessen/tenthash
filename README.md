# TentHash v0.3

A robust 160-bit *non-cryptographic* hash function.

- [TentHash Specification v0.3](docs/specification.md) **WARNING:** TentHash's design is not yet finalized, and digest results may change before the specification is declared 1.0.
- [Design Rationale Document](docs/design_rationale.md)


TentHash is a reasonably fast but (more importantly) high-quality checksum for data identification.  Moreover, it has a simple portable design that is easy to audit, doesn't require special hardware instructions, and is easy to write conforming independent implementations of.

TentHash is explicitly *not* intended to stand up to attacks.  Its otherwise robust<sup>1</sup> collision resistance is only meaningful under non-adversarial conditions.  In other words, like a good tent, it will protect you from the elements, but will do very little to protect you from attackers.



## Why yet another hash?

TentHash aims to fill a gap in hash design between cryptographic and non-cryptographic hash functions.  Cryptographic hash functions produce message digests with strong collision resistance, but sacrifice speed and/or simplicity for security against attackers.  Most non-cryptographic hash functions, on the other hand, sacrifice either simplicity or collision resistance for speed.

TentHash is squarely non-cryptographic, and is designed with the following goals in order of priority:

1. **Robust<sup>1</sup> collision resistance.**  For all practical purposes, it should be safe to assume that different pieces of (legitimate) data will never have colliding hashes.
2. **Simplicity and portability.**  It should be easy to understand and straightforward to write conforming implementations, without need for special hardware instructions.
3. **Reasonably fast.**  It doesn't need to win any speed competitions, but its speed should be measured in GB/sec, not MB/sec, on typical hardware.

Additionally, TentHash aims to be *well documented*.  Independent implementations shouldn't have to figure out the hash design from sprawling source code, and the design rationale/justifications should be publicly available and collected in one place.


## Comparison with other hashes.

Below is a comparison of hashes that have outputs large enough to be used as message digests.  Some cryptographic hashes are included at the bottom for reference.  Data throughput is measured single-threaded on an AMD Ryzen Threadripper 3960X.

| Name                  | Digest size          | Min diffusion per block | Blocks per full diffusion | Data throughput<sup>2</sup> |
|-----------------------|----------------------|-------------------------|---------------------------|------------------|
| TentHash              | 160 bits<sup>3</sup> | full                    | 1 block                   | 5.4 GB/s         |
| MeowHash              | 128 bits             | full                    | 1 block                   | 35.1 GB/s        |
| -                     |                      |                         |                           |                  |
| xxHash3 (128-bit)     | 128 bits             | ~75 bits                | 4 blocks                  | 38.3 GB/s        |
| Murmur3 (x64 128-bit) | 128 bits             | ~32 bits                | 6 blocks                  | 7.6 GB/s         |
| FNV-1a (128-bit)      | 128 bits             | 12-89 bits<sup>4</sup>  | Never fully diffuses      | 0.42 GB/s        |
| CityHash128           | 128 bits             | ~3 bits                 | 3 blocks                  | 14.8 GB/s        |
| MetroHash128          | 128 bits             | ~3 bits                 | ~22 blocks                | 15.6 GB/s        |
| -                     |                      |                         |                           |                  |
| SHA2-256              | 256 bits             | full                    | 1 block                   | 0.25 GB/s        |
| Blake2b               | 256 bits             | full                    | 1 block                   | 0.62 GB/s        |
| Blake3                | 256 bits             | full                    | 1 block                   | 1.2 GB/s         |

The "Min diffusion per block" column is a measure of how well the internal hash state is diffused between incorporating input blocks.  Details of this metric are discussed in the rotation constants section of TentHash's [design rationale document](docs/design_rationale.md).  This metric is not feasible to determine from testing final hash outputs--it requires testing the internal state of the hash function's inner loop.

**CityHash** and **MetroHash** both have extremely poor minimum diffusion per block.  It's not 100% clear how this impacts collision resistance, but it is *suspicious*, and these hashes probably shouldn't be used in contexts where good collision resistance is critical.  But if you do use one of them, CityHash is a better choice because it diffuses after just a few additional blocks.

**Murmur3** is a so-so choice here.  Its min diffusion isn't the worst of the bunch, but it's not especially good.  And it's certainly not conservative.  **xxHash3** is similar to Murmur3 in that respect, but is substantially faster.  However, unlike Murmur3, its implementation is very complex and relies on manually written SIMD code to achieve its high speeds.

**MeowHash** is an excellent choice where appropriate.  It ensures full diffusion between input blocks and its speed is best-in-class.  It also cleanly passes empirical tests (e.g. SMHasher).  However, MeowHash has a complex implementation that utilizes AES primitives, and it requires AES hardware instructions to achieve its high speeds.  That's a perfectly reasonable trade-off to make: high speeds in exchange for complexity and trickier porting.

**TentHash** is best compared to MeowHash in the above list.  Both are conservative about hash quality and collision strength, but they make a different speed/complexity trade-off: TentHash is very simple to implement, and as a result only has *reasonable* performance rather than extreme performance.  The benefit of that trade-off is that independent conformant implementations are very easy to write, easy to port, and the hash design itself is easy to understand.


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

## Notes

<ol style="font-size: 0.7em">
  <li>The term "robust" is used to avoid confusion, since "strong collision resistance" has a specific cryptographic meaning.  But in the colloquial rather than technical sense, "strong collision resistance" is the intended meaning here.</li>
  <li> This does not reflect small-input performance, since TentHash's target use case is message digests, not hash maps.  TentHash's data throughput is relatively worse on small inputs.</li>
  <li>For non-cryptographic hashes, a 160-bit digest isn't meaningfully better than a 128-bit digest in the vast majority of practical applications.  See the design rationale document for how TentHash ended up at 160 bits.</li>
  <li>For FNV-1a, "per block" isn't easy to define, because it hashes a byte at a time rather than a block at a time.  The diffusion numbers listed are lower and upper bounds based on a 128-bit input block size.</li>
</ol>
