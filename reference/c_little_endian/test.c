#include <stdio.h>
#include "tenthash.h"

void main() {
    const static uint8_t inputs[][64] = {
        {},
        {0},
        "0123456789",
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "The quick brown fox jumps over the lazy dog.",
    };
    const static int inputs_len[] = {0, 1, 10, 32, 44};
    const static uint8_t digests[][20] = {
        {0xe0, 0xd4, 0xe0, 0xa2, 0x60, 0x8a, 0x87, 0x41, 0xe3, 0x49, 0xfa, 0x1e, 0xa0, 0x26, 0x3f, 0xed, 0xbd, 0x65, 0xf6, 0x6d},
        {0x6e, 0x5f, 0x48, 0x3d, 0x20, 0x44, 0x3b, 0xb6, 0xe7, 0x0c, 0x30, 0x0b, 0x0a, 0x5a, 0xa6, 0x4c, 0xe3, 0x6d, 0x34, 0x67},
        {0xf1, 0x2f, 0x79, 0x59, 0x67, 0x31, 0x3e, 0x9a, 0x0e, 0x82, 0x2e, 0xda, 0xa3, 0x07, 0xc3, 0xd7, 0xb7, 0xd1, 0x9c, 0xe3},
        {0x9f, 0x4c, 0x56, 0xc9, 0x9c, 0x8f, 0xb9, 0x71, 0xbf, 0xbf, 0xcb, 0xcf, 0x9c, 0x62, 0x96, 0xc8, 0x5f, 0xba, 0x77, 0x33},
        {0x0b, 0xe1, 0x9c, 0x6d, 0xc0, 0x3f, 0x68, 0x00, 0x74, 0x3e, 0x41, 0xc7, 0x0f, 0x0e, 0xe0, 0xc2, 0xd7, 0x5b, 0xad, 0x67},
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
