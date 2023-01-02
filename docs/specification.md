# TentHash Specification v0.2

This document defines the TentHash hash function.  It aims to be concise and easy to follow for anyone writing an implementation of TentHash.  It does *not* explain the rationale behind TentHash's design.  For that, please see the [Design Rationale document](design_rationale.md).

**WARNING:** this specification may change in backwards-incompatible ways prior to version 1.0.  After 1.0 is declared, only changes that do not alter the hash output (such as clarifications and better prose) will be made.  There will never be a 2.0.


## Overview

This is the general hashing procedure:

```sh
fn do_hash(input_data):
    hash_state = [A, B, C, D]

    # Process input data.
    for each chunk in input_data:
        hash_state += chunk
        mix hash_state

    # Finalize.
    hash_state[0] ^= input_data_length_in_bits
    mix hash_state  # Once.
    mix hash_state  # Twice.

    return first 160 bits of hash_state
```

Details of each step are specified below.


## Details

### Hash state.

The internal hash state consists of four 64-bit unsigned integers, short-hand labeled `A`, `B`, `C`, and `D` in this document.  Before hashing starts, the hash state is initialized to the following values:

- `A` = `0x5d6daffc4411a967`
- `B` = `0xe22d4dea68577f34`
- `C` = `0xca50864d814cbc2e`
- `D` = `0x894e29b9611eb173`


### Adding the input data.

Input data is processed in 256-bit chunks.  **If the last chunk is less than 256 bits,** it is padded out to 256 bits with zeros and then processed as normal.

Each chunk of data is treated as four 64-bit **little-endian** unsigned integers and is added into the hash state as follows:

```sh
A += chunk[bits 0-63]
B += chunk[bits 64-127]
C += chunk[bits 128-191]
D += chunk[bits 192-255]
```

The additions are modulo 2<sup>64</sup>.


### Xoring the input length.

Once all input data has been processed, the length of the input data **in bits** (not bytes) is xored as an unsigned integer into the `A` component of the hash state:

```sh
A ^= data_length_in_bits
```

Note: conforming implementations of TentHash are **not** required to handle data streams longer than 2<sup>64</sup>-1 bits.  However, implementations that wish to do so should wrap `data_length_in_bits` when exceeding 2<sup>64</sup>-1.  Or in other words, `A` should be xored with the data length in bits modulo 2<sup>64</sup>.


### Mixing the hash state.

The mixing function is defined as follows:

```sh
rotation_constants = [
    [51, 59], [25, 19], [8, 10], [35, 3],
    [45, 38], [61, 32], [23, 53],
]

fn mix(hash_state):
    for pair in rotation_constants:
        A += C
        B += D
        C = (C <<< pair[0]) ^ A
        D = (D <<< pair[1]) ^ B
        swap(A, B)
```

Where `<<<` is a bit-wise left rotation and addition is modulo 2<sup>64</sup>.

As shown in the overview at the beginning of this document, the hash state is mixed once after adding each input chunk and twice during finalization.


### Producing the digest.

The output digest is simply the first 160 bits of the hash state as an array of bytes.  It starts with the least significant byte of `A` and proceeds in order from there.  Only four bytes of `C` are included and no bytes of `D`.

TentHash does not mandate a particular printable representation of the digest.  But if a printable digest is desired then by convention it follows the same procedure as most hashes: each byte of the digest is printed in turn as its unsigned numerical hex value.

For example, a digest of `[10, 212, 156, ...]` would be printed as `0ad49c...`.


## Test Vectors

Test inputs and their corresponding TentHash digests:

- Empty (no input data):
    - `5206df9490caa9093ad61971a0fcb2aa6115d542`,
- A single zero byte:
    - `b9769af5a7f421c0bbbe1063ea695d8e13e6a16d`,
- The ascii string "0123456789":
    - `6a513203d85d60e64ce3d171a28098a496f01225`,
- The ascii string "abcdefghijklmnopqrstuvwxyz":
    - `0ff9f1c49a264acea36739d91f9ac044d58f5d64`,
- The ascii string "The quick brown fox jumps over the lazy dog.":
    - `a7e5568ce1cb6d5933f2f7654f69f309b36dacac`,
