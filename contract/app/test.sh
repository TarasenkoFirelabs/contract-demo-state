#!/bin/bash
[ ! -f "res/app.wasm" ] && sh build.sh # build before testing
[ ! -f "../app-wallet-creation/res/app-wallet-creation.wasm" ] && sh ../app-wallet-creation/build.sh # build before testing
cargo test -- --nocapture