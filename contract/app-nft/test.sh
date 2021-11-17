#!/bin/bash
[ ! -f "res/app_nft.wasm" ] && sh build.sh # build before testing
cargo test -- --nocapture