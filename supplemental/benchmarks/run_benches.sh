#!/bin/sh

RUSTFLAGS="-C target-feature=+aes,+avx,+avx2" cargo bench "$@"
