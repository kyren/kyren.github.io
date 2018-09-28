#! /usr/bin/env nix-shell
#! nix-shell -i bash ../shell.nix

set -e
cd "`dirname \"$0\"`/.."

exec jekyll serve --watch
