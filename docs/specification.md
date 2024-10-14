# TentHash Specification v0.4

This document defines the TentHash hash function.  It aims to be concise and easy to follow for anyone writing an implementation of TentHash.  It does *not* explain the rationale behind TentHash's design.  For that, please see the [Design Rationale document](design_rationale.md).

**NOTE:** although it is likely that this version of the spec will become TentHash's final specification, there is still a chance it could change if issues are discovered before it is declared final.  When it is declared final, the version number will be removed, and after that point only changes that do not alter the hash output (such as clarifications and better prose) will be made.

## Overview

This is the general hashing procedure:

```sh
fn do_hash(input_data):
    hash_state = [A, B, C, D]

    # Process input data.
    for each block in input_data:
        hash_state ^= block
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


### Xoring the input data.

Input data is processed in 256-bit blocks.  **If the last block is less than 256 bits,** it is padded out to 256 bits with zeros and then processed as normal.

Each block of data is treated as four 64-bit **little-endian** unsigned integers and is xored into the hash state as follows:

```sh
A ^= block[bits 0-63]
B ^= block[bits 64-127]
C ^= block[bits 128-191]
D ^= block[bits 192-255]
```


### Xoring the input length.

Once all input data has been processed, the length of the input data **in bits** (not bytes) is xored as an unsigned integer into the `A` component of the hash state:

```sh
A ^= data_length_in_bits
```

Note: conforming implementations of TentHash are **not** required to handle data streams longer than 2<sup>64</sup>-1 bits.  However, implementations that wish to do so must wrap `data_length_in_bits` when exceeding 2<sup>64</sup>-1.  Or in other words, `A` should be xored with the data length in bits modulo 2<sup>64</sup>.


### Mixing the hash state.

The mixing function is defined as follows:

```
rotation_constants = [
    [16, 28], [14, 57], [11, 22], [35, 34],
    [57, 16], [59, 40], [44, 13],
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

As shown in the overview at the beginning of this document, the hash state is mixed once after xoring each input block and twice during finalization.


### Producing the digest.

The output digest is simply the first 160 bits of the hash state as an array of bytes.  It starts with the least significant byte of `A` and proceeds in order from there.  Only four bytes of `C` are included and no bytes of `D`.

TentHash does not mandate a particular printable representation of the digest.  But if a printable digest is desired then by convention it follows the same procedure as most hashes: each byte of the digest is printed in turn as its unsigned numerical hex value.

For example, a digest of `[10, 212, 156, ...]` would be printed as `0ad49c...`.


## Test Vectors

Test inputs and their corresponding TentHash digests:

- Empty (no input data):
    - `68c8213b7a76b8ed267dddb3d8717bb3b6e7cc0a`
- A single zero byte:
    - `3cf6833cca9c4d5e211318577bab74bf12a4f090`
- The ascii string "0123456789":
    - `a7d324bde0bf6ce3427701628f0f8fc329c2a116`
- The ascii string "abcdefghijklmnopqrstuvwxyz":
    - `f1be4be1a0f9eae6500fb2f6b64f3daa3990ac1a`
- The ascii string "This string is exactly 32 bytes.":
    - `f7c5e4763d89bddce33e97712b712d869aabcfe9`
- The ascii string "The quick brown fox jumps over the lazy dog.":
    - `de77f1c134228be1b5b25c941d5102f87f3e6d39`
