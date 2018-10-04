#! /usr/bin/env bash

set -e
cd "`dirname \"$0\"`/.."

rm -rf .sass-cache/
rm -rf _site
rm -rf _source/block_lighting_demos/target
rm -rf _source/block_lighting_demos/node_modules
rm -f _source/block_lighting_demos/block_lighting_demos.js
rm -f _source/block_lighting_demos/block_lighting_demos_bg.wasm
rm -f _source/block_lighting_demos/dist
