#! /usr/bin/env nix-shell
#! nix-shell -i bash ../shell.nix

set -e
cd "`dirname \"$0\"`/.."

bundler update
bundler lock
bundix --gemset=./gemset.nix
