#! /usr/bin/env nix-shell
#! nix-shell -i bash ../shell.nix

set -e
cd "`dirname \"$0\"`/.."

cd _source/block_lighting_demos/
cargo build --release --target=wasm32-unknown-unknown
wasm-bindgen --no-typescript --out-dir . target/wasm32-unknown-unknown/release/block_lighting_demos.wasm
npm install
npx webpack
rm block_lighting_demos.js block_lighting_demos_bg.wasm
cd ../../

rm -rf block_lighting_demos
mv _source/block_lighting_demos/dist ./block_lighting_demos
