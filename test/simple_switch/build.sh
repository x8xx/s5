#!/bin/bash
set -ex

cd "$(dirname "$0")"

# Build WASM
cargo build --target wasm32-unknown-unknown --release

# Copy output
cp target/wasm32-unknown-unknown/release/simple_switch_parser.wasm ./parser.wasm

echo "Built: $(pwd)/parser.wasm"
