# TentHash

[![Latest Release][crates-io-badge]][crates-io-url]
[![Documentation][docs-rs-img]][docs-rs-url]

A strong 160-bit *non-cryptographic* hash function.

**WARNING:** TentHash's design is not yet finalized, and digest results may change before 1.0 is declared.  Please do not rely on this (yet) for persistent, long-term checksums.

TentHash is intended to be used as a reasonably fast but (more importantly) high-quality checksum for data identification.  Moreover, it has a simple portable design that is easy to audit, doesn't require special hardware instructions, and is easy to write conforming independent implementations of.

TentHash is explicitly *not* intended to stand up to attacks.  In fact, attacks are *quite easy* to mount against it.  Its otherwise strong collision resistance is only meaningful under non-adversarial conditions.  In other words, like a good tent, it will protect you from the elements, but will do very little to protect you from attackers.


## Why yet another hash?

TentHash aims to fill a gap I encountered when searching for a hash to use in a content-addressable system that had large binary files.  My use case was not at all security sensitive, but strong collision resistance (with legitimate data) was critical.  Moreover, I wanted to be able to understand the hash and write independent implementations if needed.

This led to the following priorities, in order of importance:

1. **Strong collision resistence.**  For all practical purposes, it should be safe to assume that different pieces of (legitimate) data will never have colliding hashes.
2. **Simplicity.**  It shouldn't require diagrams to understand, and it should be straightforward to write conforming implementations for just about any platform.  It should also be *easy to audit* for (non-security) weaknesses.
3. **Reasonably fast.**  It doesn't need to be break-neck fast, but it should be able to process 20 GB+ files in seconds, not minutes, without resorting to multi-threading.

I couldn't find any hash that fit these priorities.  There are hashes that meet items 1 and 3, but are obtuse or complex in their design, or require special hardware instructions to run efficiently.  Similarly, there are hashes that meet items 2 and 3 but either produce too small of digests or have designs that don't inspire confidence in their collision resistance.

(As an aside: there are also some hashes that seem to be more-or-less designed against SMHasher, which does not inspire confidence.  SMHasher is great, but you still need your hash to be good *by construction* when collision resistance is critical.  See Goodhart's law.)

So I set to work creating TentHash.


## What qualifies you to create a hash function?

Honestly?  Nothing in particular.  I do have experience working with hash-like constructions for some [unusual](https://psychopath.io/post/2021_01_30_building_a_better_lk_hash) [purposes](https://psychopath.io/post/2022_08_14_a_fast_hash_for_base_4_owen_scrambling), which has given me an appreciation for the subtley of bit mixing.

But more importantly, my hope is that by keeping this hash simple it will be easy for others to evaluate on their own, rather than just trusting that I know what I'm doing.

That's also why I haven't declared 1.0 yet, because I'd like a chance for others to take a look and find possible issues first.


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
