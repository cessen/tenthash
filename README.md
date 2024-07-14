# TentHash v0.3

A robust 160-bit *non-cryptographic* hash function.

- [TentHash Specification v0.3](docs/specification.md) **WARNING:** TentHash's design is not yet finalized, and digest results may change before the specification is declared 1.0.
- [Design Rationale Document](docs/design_rationale.md)

TentHash is a high-quality, reasonably fast checksum for data identification.  It has a simple design, doesn't require any special hardware instructions, and takes less than 50 lines of straightforward code for a full implementation.

TentHash is explicitly *not* intended to stand up to attacks.  Its otherwise robust<sup>1</sup> collision resistance is only meaningful under non-adversarial conditions.  In other words, like a good tent, it will protect you from the elements, but will do very little to protect you from attackers.


## Why yet another hash?

I wanted a checksum hash that fulfilled *all* of the following criteria:

1. **Robust<sup>1</sup> collision resistance.**  For all practical purposes, it should be safe to assume that different pieces of (legitimate) data will never have colliding hashes.
2. **Simplicity and portability.**  It should be easy to understand and straightforward to write conforming (and performant) implementations, without need for special hardware instructions.
3. **A documented and justified design.**  Its design should be properly documented, along with the rationale behind that design.  People shouldn't have to guess at the rationale, and they shouldn't have to wade through sprawling, obtuse source code to figure out how to write an independent implementation.
4. **Reasonably fast.**  It doesn't need to win any speed competitions, but its speed should be measured in GB/sec, not MB/sec, on typical hardware.

I was unable to find any hash that brought all four of these things together, so I set out to make one.

Additionally, there are *a lot* of hashes out there that do not meet criteria 3.  I think that's a real shame, [especially for hashes with large-output variants](https://blog.cessen.com/post/2024_07_10_hash_design_and_goodharts_law).  So TentHash also aims to be among the hashes that help raise the bar for what's expected of non-cryptographic hashes in that respect.


## Comparison with other hashes.

The table below is a comparison of TentHash to a selection of other hashes with outputs large enough to be used as message digests.  Some cryptographic hashes are also included at the bottom for reference.

The "blocks per full diffusion" column is a partial indicator of hash quality.  It measures how quickly the internal hash state is diffused while processing input, where "full" means meeting or exceeding the digest size.  1 block is optimal and more blocks is generally worse.<sup>2</sup>  (Note that this measure cannot be determined from the final hash output, and requires instrumenting or extracting internal components of the hash function.)

"Data throughput" was measured single-threaded on an AMD Ryzen 5 7640U.  TentHash throughput was measured using its Rust implementation, and the other hashes using their implementations in [SMHasher](https://github.com/rurban/smhasher).

"Blocks per full diffusion" was measured via the [supplementary code](https://github.com/cessen/goodhart_hash_supplemental) from [Hash Design and Goodhart's Law](https://blog.cessen.com/post/2024_07_10_hash_design_and_goodharts_law).

| Name                                  | Digest size          | Data throughput<sup>3</sup> | Blocks per full diffusion | Documented design rationale |
|---------------------------------------|----------------------|-----------------------------|---------------------------|-----------------------------|
| TentHash                              | 160 bits<sup>4</sup> | 8.6 GB/s                    | 1 block                   | Yes                         |
| -                                     |                      |                             |                           |                             |
| xxHash3 (128-bit)                     | 128 bits             | 56.0 GB/s                   | Never<sup>5</sup>         | No                          |
| MeowHash v0.5                         | 128 bits             | 50.5 GB/s                   | ~6 blocks                 | No<sup>6</sup>              |
| MetroHash128                          | 128 bits             | 20.4 GB/s                   | ~22 blocks                | No                          |
| CityHash128 / FarmHash128<sup>7</sup> | 128 bits             | 17.5 GB/s                   | ~3 blocks                 | No                          |
| Murmur3 (x64 128-bit)                 | 128 bits             | 8.2 GB/s                    | ~6 blocks                 | No                          |
| FNV-1a (128-bit)                      | 128 bits             | 0.46 GB/s                   | Never<sup>2</sup>         | No                          |
| -                                     |                      |                             |                           |                             |
| SHA2-256                              | 256 bits             | 0.3 GB/s                    | -                         | Yes                         |
| Blake2b                               | 256 bits             | 0.74 GB/s                   | -                         | Yes                         |
| Blake3                                | 256 bits             | 1.9 GB/s                    | -                         | Yes                         |



**FNV** and **MetroHash** should probably be avoided for use cases that can't tolerate collisions, especially considering there are other better options available.  **XXHash3** is similar in that respect, except it doesn't currently have any competition I'm aware of in its performance bracket (although MeowHash will be a contender when it's declared 1.0).

The remaining non-cryptographic hashes (other than TentHash) all sit pretty near each other in terms of quality.  They *may* be fine, but it's hard to say.  And in any case, their designs don't appear to be *conservative* about hash quality (pending design rationale documents that explain otherwise).

**TentHash** is the only non-cryptographic hash in the list that is unambiguously conservative about quality, and which can confidently be used in situations that can't tolerate collisions.  It's also the only non-cryptographic hash in the list that publishes its design rationale for auditing and critique.

In those respects, TentHash is better compared to the cryptographic hashes in the list.  TentHash is, of course, *in no way* cryptographically secure.  But for use cases where that isn't needed, TentHash compares favorably.

Disregarding cryptographic security, TentHash's main competitor appears to be Blake3.  Blake3 is also reasonably fast, conservative about quality, and of course has a thoroughly documented and justified design.  The main benefit of TentHash over Blake3 is that TentHash requires very little code and is very simple.

Unlike Blake3, TentHash achieves its performance without SIMD or other special hardware instructions.  A real implementation can be written in under 50 lines of straightforward, portable code.  Moreover, its design is straightforward to grasp, without requiring any advanced theory.


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

2. There are a couple of things to note about this measure:

   First, it's only a "real" measure for hashes that strictly process input data in fixed-size blocks.  FNV is a good example of a hash that does *not* do that, although in this case that ends up being moot since its internal state simply never fully diffuses anyway.

   Second, there are hash designs that can tolerate slower diffusion while still being optimal, so a listing of more than 1 block isn't *necessarily* a ding on quality.  However, none of these hashes have documentation to indicate or justify such a design.  Moreover, some of these hashes definitely *don't* have such a design.

3. This does not reflect small-input performance, since TentHash's target use case is message digests, not hash maps.  TentHash's data throughput is relatively worse on small inputs.

4. For non-cryptographic hashes, a 160-bit digest isn't meaningfully better than a 128-bit digest in the vast majority of practical applications, and its listing here is just for completeness, not to indicate an advantage over the other hashes' digest sizes.  See the design rationale document for how TentHash ended up at 160 bits.

5. For xxhash3 this isn't *quite* as bad as it sounds, because although the inner accumulation loop never fully diffuses on its own, there is a loop outside of that which further diffuses the hash state.  However, that diffusion is only run once every N blocks, so it's still an issue regardless.  And I haven't measured that outer component to see how effective it is.

6. MeowHash is still a work in progress, so insofar as it isn't yet recommending itself for real use, this doesn't count against it.

7. CityHash128 and FarmHash128 use exactly the same construction for the part of the hash relevant to this metric, and also have very similar performance.  (They may even just be identical hashes...?  I didn't check that far.)
