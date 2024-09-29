# TentHash v0.4

A robust 160-bit *non-cryptographic* hash function.

- [TentHash Specification v0.4](docs/specification.md) **WARNING:** although it is very likely that this will become TentHash's final specification, there is still a chance it could change if issues are discovered before it is declared final.
- [Design Rationale Document](docs/design_rationale.md)

TentHash is a high-quality, reasonably fast, large-output hash.  Its target applications are data fingerprinting, checksums, content-addressable systems, and other use cases that don't tolerate hash collisions.

Importantly, TentHash is explicitly *not* intended to stand up to attacks.  Its robustness against collisions is only meaningful under non-adversarial conditions.  In other words, like a good tent, it will protect you from the elements, but will do very little to protect you from attackers.

Also like a good tent, it is compact (a full implementation is around 50 lines of straightforward code) and you can take it anywhere (no special hardware instructions needed).


## Why yet another hash?

TentHash was born out of a desire for a hash that fulfilled *all* of the following criteria:

1. **Robust against collisions.**  For all practical purposes, it should be safe to assume that different pieces of (legitimate) data will never have colliding hashes.
2. **Simple and portable.**  It should be easy to understand and straightforward to write conforming (and performant) implementations, without need for special hardware instructions.
3. **Documented & justified design.**  Its design should be properly documented, along with the rationale justifying that design.  People shouldn't have to guess at the rationale, and they shouldn't have to wade through sprawling, obtuse source code to figure out how to write an independent implementation.
4. **Reasonably fast.**  It doesn't need to win any speed competitions, but its speed should be measured in GB/sec, not MB/sec, on typical hardware.

When I started work on TentHash I was unable to find any hashes that met all four of these criteria, and TentHash aims to fill that gap.


## Comparison with other hashes.

The table below is a comparison of TentHash to a selection of other hashes with outputs large enough to be used as message digests.  Some cryptographic hashes are also included at the bottom for reference.

The "blocks per full diffusion" column is a partial indicator of hash quality, with 1 block being optimal and more blocks (typically) being worse.<sup>1</sup>

Data throughput was measured single-threaded on an AMD Ryzen 5 7640U.  TentHash's throughput was measured using its Rust implementation, and the other hashes using their implementations in [SMHasher](https://github.com/rurban/smhasher).

| Name                                  | Digest size          | Data throughput<sup>2</sup> | Blocks per full diffusion<sup>1</sup> | Documented design rationale |
|---------------------------------------|----------------------|-----------------------------|---------------------------------------|-----------------------------|
| TentHash                              | 160 bits<sup>3</sup> | 9.0 GB/s                    | 1 block                               | Yes                         |
| -                                     |                      |                             |                                       |                             |
| xxHash3 (128-bit)                     | 128 bits             | 56.0 GB/s                   | Never                                 | No                          |
| MeowHash v0.5                         | 128 bits             | 50.5 GB/s                   | ~6 blocks                             | No<sup>4</sup>              |
| MetroHash128                          | 128 bits             | 20.4 GB/s                   | ~22 blocks                            | No                          |
| CityHash128 / FarmHash128<sup>5</sup> | 128 bits             | 17.5 GB/s                   | ~3 blocks                             | No                          |
| Murmur3 (x64 128-bit)                 | 128 bits             | 8.2 GB/s                    | ~6 blocks                             | No                          |
| FNV-1a (128-bit)                      | 128 bits             | 0.46 GB/s                   | Never                                 | No                          |
| -                                     |                      |                             |                                       |                             |
| SHA2-256                              | 256 bits             | 0.3 GB/s                    | -                                     | Yes                         |
| Blake2b                               | 256 bits             | 0.74 GB/s                   | -                                     | Yes                         |
| Blake3                                | 256 bits             | 1.9 GB/s<sup>6</sup>                    | -                                     | Yes                         |

**MetroHash** and **FNV** should probably be avoided for use cases that can't tolerate collisions, especially considering there are better options available.  **xxHash3** is similar in that respect, except it doesn't currently have any competition I'm aware of in its performance bracket among non-WIP hashes.

The remaining non-cryptographic hashes (other than TentHash) all sit pretty near each other in terms of quality.  They *may* be fine, but it's hard to say.  And in any case, their designs don't appear to be *conservative* about hash quality (pending design rationale documents that explain otherwise).

**TentHash** is the only non-cryptographic hash in the list that is unambiguously conservative about quality, and which can confidently be used in situations that can't tolerate collisions.  It's also the only non-cryptographic hash in the list that publishes a full design rationale for auditing and critique.

In those respects, TentHash is better compared to the cryptographic hashes in the list.  TentHash is, of course, *in no way* cryptographically secure.  But for use cases where that isn't needed, TentHash compares favorably while being both faster and substantially simpler to implement and port.


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

1. "Blocks per full diffusion" means the number of input blocks that must be processed before the mixing/absorption component of the hash diffuses the hash state enough to meet the output size of the hash.

   There are a bunch of things to note about this measure:

   First, because it's a measure of an internal component, it cannot be determined from testing the final hash output.  However, that also means it needs to be interpreted carefully.  For example, xxHash3 ends up looking really bad here with "never", but it's not as bad as it looks because the full hash has an additional outer loop that further diffuses the hash state every N blocks.  The lack of diffusion is still an issue for xxHash3, but it's not as damning as "never" might imply.

   Second, it's only a "real" measure for hashes that strictly process input data in fixed-size blocks.  FNV is a prime example of a hash that does *not* do that.  (Although in that particular case it ends up being moot since it never fully diffuses anyway.)

   Third, there are hash designs that can tolerate slower diffusion while still being optimal, so a listing of more than 1 block isn't *necessarily* a ding on quality.  In particular, I suspect that both Murmur3 and MeowHash may be notably better than their numbers here would suggest, although I don't know by how much.  Nevertheless, none of the listed non-cryptographic hashes have documentation to indicate or justify such a design.  Moreover, some of them definitely *don't* have such a design.

   Fourth, the "never" case cannot, of course, be verified experimentally.  Rather, I verified by construction (e.g. by what operations are used, the size of the internal state, etc.) that they *cannot* fully diffuse.  I also tested them up to 10000 blocks to sanity check.

   Fifth, I haven't listed numbers for the cryptographic hashes simply because I haven't bothered to verify them.  But it's safe to assume that they're either 1 block or are designed such that it doesn't matter.

   Finally, the testing for this measure was done with the [supplementary code](https://github.com/cessen/goodhart_hash_supplemental) from [Hash Design and Goodhart's Law](https://blog.cessen.com/post/2024_07_10_hash_design_and_goodharts_law), in case you want to verify the work.

2. This does not reflect small-input performance, because TentHash's target use case is data identification/fingerprinting rather than hash maps.  TentHash's data throughput is relatively worse on small inputs.

3. For non-cryptographic hashes, a 160-bit digest is unlikely to be meaningfully better than 128-bit in most realistic applications.  Its listing here is for completeness, not to indicate an advantage over the other hashes.  See the design rationale document for how TentHash ended up at 160 bits.

4. MeowHash is still a work in progress, and thus insofar as it isn't yet recommending itself for real use, this doesn't count against it.

5. CityHash128 and FarmHash128 use exactly the same construction for at least the part of the hash relevant to the diffusion metric, and also have the same data throughput.  (I *think* they're even just identical hashes, but I haven't bothered to properly confirm that.)

6. As noted earlier, this is the speed of Blake3 in SMHasher.  The official implementation of Blake3 with default build flags is several times faster, at around 7 GB/sec.  That higher speed is achieved via hand-written assembly and SIMD intrinsics.