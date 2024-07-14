/// TentHash reference implementation in C, for little-endian platforms.
///
/// Note: this gives incorrect results on big-endian platforms.

#ifndef TENTHASH_H
#define TENTHASH_H

#include <stdint.h>
#include <string.h>

#define TENT_BLOCK_SIZE (256 / 8)
#define TENT_DIGEST_SIZE (160 / 8)
#define ROTL_64(x, n) ((x << n) | (x >> (64 - n)))

void mix_state(uint64_t *);

typedef struct {
    uint8_t bytes[TENT_DIGEST_SIZE];
} Digest;

Digest hash(const void *in_data, uint64_t data_len) {
    const uint64_t data_len_bits = data_len * 8;
    uint8_t *data = (uint8_t *)in_data;

    uint64_t state[4] = {
        0x5d6daffc4411a967,
        0xe22d4dea68577f34,
        0xca50864d814cbc2e,
        0x894e29b9611eb173,
    };

    // Process the input data in 256-bit blocks.
    while (data_len > 0) {
        uint64_t block_size = (data_len < TENT_BLOCK_SIZE) ? data_len : TENT_BLOCK_SIZE;

        // Copy the block into a zeroed-out buffer.  When the block is
        // smaller than 256 bits this pads it out to 256 bits with zeros.
        uint8_t buffer[TENT_BLOCK_SIZE] = {0};
        memcpy(buffer, data, block_size);

        // Incorporate the block into the hash state.
        state[0] ^= *((uint64_t *)buffer);
        state[1] ^= *((uint64_t *)(buffer + 8));
        state[2] ^= *((uint64_t *)(buffer + 16));
        state[3] ^= *((uint64_t *)(buffer + 24));

        mix_state(state);

        data += block_size;
        data_len -= block_size;
    }

    // Finalize.
    state[0] ^= data_len_bits;
    mix_state(state);
    mix_state(state);

    // Convert the hash state into a digest.
    Digest digest;
    memcpy(digest.bytes, ((uint8_t*)state), TENT_DIGEST_SIZE);

    return digest;
}

void mix_state(uint64_t *state) {
    // Rotation constants.
    const static int ROTS[7][2] = {
        {51, 59}, {25, 19}, {8, 10}, {35, 3},
        {45, 38}, {61, 32}, {23, 53},
    };

    for (int i = 0; i < 7; i++) {
        state[0] += state[2];
        state[1] += state[3];
        state[2] = ROTL_64(state[2], ROTS[i][0]) ^ state[0];
        state[3] = ROTL_64(state[3], ROTS[i][1]) ^ state[1];

        // Swap.
        const uint64_t tmp = state[0];
        state[0] = state[1];
        state[1] = tmp;
    }
}

#endif
