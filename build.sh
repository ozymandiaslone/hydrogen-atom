#!/bin/bash

set -e

echo "INFO: Compiling project for: 'wasm32-unknown-unknown...'"
cargo build --release --target wasm32-unknown-unknown
echo "INFO: Moving .wasm file..."
rm hydrogen-atom/static/spellstrike.wasm
mv target/wasm32-unknown-unknown/release/hydrogen-atom.wasm ../hydrogen-web/static
cd ../hydrogen-web 
cargo run --release

