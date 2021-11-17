#!/bin/bash
[ ! -f "res/app_wallet_creation.wasm" ] && sh build.sh # build before testing
cargo test -- --nocapture