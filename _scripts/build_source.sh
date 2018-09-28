#! /usr/bin/env nix-shell
#! nix-shell -i bash ../shell.nix

set -e
cd "`dirname \"$0\"`/.."

cd _source/block_lighting_demos/
cargo web deploy --release --target=wasm32-unknown-unknown
cd ../../

rm -rf ./block_lighting_demos
cp -r _source/block_lighting_demos/target/deploy ./block_lighting_demos
