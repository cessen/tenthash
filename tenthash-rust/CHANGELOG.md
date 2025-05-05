# Changelog

## [Unreleased]


## [1.1.0] - 2025-05-05

- Added helper trait `DigestExt`, with methods for truncating the digest to 128 bits.  Thanks to Gray Olson (@fu5ha).

## [1.0.0] - 2025-01-28

- This release represents a commitment to API stability moving forward.
- Renamed `TentHasher` struct to just `TentHash`.
- Documentation polish.


## [0.5.0] - 2025-01-01

- TentHash spec declared final, with no changes from draft v0.4.
- No functional changes, but removed notices of hash output possibly changing in the future, since that's no longer true.


## [0.4.0] - 2024-09-29

- Implement v0.4 of the draft TentHash spec.
  - Further improved rotation constants, giving even better diffusion.  This wasn't necessary, since diffusion was already more than sufficient, but gives even more quality margin.
- Include a `hash()` function for conveniently hashing data that's already contiguous in memory.
- Minor performance improvements.


## [0.3.0] - 2024-01-20

- Implement v0.3 of the draft TentHash spec:
  - Change back to using xor to incorporate blocks.  Upon further consideration, the switch to addition before wasn't actually justified or necessary, and xor is faster on platforms without native 64-bit integer support (e.g. certain embedded platforms).


## [0.2.0] - 2023-01-11

- Implement v0.2 of the draft TentHash spec:
  - Change initial hash state.
  - Use addition instead of xor to incorporate blocks.
  - More mix rounds and better rotation constants, to improve diffusion.


## [0.1.0] - 2022-09-04

- Initial release, implementing the first draft of the hash design.


[Unreleased]: https://github.com/cessen/tenthash/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/cessen/tenthash/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/cessen/tenthash/compare/v0.5.0...v1.0.0
[0.5.0]: https://github.com/cessen/tenthash/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/cessen/tenthash/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/cessen/tenthash/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/cessen/tenthash/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/cessen/tenthash/releases/tag/v0.1.0
