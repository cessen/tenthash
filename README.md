# TentHash

TentHash is a high-quality, non-cryptographic, 160-bit hash function.  It is also portable, easy to implement, and reasonably fast.

- [TentHash Specification](docs/specification.md)
- [Design Rationale Document](docs/design_rationale.md)

TentHash's target applications are data fingerprinting, content-addressable systems, and other use cases that don't tolerate hash collisions.

Importantly, TentHash is [explicitly *not* intended to stand up to attacks](supplemental/collision/), and should never be used where the choice of hash function has security considerations.  Its robustness against collisions is only meaningful under non-adversarial conditions.  In other words, like a good tent, it will protect you from the elements, but will do very little to protect you from attackers.

Also like a good tent, it is compact (a full implementation is around 50 lines of straightforward code) and you can take it anywhere (no special hardware instructions needed).


## Why yet another hash?

Very few non-cryptographic hashes have a public design justification, instead relying on empirical test suites like SMHasher to assert hash quality.  Unfortunately, such test suites are [fundamentally inadequate for asserting the quality of large-output hashes](https://blog.cessen.com/post/2024_07_10_hash_design_and_goodharts_law) due to the enormous state space involved, leaving the quality of such hashes in question.  **This is concerning** because the main purpose of large-output hashes is to effectively guarantee zero hash collisions, making hash quality critical.

TentHash was created partly in response to this situation.  As such, test suites like SMHasher were not used in its design process at any point.  Instead, TentHash was designed through proper analysis, and comes with a [public design rationale](docs/design_rationale.md) that anyone can review and audit.

TentHash does pass SMHasher, and should easily pass any battery of tests that aren't security related.  But TentHash's design rationale is the main source of its quality claims, not test suite results.

Of course, hash quality is only one factor when considering whether to use a particular hash function.  With that in mind, here are the goals that went into TentHash's development, ordered from highest to lowest priority:

1. **Robust against collisions.**  For all practical purposes, it should be safe to assume that different pieces of (non-malicious) data will never have colliding hashes.
2. **Documented & justified design.**  Its design should be properly documented, along with the rationale justifying that design.  People shouldn't have to guess at the rationale, and they shouldn't have to wade through sprawling, obtuse source code to figure out how to write an independent implementation.
3. **Simple and portable.**  It should be easy to understand and straightforward to write conforming (and performant) implementations, without need for special hardware instructions.
4. **Reasonably fast.** It doesn't need to win any speed competitions, but its speed should be measured in GB/sec, not MB/sec, on typical hardware.

If these priorities align well with your use case and preferences, then TentHash is likely a good choice of hash function for you.

However, keep in mind that it's still worth considering a cryptographic hash function over TentHash if simplicity and performance are unimportant to your use case.  TentHash aims to fill the niche where the hash function itself does not need to be secure, but a simple, portable, performant implementation is preferred or needed.


## Comparison with other hashes.

The table below is a comparison of TentHash to a selection of other non-cryptographic hashes with outputs large enough to be used as data fingerprints.  Some cryptographic hashes are also included at the bottom for reference.

The "blocks per full diffusion" column is a partial indicator of hash quality, with 1 block being optimal and more blocks (typically) being worse.[^1]

Data throughput was measured single-threaded on an AMD Ryzen 5 7640U with [SMHasher3](https://gitlab.com/fwojcik/smhasher3).

| Name                  | Output size  | Data throughput[^2] | Blocks per full diffusion[^1] | Documented design rationale |
|-----------------------|--------------|---------------------|-------------------------------|-----------------------------|
| TentHash              | 160 bits[^3] | 9.0 GB/s            | 1 block                       | Yes                         |
| -                     |              |                     |                               |                             |
| xxHash3 (128-bit)     | 128 bits     | 56.4 GB/s           | Never[^4]                     | No                          |
| MeowHash v0.5         | 128 bits     | 53.2 GB/s           | ~6 blocks                     | No[^5]                      |
| MetroHash128          | 128 bits     | 24.5 GB/s           | ~22 blocks                    | No                          |
| CityHash128           | 128 bits     | 21.0 GB/s           | ~3 blocks                     | No                          |
| Murmur3 (x64 128-bit) | 128 bits     | 9.1 GB/s            | ~6 blocks                     | No                          |
| FNV-1a (128-bit)      | 128 bits     | 0.8 GB/s            | Never                         | No                          |
| -                     |              |                     |                               |                             |
| SHA2-256              | 256 bits     | 1.9 GB/s            | -                             | Yes                         |
| Blake2b               | 256 bits     | 1.0 GB/s            | -                             | Yes                         |
| Blake3 (SSE4)         | 256 bits     | 2.6 GB/s[^6]        | -                             | Yes                         |

**TentHash is the only non-cryptographic hash in the list that is unambiguously conservative about quality,** and which can therefore be confidently used in situations that don't tolerate collisions.  It's also the only non-cryptographic hash in the list that publishes a full design rationale for auditing and critique.

In those respects, TentHash is better compared to the cryptographic hashes in the list.  TentHash is, of course, *in no way* cryptographically secure.  But for use cases where that isn't needed, TentHash compares favorably while being both faster and substantially simpler to implement and port.

Not listed in the comparison table are hashes like UMASH and HalftimeHash, which are non-cryptographic but have provable bounds on their collision probabilities.  Such hashes are an excellent contribution to the world of hashing, and have many applications.  However, their collision probabilities (usually around 2<sup>-70</sup> to 2<sup>-80</sup> for a single pair of hashes) are insufficient for use cases like content addressable systems due to the birthday paradox.  These kinds of hashes also tend to be somewhat complex to implement and/or require special hardware instructions.

[^1]: "Blocks per full diffusion" means the number of input blocks that must be processed before the mixing/absorption component of the hash diffuses the hash state enough to reach the output size of the hash.</br></br>
  This is an incomplete measure of quality both in the sense that it's insufficient on its own to assert quality *and* in the sense that (due to being a simplistic measure of a single internal component) a hash can be conservative about quality without meeting the "ideal" of 1 block, depending on its design.  Additionally, this measure isn't well defined for hashes like FNV that don't process data in blocks (although in FNV's case it never fully diffuses anyway).  So the results in the table should be interpreted carefully.</br></br>
  The testing for this measure was done via the [supplementary code](https://github.com/cessen/goodhart_hash_supplemental) from [Hash Design and Goodhart's Law](https://blog.cessen.com/post/2024_07_10_hash_design_and_goodharts_law), in case you want to verify the work or see how it was computed for the various hashes.</br></br>
  The results for the cryptographic hashes are omitted because I didn't test them, as it's safe to assume they're 1 block (or equivalent for their design).

[^2]: The data throughput listed does not reflect small-input performance, because TentHash's target use case is data identification/fingerprinting rather than hash maps.  TentHash's data throughput is relatively worse on small inputs.

[^3]: For non-cryptographic hashes, a 160-bit output is unlikely to be meaningfully better than 128-bit in most applications.  Its listing is just for completeness, not to suggest that this particular aspect of TentHash is an advantage over the other hashes.  See the design rationale document for how TentHash ended up at 160 bits.

[^4]: This is a little unfair to xxHash3.  xxHash3 has an additional loop outside the block accumulation loop that further diffuses the hash state every N blocks, such that it does *eventually* diffuse the hash state to 128 bits.  Nevertheless, that further diffusion only runs every N blocks, not every block, and the per-block mixing can never diffuse to 128 bits on its own.  So this is still a potential issue.

[^5]: MeowHash is still a work in progress, and thus insofar as it isn't yet recommending itself for real use, lacking a design rationale document doesn't yet count against it.

[^6]: This is the speed of Blake3 in SMHasher3, which by default only builds it with SSE4.  The official implementation of Blake3 can reach up to 7 GB/sec, depending on build flags.  Those higher speeds are achieved via wider SIMD instructions and hand-written assembly.


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
* Implementations in other programming languages.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you will be licensed as above (MIT/Apache/CC0), without any additional terms or conditions.
