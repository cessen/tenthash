Basic benchmarks for various Rust implementations of some hash functions.

Aside from TentHash and Blake3, none of the hash implementations benchmarked here are the official ones (which are usually written in C or C++), and are typically a bit slower than the official ones.

The most important benchmark here is Blake3, because its official implementation is in Rust, and as far as I know that's the only fully optimized implementation.  For example, the C implementation used in SMHasher is several times slower.

TentHash is also benchmarked here with its main Rust implementation, but in TentHash's case that doesn't make much difference because a straightforward C implementation has essentially identical performance.
