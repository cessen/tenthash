# TentHash Specification v0.1

This document defines the TentHash hash function.  It aims to be concise and easy to follow for anyone writing an implementation of TentHash.  It does *not* explain the rationale behind TentHash's design.  For that, please see the [Design Rationale document](design_rationale.md).

**WARNING:** this specification may change in backwards-incompatible ways prior to version 1.0.  After 1.0 is declared, only changes that do not alter the hash output (such as clarifications and better prose) will be made.  There will never be a 2.0.


## Overview

This is the general hashing procedure:

```sh
fn do_hash(input_data):
    hash_state = [A, B, C, D]

    # Process data.
    for each chunk in input_data:
        xor chunk into hash_state
        mix hash_state for 6 rounds

    # Finalize.
    xor input_data length in bits into hash_state
    mix hash_state for 12 rounds

    return first 160 bits of hash_state
```

Details of each step are specified below.


## Details

### Hash state.

The internal hash state consists of four 64-bit unsigned integers, short-hand labeled `A`, `B`, `C`, and `D` in this document.  Before hashing starts, the hash state is initialized to the following values:

- `A` = `0xe2b8d3b67882709f`
- `B` = `0x045e21ec46bcea22`
- `C` = `0x51ea37fa96fbae67`
- `D` = `0xf5d94991b6b9b944`


### Xoring the input data.

Input data is processed in 256-bit chunks.  **If the last chunk is less than 256 bits,** it is padded out to 256 bits with zeros and then processed as normal.

Each chunk of data is treated as four 64-bit *little-endian* unsigned integers (i.e. on big-endian platforms the byte order within each individual 64-bit sub-chunk must be reversed) and is xored into the hash state as follows:

```sh
A ^= chunk[bits 0-63]
B ^= chunk[bits 64-127]
C ^= chunk[bits 128-191]
D ^= chunk[bits 192-255]
```


### Xoring the input length.

Once all input data has been processed, the length of the input data **in bits** (not bytes) is xored as an unsigned integer into the `A` component of the hash state:

```sh
A ^= data_length_in_bits
```

(Note: TentHash is not intended to be used with data streams longer than 2<sup>64</sup>-1 bits.  However, as a matter of specification, `data_length_in_bits` should simply wrap when exceeding 2<sup>64</sup>-1.  Or in other words, `A` should be xored with the data length in bits modulo 2<sup>64</sup>.)


### Mixing the hash state.

The mixing function is parameterized by a number of rounds, and is defined as follows:

```sh
fn mix_hash_state(number_of_rounds):
    constants = [
        [31, 25], [5, 48],
        [20, 34], [21, 57],
        [11, 41], [18, 33],
    ]

    for i in 0 to number_of_rounds:
        A += C
        B += D
        C = (C <<< constants[i % 6][0]) ^ A
        D = (D <<< constants[i % 6][1]) ^ B
        swap(C, D)
```

Where `<<<` is a bit-wise left rotation, and the loop's 0-to-`number_of_rounds` range is exclusive of `number_of_rounds`.

As shown in the overview at the beginning of this document, the hash state is mixed for 6 rounds after xoring each input chunk and for 12 rounds after xoring the input length.


### Producing the digest.

The output digest is simply the first 160 bits of the hash state.  It should be returned as an array of 20 bytes, with the least significant byte of `A` being at index 0, and proceeding in order from there.  This means only four bytes of `C` are included and no bytes of `D`.

TentHash does not mandate a particular printable string representation of the digest.  But if a printable digest is desired then by convention it follows the same procedure as most hashes: the bytes of the digest are each printed as their unsigned numerical hex value, starting from the byte at index 0.

For example, a digest of `[10, 212, 156, ...]` would be printed as `0ad49c...`.


## Example Digests / Test Vectors

- Empty (no input data):
    - `e0d4e0a2608a8741e349fa1ea0263fedbd65f66d`
- A single zero byte:
    - `6e5f483d20443bb6e70c300b0a5aa64ce36d3467`
- The ascii string "0123456789":
    - `f12f795967313e9a0e822edaa307c3d7b7d19ce3`
- The ascii string "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa":
	- `9f4c56c99c8fb971bfbfcbcf9c6296c85fba7733`
- The ascii string "The quick brown fox jumps over the lazy dog.":
    - `0be19c6dc03f6800743e41c70f0ee0c2d75bad67`
