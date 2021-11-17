#!/bin/bash
[ ! -d "/res" ] && ./build.sh # build before testing
cargo test --workspace -- --show-output --nocapture --quiet