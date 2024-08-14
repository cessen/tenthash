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
        {0x68, 0xc8, 0x21, 0x3b, 0x7a, 0x76, 0xb8, 0xed, 0x26, 0x7d, 0xdd, 0xb3, 0xd8, 0x71, 0x7b, 0xb3, 0xb6, 0xe7, 0xcc, 0x0a},
        {0x3c, 0xf6, 0x83, 0x3c, 0xca, 0x9c, 0x4d, 0x5e, 0x21, 0x13, 0x18, 0x57, 0x7b, 0xab, 0x74, 0xbf, 0x12, 0xa4, 0xf0, 0x90},
        {0xa7, 0xd3, 0x24, 0xbd, 0xe0, 0xbf, 0x6c, 0xe3, 0x42, 0x77, 0x01, 0x62, 0x8f, 0x0f, 0x8f, 0xc3, 0x29, 0xc2, 0xa1, 0x16},
        {0xf1, 0xbe, 0x4b, 0xe1, 0xa0, 0xf9, 0xea, 0xe6, 0x50, 0x0f, 0xb2, 0xf6, 0xb6, 0x4f, 0x3d, 0xaa, 0x39, 0x90, 0xac, 0x1a},
        {0xde, 0x77, 0xf1, 0xc1, 0x34, 0x22, 0x8b, 0xe1, 0xb5, 0xb2, 0x5c, 0x94, 0x1d, 0x51, 0x02, 0xf8, 0x7f, 0x3e, 0x6d, 0x39},
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
