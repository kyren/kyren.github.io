#! /usr/bin/env nix-shell
#! nix-shell -i bash -p jekyll bundler ruby bundix

set -e

bundler lock
bundler package --path vendor/cache --no-install
bundix
