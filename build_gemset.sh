#! /usr/bin/env nix-shell
#! nix-shell -i bash -p bundler bundix

set -e

bundler lock
bundix
