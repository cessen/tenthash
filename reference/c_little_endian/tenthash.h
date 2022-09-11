/// TentHash reference implementation in C, for little-endian platforms.
///
/// Note: this gives incorrect results on big-endian platforms.

#ifndef TENTHASH_H
#define TENTHASH_H

#include <stdint.h>
#include <string.h>

#define TENT_BLOCK_SIZE (256 / 8)
#define TENT_DIGEST_SIZE (160 / 8)
#define ROTATE_L64(x, n) ((x << n) | (x >> (64 - n)))

void mix_state(uint64_t *, int);

typedef struct {
    uint8_t bytes[TENT_DIGEST_SIZE];
} Digest;

Digest hash(const void *in_data, uint64_t data_len) {
    const uint64_t data_len_bits = data_len * 8;
    uint8_t *data = (uint8_t *)in_data;

    uint64_t state[4] = {
        0xe2b8d3b67882709f,
        0x045e21ec46bcea22,
        0x51ea37fa96fbae67,
        0xf5d94991b6b9b944,
    };

    // Process the input data in 256-bit chunks.
    while (data_len > 0) {
        uint64_t chunk_size = (data_len < TENT_BLOCK_SIZE) ? data_len : TENT_BLOCK_SIZE;

        // Copy the chunk into a zeroed-out buffer.  When the chunk is
        // smaller than 256 bits this pads it out to 256 bits with zeros.
        uint8_t buffer[TENT_BLOCK_SIZE] = {0};
        memcpy(buffer, data, chunk_size);

        // Xor the chunk/buffer into the hash state.
        state[0] ^= *((uint64_t *)buffer);
        state[1] ^= *((uint64_t *)(buffer + 8));
        state[2] ^= *((uint64_t *)(buffer + 16));
        state[3] ^= *((uint64_t *)(buffer + 24));

        mix_state(state, 6);

        data += chunk_size;
        data_len -= chunk_size;
    }

    // Finalize.
    state[0] ^= data_len_bits;
    mix_state(state, 12);

    // Convert the hash state into a 160-bit digest.
    Digest digest;
    memcpy(digest.bytes, ((uint8_t*)state), TENT_DIGEST_SIZE);

    return digest;
}

void mix_state(uint64_t *state, int rounds) {
    // Rotation constants.
    const static int ROTS[6][2] = {
        {31, 25}, {5, 48}, {20, 34}, {21, 57}, {11, 41}, {18, 33},
    };

    for (int i = 0; i < rounds; i++) {
        state[0] += state[2];
        state[1] += state[3];
        state[2] = ROTATE_L64(state[2], ROTS[i % 6][0]) ^ state[0];
        state[3] = ROTATE_L64(state[3], ROTS[i % 6][1]) ^ state[1];

        // Swap elements 2 and 3.
        uint64_t tmp = state[2];
        state[2] = state[3];
        state[3] = tmp;
    }
}

#endif
