#include <stdio.h>
#include "tenthash.h"

void main() {
    const static uint8_t inputs[][64] = {
        {},
        {0},
        "0123456789",
        "abcdefghijklmnopqrstuvwxyz",
        "The quick brown fox jumps over the lazy dog.",
    };
    const static int inputs_len[] = {0, 1, 10, 26, 44};
    const static uint8_t digests[][24] = {
        {0x52, 0x06, 0xdf, 0x94, 0x90, 0xca, 0xa9, 0x09, 0x3a, 0xd6, 0x19, 0x71, 0xa0, 0xfc, 0xb2, 0xaa, 0x61, 0x15, 0xd5, 0x42},
        {0xb9, 0x76, 0x9a, 0xf5, 0xa7, 0xf4, 0x21, 0xc0, 0xbb, 0xbe, 0x10, 0x63, 0xea, 0x69, 0x5d, 0x8e, 0x13, 0xe6, 0xa1, 0x6d},
        {0x6a, 0x51, 0x32, 0x03, 0xd8, 0x5d, 0x60, 0xe6, 0x4c, 0xe3, 0xd1, 0x71, 0xa2, 0x80, 0x98, 0xa4, 0x96, 0xf0, 0x12, 0x25},
        {0x0f, 0xf9, 0xf1, 0xc4, 0x9a, 0x26, 0x4a, 0xce, 0xa3, 0x67, 0x39, 0xd9, 0x1f, 0x9a, 0xc0, 0x44, 0xd5, 0x8f, 0x5d, 0x64},
        {0xa7, 0xe5, 0x56, 0x8c, 0xe1, 0xcb, 0x6d, 0x59, 0x33, 0xf2, 0xf7, 0x65, 0x4f, 0x69, 0xf3, 0x09, 0xb3, 0x6d, 0xac, 0xac},
    };

    int failures = 0;
    for (int i = 0; i < 5; i++) {
        Digest output = hash(inputs[i], inputs_len[i]);

        for (int j = 0; j < 20; j++) {
            if (output.bytes[j] != digests[i][j]) {
                printf("Failed test vector %d.\n", i + 1);
                failures++;
                break;
            }
        }
    }

    if (failures == 0) {
        printf("All test vectors passed.\n");
    }
}
