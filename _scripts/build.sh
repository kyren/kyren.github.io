#! /usr/bin/env nix-shell
#! nix-shell -i bash ../shell.nix

set -e
cd "`dirname \"$0\"`/.."

pushd _source/block_lighting_demos/
cargo build --release --target=wasm32-unknown-unknown
wasm-bindgen --no-typescript --no-modules --out-dir ../../block_lighting_demos/ target/wasm32-unknown-unknown/release/block_lighting_demos.wasm
popd

jekyll build
